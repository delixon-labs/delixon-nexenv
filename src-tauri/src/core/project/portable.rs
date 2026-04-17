use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::error::NexenvError;
use crate::core::manifest::{self, ProjectManifest};
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::store;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NexenvExport {
    pub version: String,
    pub exported_at: String,
    pub project: ExportedProject,
    /// Manifest completo del proyecto (si existe)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<ProjectManifest>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportedProject {
    pub name: String,
    pub description: Option<String>,
    pub runtimes: Vec<RuntimeConfig>,
    pub tags: Vec<String>,
    pub template_id: Option<String>,
    pub env_keys: Vec<String>,
}

/// Exporta un proyecto como JSON portable (.nexenv)
pub fn export_project(project_id: &str) -> Result<String, NexenvError> {
    let projects = store::get().list_projects()?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| NexenvError::ProjectNotFound(project_id.to_string()))?;

    let env_vars = store::get().load_env_vars(project_id)?;
    let env_keys: Vec<String> = env_vars.keys().cloned().collect();

    // Cargar manifest si existe
    let project_manifest = manifest::load_manifest(&project.path).unwrap_or(None);

    let export = NexenvExport {
        version: "1".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        project: ExportedProject {
            name: project.name.clone(),
            description: project.description.clone(),
            runtimes: project.runtimes.clone(),
            tags: project.tags.clone(),
            template_id: project.template_id.clone(),
            env_keys,
        },
        manifest: project_manifest,
    };

    let json = serde_json::to_string_pretty(&export)?;
    Ok(json)
}

/// Valida el target_path de un import contra path traversal, control chars
/// y paths relativos. Devuelve el path normalizado.
///
/// No canonicaliza (el path puede no existir todavia), pero rechaza:
/// - Strings vacios.
/// - Paths relativos (debe ser absoluto).
/// - Control chars (\n, \r, \0, etc.) que pueden romper parsers downstream.
/// - Cualquier componente `..` (ParentDir) — corta traversal explicito.
pub fn validate_target_path(target: &str) -> Result<PathBuf, NexenvError> {
    if target.trim().is_empty() {
        return Err(NexenvError::InvalidPath(
            "El path de destino no puede estar vacio".to_string(),
        ));
    }

    if target.chars().any(|c| c.is_control()) {
        return Err(NexenvError::InvalidPath(
            "El path contiene caracteres de control".to_string(),
        ));
    }

    let path = PathBuf::from(target);

    if !path.is_absolute() {
        return Err(NexenvError::InvalidPath(format!(
            "El path debe ser absoluto: {}",
            target
        )));
    }

    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(NexenvError::InvalidPath(format!(
            "El path contiene traversal ('..'): {}",
            target
        )));
    }

    // Si el path existe, canonicalizar para resolver symlinks y comparar
    // contra cualquier constraint futuro. Si no existe (caso normal en
    // un import a carpeta nueva), devolvemos el path tal cual.
    match Path::new(&path).canonicalize() {
        Ok(resolved) => Ok(resolved),
        Err(_) => Ok(path),
    }
}

