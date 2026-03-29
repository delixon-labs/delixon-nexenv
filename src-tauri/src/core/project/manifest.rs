use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::core::error::DelixonError;
use crate::core::models::project::Project;
use crate::core::utils::fs::ensure_dir;

/// Version actual del schema del manifest.
/// Se incrementa cuando cambia la estructura de forma incompatible.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectManifest {
    /// Version del schema — permite migraciones futuras
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub name: String,
    #[serde(default)]
    pub project_type: String,
    #[serde(default)]
    pub profile: String,
    #[serde(default)]
    pub runtime: String,
    #[serde(default)]
    pub technologies: Vec<String>,
    #[serde(default)]
    pub services: Vec<ManifestService>,
    #[serde(default)]
    pub env_vars: ManifestEnvVars,
    #[serde(default)]
    pub commands: HashMap<String, String>,
    #[serde(default)]
    pub ports: Vec<u16>,
    #[serde(default)]
    pub recipes_applied: Vec<String>,
    #[serde(default)]
    pub health_checks: Vec<ManifestHealthCheck>,
    #[serde(default)]
    pub metadata: ManifestMetadata,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<String>,
}

fn default_schema_version() -> u32 {
    CURRENT_SCHEMA_VERSION
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManifestMetadata {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ManifestService {
    pub name: String,
    #[serde(default)]
    pub docker: bool,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub health_check: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ManifestEnvVars {
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub optional: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ManifestHealthCheck {
    pub name: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub endpoint: String,
}

fn manifest_dir(project_path: &str) -> std::path::PathBuf {
    Path::new(project_path).join(".delixon")
}

fn manifest_file(project_path: &str) -> std::path::PathBuf {
    manifest_dir(project_path).join("manifest.yaml")
}

/// Valida un manifest antes de guardarlo.
/// Rechaza manifests con datos invalidos o incompletos.
pub fn validate_manifest(manifest: &ProjectManifest) -> Result<(), DelixonError> {
    if manifest.name.trim().is_empty() {
        return Err(DelixonError::InvalidManifest(
            "El campo 'name' es obligatorio y no puede estar vacio".to_string(),
        ));
    }

    if manifest.schema_version == 0 {
        return Err(DelixonError::InvalidManifest(
            "schema_version debe ser >= 1".to_string(),
        ));
    }

    for &port in &manifest.ports {
        if port == 0 {
            return Err(DelixonError::InvalidManifest(
                "Puerto invalido: 0 (rango valido: 1-65535)".to_string(),
            ));
        }
    }

    // Detectar puertos duplicados
    let mut seen_ports = std::collections::HashSet::new();
    for &port in &manifest.ports {
        if !seen_ports.insert(port) {
            return Err(DelixonError::InvalidManifest(
                format!("Puerto duplicado: {}", port),
            ));
        }
    }

    // Validar que env vars no contengan valores (solo nombres)
    for key in manifest.env_vars.required.iter().chain(manifest.env_vars.optional.iter()) {
        if key.contains('=') {
            return Err(DelixonError::InvalidManifest(
                format!("env_vars debe contener solo nombres de variables, no valores. Encontrado: '{}'", key),
            ));
        }
    }

    Ok(())
}

/// Normaliza un manifest: corrige valores por defecto y limpia datos.
fn normalize_manifest(manifest: &mut ProjectManifest) {
    // Si schema_version viene como 0 (manifest antiguo pre-versionado), asumir v1
    if manifest.schema_version == 0 {
        manifest.schema_version = CURRENT_SCHEMA_VERSION;
    }

    // Deduplicar puertos
    manifest.ports.sort();
    manifest.ports.dedup();

    // Deduplicar technologies
    manifest.technologies.sort();
    manifest.technologies.dedup();

    // Deduplicar recipes
    manifest.recipes_applied.sort();
    manifest.recipes_applied.dedup();

    // Limpiar espacios en nombre
    manifest.name = manifest.name.trim().to_string();
}

pub fn load_manifest(project_path: &str) -> Result<Option<ProjectManifest>, DelixonError> {
    let path = manifest_file(project_path);
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&path)?;
    let mut manifest: ProjectManifest = serde_yml::from_str(&data)
        .map_err(|e| DelixonError::InvalidConfig(format!("Error parseando manifest: {}", e)))?;

    // Normalizar manifests antiguos al cargar
    normalize_manifest(&mut manifest);

    Ok(Some(manifest))
}

/// Guarda un manifest validandolo y normalizandolo antes de escribir.
/// Rechaza manifests invalidos — nunca se guarda basura.
pub fn save_manifest(
    project_path: &str,
    manifest: &ProjectManifest,
) -> Result<(), DelixonError> {
    let mut normalized = manifest.clone();
    normalize_manifest(&mut normalized);
    validate_manifest(&normalized)?;

    let dir = manifest_dir(project_path);
    ensure_dir(&dir)?;
    let path = manifest_file(project_path);
    let data = serde_yml::to_string(&normalized)
        .map_err(|e| DelixonError::InvalidConfig(format!("Error serializando manifest: {}", e)))?;
    std::fs::write(&path, data)?;

    // Asegurar que .delixon/ este en .gitignore del proyecto
    let project = std::path::Path::new(project_path);
    let _ = crate::core::utils::fs::ensure_gitignore_entries(project, &[".delixon/"]);

    Ok(())
}

/// Genera un manifest a partir de un proyecto registrado y su stack detectado
pub fn generate_manifest_from_project(project: &Project) -> ProjectManifest {
    let runtime = project
        .runtimes
        .first()
        .map(|r| r.runtime.clone())
        .unwrap_or_default();

    let technologies: Vec<String> = project
        .runtimes
        .iter()
        .map(|r| r.runtime.clone())
        .collect();

    // Detectar commands segun runtime
    let mut commands = HashMap::new();
    match runtime.as_str() {
        "node" => {
            commands.insert("dev".to_string(), "npm run dev".to_string());
            commands.insert("build".to_string(), "npm run build".to_string());
            commands.insert("test".to_string(), "npm run test".to_string());
            commands.insert("lint".to_string(), "npm run lint".to_string());
        }
        "python" => {
            commands.insert("dev".to_string(), "uvicorn app.main:app --reload".to_string());
            commands.insert("test".to_string(), "pytest".to_string());
        }
        "rust" => {
            commands.insert("dev".to_string(), "cargo run".to_string());
            commands.insert("build".to_string(), "cargo build --release".to_string());
            commands.insert("test".to_string(), "cargo test".to_string());
            commands.insert("lint".to_string(), "cargo clippy".to_string());
        }
        "go" => {
            commands.insert("dev".to_string(), "go run .".to_string());
            commands.insert("build".to_string(), "go build .".to_string());
            commands.insert("test".to_string(), "go test ./...".to_string());
        }
        _ => {}
    }

    // Detectar puertos por tags
    let mut ports = Vec::new();
    for tag in &project.tags {
        match tag.as_str() {
            "express" | "fastify" | "nestjs" => ports.push(3000),
            "fastapi" | "flask" | "django" => ports.push(8000),
            "react" | "vue" | "svelte" | "vite" => ports.push(5173),
            "nextjs" | "nuxt" => ports.push(3000),
            _ => {}
        }
    }
    ports.sort();
    ports.dedup();

    let now = chrono::Utc::now().to_rfc3339();

    ProjectManifest {
        schema_version: CURRENT_SCHEMA_VERSION,
        name: project.name.clone(),
        project_type: infer_project_type(&project.tags),
        profile: "standard".to_string(),
        runtime,
        technologies,
        services: Vec::new(),
        env_vars: ManifestEnvVars::default(),
        commands,
        ports,
        recipes_applied: Vec::new(),
        health_checks: Vec::new(),
        metadata: ManifestMetadata {
            description: project.description.clone().unwrap_or_default(),
            created_at: now,
            author: String::new(),
        },
        editor: None,
    }
}

fn infer_project_type(tags: &[String]) -> String {
    for tag in tags {
        match tag.as_str() {
            "nextjs" | "nuxt" => return "fullstack".to_string(),
            "express" | "fastify" | "fastapi" | "django" | "flask" | "nestjs" | "gin" => {
                return "api".to_string()
            }
            "react" | "vue" | "svelte" => return "frontend".to_string(),
            "cli" => return "cli".to_string(),
            "tauri" | "electron" => return "desktop".to_string(),
            "docker" => return "infra".to_string(),
            _ => {}
        }
    }
    "project".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};

    fn make_project() -> Project {
        Project {
            id: "test-manifest".to_string(),
            name: "my-api".to_string(),
            path: "/tmp/delixon-manifest-test".to_string(),
            description: Some("Test API".to_string()),
            runtimes: vec![RuntimeConfig {
                runtime: "node".to_string(),
                version: "20".to_string(),
            }],
            status: ProjectStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            last_opened_at: None,
            template_id: None,
            tags: vec!["express".to_string(), "typescript".to_string()],
        }
    }

    #[test]
    fn test_generate_manifest_from_project() {
        let project = make_project();
        let manifest = generate_manifest_from_project(&project);

        assert_eq!(manifest.name, "my-api");
        assert_eq!(manifest.runtime, "node");
        assert_eq!(manifest.project_type, "api");
        assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(manifest.metadata.description, "Test API");
        assert!(!manifest.metadata.created_at.is_empty());
        assert!(manifest.commands.contains_key("dev"));
        assert!(manifest.ports.contains(&3000));
    }

    #[test]
    fn test_save_load_manifest_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();

        let manifest = ProjectManifest {
            schema_version: CURRENT_SCHEMA_VERSION,
            name: "test-project".to_string(),
            project_type: "api".to_string(),
            profile: "standard".to_string(),
            runtime: "python".to_string(),
            technologies: vec!["python".to_string(), "fastapi".to_string()],
            ports: vec![8000],
            metadata: ManifestMetadata {
                description: "Mi API de prueba".to_string(),
                created_at: "2026-01-01T00:00:00Z".to_string(),
                author: "dev".to_string(),
            },
            editor: Some("code".to_string()),
            ..Default::default()
        };

        save_manifest(project_path, &manifest).expect("save should work");
        let loaded = load_manifest(project_path).expect("load should work");
        assert!(loaded.is_some());

        let loaded = loaded.unwrap();
        assert_eq!(loaded.name, "test-project");
        assert_eq!(loaded.runtime, "python");
        assert_eq!(loaded.ports, vec![8000]);
        assert_eq!(loaded.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(loaded.metadata.description, "Mi API de prueba");
        assert_eq!(loaded.editor, Some("code".to_string()));
    }

    #[test]
    fn test_load_manifest_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let result = load_manifest(dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_infer_project_type() {
        assert_eq!(infer_project_type(&["express".to_string()]), "api");
        assert_eq!(infer_project_type(&["react".to_string()]), "frontend");
        assert_eq!(infer_project_type(&["nextjs".to_string()]), "fullstack");
        assert_eq!(infer_project_type(&["cli".to_string()]), "cli");
        assert_eq!(infer_project_type(&["tauri".to_string()]), "desktop");
        assert_eq!(infer_project_type(&[]), "project");
    }

    #[test]
    fn test_validate_rejects_empty_name() {
        let manifest = ProjectManifest {
            schema_version: 1,
            name: "".to_string(),
            ..Default::default()
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_rejects_zero_schema_version() {
        let manifest = ProjectManifest {
            schema_version: 0,
            name: "test".to_string(),
            ..Default::default()
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_rejects_zero_port() {
        let manifest = ProjectManifest {
            schema_version: 1,
            name: "test".to_string(),
            ports: vec![3000, 0],
            ..Default::default()
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_rejects_duplicate_ports() {
        let manifest = ProjectManifest {
            schema_version: 1,
            name: "test".to_string(),
            ports: vec![3000, 3000],
            ..Default::default()
        };
        // normalize_manifest dedup ports before validation, so save_manifest would pass.
        // But raw validate should catch it.
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_rejects_env_var_with_value() {
        let manifest = ProjectManifest {
            schema_version: 1,
            name: "test".to_string(),
            env_vars: ManifestEnvVars {
                required: vec!["DB_URL=postgres://localhost".to_string()],
                optional: vec![],
            },
            ..Default::default()
        };
        assert!(validate_manifest(&manifest).is_err());
    }

    #[test]
    fn test_validate_accepts_valid_manifest() {
        let manifest = ProjectManifest {
            schema_version: 1,
            name: "my-project".to_string(),
            project_type: "api".to_string(),
            ports: vec![3000, 5432],
            env_vars: ManifestEnvVars {
                required: vec!["DATABASE_URL".to_string()],
                optional: vec!["REDIS_URL".to_string()],
            },
            ..Default::default()
        };
        assert!(validate_manifest(&manifest).is_ok());
    }

    #[test]
    fn test_normalize_deduplicates() {
        let mut manifest = ProjectManifest {
            schema_version: 1,
            name: "  test  ".to_string(),
            ports: vec![3000, 5432, 3000],
            technologies: vec!["react".to_string(), "nodejs".to_string(), "react".to_string()],
            recipes_applied: vec!["docker".to_string(), "docker".to_string()],
            ..Default::default()
        };
        normalize_manifest(&mut manifest);
        assert_eq!(manifest.name, "test");
        assert_eq!(manifest.ports, vec![3000, 5432]);
        assert_eq!(manifest.technologies, vec!["nodejs".to_string(), "react".to_string()]);
        assert_eq!(manifest.recipes_applied, vec!["docker".to_string()]);
    }

    #[test]
    fn test_normalize_upgrades_schema_version() {
        let mut manifest = ProjectManifest {
            schema_version: 0,
            name: "old-project".to_string(),
            ..Default::default()
        };
        normalize_manifest(&mut manifest);
        assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn test_save_rejects_invalid_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();

        let manifest = ProjectManifest {
            name: "".to_string(),
            ..Default::default()
        };
        let result = save_manifest(project_path, &manifest);
        assert!(result.is_err());
    }
}
