use serde::{Deserialize, Serialize};

use crate::core::error::DelixonError;
use crate::core::manifest::{self, ProjectManifest};
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::storage;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelixonExport {
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

/// Exporta un proyecto como JSON portable (.delixon)
pub fn export_project(project_id: &str) -> Result<String, DelixonError> {
    let projects = storage::load_projects()?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| DelixonError::ProjectNotFound(project_id.to_string()))?;

    let env_vars = storage::load_env_vars(project_id)?;
    let env_keys: Vec<String> = env_vars.keys().cloned().collect();

    // Cargar manifest si existe
    let project_manifest = manifest::load_manifest(&project.path).unwrap_or(None);

    let export = DelixonExport {
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

/// Importa un proyecto desde JSON portable (.delixon)
pub fn import_project(json: &str, target_path: &str) -> Result<Project, DelixonError> {
    let export: DelixonExport = serde_json::from_str(json)?;

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
    let mut projects = storage::load_projects()?;
    if projects.iter().any(|p| p.path == target_path) {
        return Err(DelixonError::InvalidPath(format!(
            "Ya existe un proyecto registrado en esa ruta: {}",
            target_path
        )));
    }

    projects.push(project.clone());
    storage::save_projects(&projects)?;

    // Crear env vars vacias con las keys exportadas
    if !export.project.env_keys.is_empty() {
        let vars: std::collections::HashMap<String, String> = export
            .project
            .env_keys
            .into_iter()
            .map(|k| (k, String::new()))
            .collect();
        storage::save_env_vars(&project.id, &vars)?;
    }

    // Restaurar manifest si viene en el export
    if let Some(m) = export.manifest {
        let _ = manifest::save_manifest(target_path, &m);
    }

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
    use crate::core::storage;
    use serial_test::serial;

    fn make_test_project(suffix: &str) -> Project {
        Project {
            id: format!("test-portable-{}", suffix),
            name: format!("Portable Test {}", suffix),
            path: format!("/tmp/delixon-portable-test-{}", suffix),
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
        if let Ok(projects) = storage::load_projects() {
            let filtered: Vec<_> = projects.into_iter().filter(|p| p.id != id).collect();
            let _ = storage::save_projects(&filtered);
        }
        let _ = storage::delete_env_vars(id);
    }

    fn cleanup_by_path(path: &str) {
        if let Ok(projects) = storage::load_projects() {
            // Find ids to clean env vars
            for p in projects.iter().filter(|p| p.path == path) {
                let _ = storage::delete_env_vars(&p.id);
            }
            let filtered: Vec<_> = projects.into_iter().filter(|p| p.path != path).collect();
            let _ = storage::save_projects(&filtered);
        }
    }

    #[test]
    #[serial(disk)]
    fn test_export_import_roundtrip() {
        let proj = make_test_project("roundtrip");
        let proj_id = proj.id.clone();

        // Save project + env vars
        let mut projects = storage::load_projects().unwrap_or_default();
        projects.retain(|p| p.id != proj.id);
        projects.push(proj.clone());
        storage::save_projects(&projects).unwrap();

        let mut vars = std::collections::HashMap::new();
        vars.insert("SECRET".to_string(), "value123".to_string());
        storage::save_env_vars(&proj.id, &vars).unwrap();

        // Export
        let json = export_project(&proj.id).expect("export should succeed");

        // Import at a different path
        let import_path = "/tmp/delixon-portable-import-roundtrip";
        cleanup_by_path(import_path);
        let imported = import_project(&json, import_path).expect("import should succeed");

        assert_eq!(imported.name, proj.name);
        assert_eq!(imported.path, import_path);
        assert_eq!(imported.tags, proj.tags);
        assert_ne!(imported.id, proj.id, "imported project should get a new id");

        // Cleanup
        cleanup_project(&proj_id);
        cleanup_project(&imported.id);
        cleanup_by_path(import_path);
    }

    #[test]
    #[serial(disk)]
    fn test_export_nonexistent_project() {
        let result = export_project("nonexistent-project-xyz-portable-999");
        assert!(result.is_err(), "exporting nonexistent project should error");
    }

    #[test]
    fn test_import_invalid_json() {
        let result = import_project("this is not json", "/tmp/delixon-invalid");
        assert!(result.is_err(), "importing invalid JSON should error");
    }

    #[test]
    #[serial(disk)]
    fn test_import_duplicate_path() {
        let proj = make_test_project("dup-path");
        let proj_id = proj.id.clone();
        let dup_path = proj.path.clone();

        // Save the project
        let mut projects = storage::load_projects().unwrap_or_default();
        projects.retain(|p| p.id != proj.id && p.path != proj.path);
        projects.push(proj.clone());
        storage::save_projects(&projects).unwrap();

        // Export then try to import at the SAME path
        let json = export_project(&proj.id).unwrap();
        let result = import_project(&json, &dup_path);
        assert!(result.is_err(), "importing to duplicate path should error");

        cleanup_project(&proj_id);
    }

    #[test]
    #[serial(disk)]
    fn test_export_env_keys_not_values() {
        let proj = make_test_project("env-keys");
        let proj_id = proj.id.clone();

        let mut projects = storage::load_projects().unwrap_or_default();
        projects.retain(|p| p.id != proj.id);
        projects.push(proj.clone());
        storage::save_projects(&projects).unwrap();

        let mut vars = std::collections::HashMap::new();
        vars.insert("MY_SECRET".to_string(), "super_secret_value".to_string());
        storage::save_env_vars(&proj.id, &vars).unwrap();

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