/// Importa un proyecto desde JSON portable (.nexenv)
pub fn import_project(json: &str, target_path: &str) -> Result<Project, NexenvError> {
    let validated = validate_target_path(target_path)?;
    let target_path = validated.to_string_lossy().to_string();

    let export: NexenvExport = serde_json::from_str(json)?;

    let now = chrono::Utc::now().to_rfc3339();
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: export.project.name,
        path: target_path.to_string(),
        description: export.project.description,
        runtimes: export.project.runtimes,
        status: ProjectStatus::Active,
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: export.project.template_id,
        tags: export.project.tags,
    };

    // Verificar que no exista un proyecto en la misma ruta
    let mut projects = store::get().list_projects()?;
    if projects.iter().any(|p| p.path == target_path) {
        return Err(NexenvError::InvalidPath(format!(
            "Ya existe un proyecto registrado en esa ruta: {}",
            target_path
        )));
    }

    projects.push(project.clone());
    store::get().save_projects(&projects)?;

    // Crear env vars vacias con las keys exportadas
    if !export.project.env_keys.is_empty() {
        let vars: std::collections::HashMap<String, String> = export
            .project
            .env_keys
            .into_iter()
            .map(|k| (k, String::new()))
            .collect();
        store::get().save_env_vars(&project.id, &vars)?;
    }

    // Restaurar manifest si viene en el export
    if let Some(m) = export.manifest {
        let _ = manifest::save_manifest(&target_path, &m);
    }

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
    use crate::core::store;
    use serial_test::serial;

    fn temp_path(suffix: &str) -> String {
        std::env::temp_dir()
            .join(format!("nexenv-portable-test-{}", suffix))
            .to_string_lossy()
            .to_string()
    }

    fn make_test_project(suffix: &str) -> Project {
        Project {
            id: format!("test-portable-{}", suffix),
            name: format!("Portable Test {}", suffix),
            path: temp_path(suffix),
            description: Some("portable test".to_string()),
            runtimes: vec![RuntimeConfig {
                runtime: "node".to_string(),
                version: "20".to_string(),
            }],
            status: ProjectStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            last_opened_at: None,
            template_id: Some("tmpl-1".to_string()),
            tags: vec!["test".to_string(), "docker".to_string()],
        }
    }

    fn cleanup_project(id: &str) {
        if let Ok(projects) = store::get().list_projects() {
            let filtered: Vec<_> = projects.into_iter().filter(|p| p.id != id).collect();
            let _ = store::get().save_projects(&filtered);
        }
        let _ = store::get().delete_env_vars(id);
    }

    fn cleanup_by_path(path: &str) {
        if let Ok(projects) = store::get().list_projects() {
            // Find ids to clean env vars
            for p in projects.iter().filter(|p| p.path == path) {
                let _ = store::get().delete_env_vars(&p.id);
            }
            let filtered: Vec<_> = projects.into_iter().filter(|p| p.path != path).collect();
            let _ = store::get().save_projects(&filtered);
        }
    }

    #[test]
    #[serial(disk)]
    fn test_export_import_roundtrip() {
        crate::core::store::init_test_store();
        let proj = make_test_project("roundtrip");
        let proj_id = proj.id.clone();

        // Save project + env vars
        let mut projects = store::get().list_projects().unwrap_or_default();
        projects.retain(|p| p.id != proj.id);
        projects.push(proj.clone());
        store::get().save_projects(&projects).unwrap();

        let mut vars = std::collections::HashMap::new();
        vars.insert("SECRET".to_string(), "value123".to_string());
        store::get().save_env_vars(&proj.id, &vars).unwrap();

        // Export
        let json = export_project(&proj.id).expect("export should succeed");

        // Import at a different path
        let import_path = temp_path("import-roundtrip");
        cleanup_by_path(&import_path);
        let imported = import_project(&json, &import_path).expect("import should succeed");

        assert_eq!(imported.name, proj.name);
        assert!(imported.path.ends_with("nexenv-portable-test-import-roundtrip"));
        assert_eq!(imported.tags, proj.tags);
        assert_ne!(imported.id, proj.id, "imported project should get a new id");

        // Cleanup
        cleanup_project(&proj_id);
        cleanup_project(&imported.id);
        cleanup_by_path(&import_path);
    }

    #[test]
    #[serial(disk)]
    fn test_export_nonexistent_project() {
        crate::core::store::init_test_store();
        let result = export_project("nonexistent-project-xyz-portable-999");
        assert!(result.is_err(), "exporting nonexistent project should error");
    }

    #[test]
    fn test_import_invalid_json() {
        crate::core::store::init_test_store();
        let path = temp_path("invalid");
        let result = import_project("this is not json", &path);
        assert!(result.is_err(), "importing invalid JSON should error");
    }

    #[test]
    fn test_validate_target_path_rejects_empty() {
        assert!(matches!(
            validate_target_path(""),
            Err(NexenvError::InvalidPath(_))
        ));
        assert!(matches!(
            validate_target_path("   "),
            Err(NexenvError::InvalidPath(_))
        ));
    }

    #[test]
    fn test_validate_target_path_rejects_control_chars() {
        assert!(matches!(
            validate_target_path("/tmp/evil\n/etc/passwd"),
            Err(NexenvError::InvalidPath(_))
        ));
        assert!(matches!(
            validate_target_path("/tmp/evil\0null"),
            Err(NexenvError::InvalidPath(_))
        ));
    }

    #[test]
    fn test_validate_target_path_rejects_relative() {
        assert!(matches!(
            validate_target_path("relative/path"),
            Err(NexenvError::InvalidPath(_))
        ));
        assert!(matches!(
            validate_target_path("./foo"),
            Err(NexenvError::InvalidPath(_))
        ));
    }

    #[test]
    fn test_validate_target_path_rejects_traversal() {
        #[cfg(unix)]
        let evil = "/tmp/../etc/passwd";
        #[cfg(windows)]
        let evil = r"C:\tmp\..\Windows\System32";
        assert!(matches!(
            validate_target_path(evil),
            Err(NexenvError::InvalidPath(_))
        ));
    }

    #[test]
    fn test_validate_target_path_accepts_valid() {
        let valid = temp_path("valid-path-check");
        assert!(validate_target_path(&valid).is_ok());
    }

    #[test]
    fn test_import_rejects_invalid_path() {
        crate::core::store::init_test_store();
        let bogus_json = serde_json::json!({
            "version": "1",
            "exportedAt": "2026-01-01T00:00:00Z",
            "project": {
                "name": "x", "runtimes": [], "tags": [], "envKeys": []
            }
        })
        .to_string();
        // relative path
        assert!(import_project(&bogus_json, "relative/path").is_err());
        // control char
        assert!(import_project(&bogus_json, "/tmp/evil\nname").is_err());
    }

    #[test]
    #[serial(disk)]
    fn test_import_duplicate_path() {
        crate::core::store::init_test_store();
        let proj = make_test_project("dup-path");
        let proj_id = proj.id.clone();
        let dup_path = proj.path.clone();

        // Save the project
        let mut projects = store::get().list_projects().unwrap_or_default();
        projects.retain(|p| p.id != proj.id && p.path != proj.path);
        projects.push(proj.clone());
        store::get().save_projects(&projects).unwrap();

        // Export then try to import at the SAME path
        let json = export_project(&proj.id).unwrap();
        let result = import_project(&json, &dup_path);
        assert!(result.is_err(), "importing to duplicate path should error");

        cleanup_project(&proj_id);
    }

    #[test]
    #[serial(disk)]
    fn test_export_env_keys_not_values() {
        crate::core::store::init_test_store();
        let proj = make_test_project("env-keys");
        let proj_id = proj.id.clone();

        let mut projects = store::get().list_projects().unwrap_or_default();
        projects.retain(|p| p.id != proj.id);
        projects.push(proj.clone());
        store::get().save_projects(&projects).unwrap();

        let mut vars = std::collections::HashMap::new();
        vars.insert("MY_SECRET".to_string(), "super_secret_value".to_string());
        store::get().save_env_vars(&proj.id, &vars).unwrap();

        let json = export_project(&proj.id).unwrap();

        // Keys should be present, values should NOT
        assert!(json.contains("MY_SECRET"), "export should contain env key");
        assert!(
            !json.contains("super_secret_value"),
            "export should NOT contain env value"
        );

        cleanup_project(&proj_id);
    }
}
