use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::core::error::DelixonError;
use crate::core::models::project::Project;
use crate::core::utils::fs::ensure_dir;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectManifest {
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

pub fn load_manifest(project_path: &str) -> Result<Option<ProjectManifest>, DelixonError> {
    let path = manifest_file(project_path);
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&path)?;
    let manifest: ProjectManifest = serde_yml::from_str(&data)
        .map_err(|e| DelixonError::InvalidConfig(format!("Error parseando manifest: {}", e)))?;
    Ok(Some(manifest))
}

pub fn save_manifest(
    project_path: &str,
    manifest: &ProjectManifest,
) -> Result<(), DelixonError> {
    let dir = manifest_dir(project_path);
    ensure_dir(&dir)?;
    let path = manifest_file(project_path);
    let data = serde_yml::to_string(manifest)
        .map_err(|e| DelixonError::InvalidConfig(format!("Error serializando manifest: {}", e)))?;
    std::fs::write(&path, data)?;
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

    ProjectManifest {
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
        assert!(manifest.commands.contains_key("dev"));
        assert!(manifest.ports.contains(&3000));
    }

    #[test]
    fn test_save_load_manifest_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();

        let manifest = ProjectManifest {
            name: "test-project".to_string(),
            project_type: "api".to_string(),
            profile: "standard".to_string(),
            runtime: "python".to_string(),
            technologies: vec!["python".to_string(), "fastapi".to_string()],
            ports: vec![8000],
            ..Default::default()
        };

        save_manifest(project_path, &manifest).expect("save should work");
        let loaded = load_manifest(project_path).expect("load should work");
        assert!(loaded.is_some());

        let loaded = loaded.unwrap();
        assert_eq!(loaded.name, "test-project");
        assert_eq!(loaded.runtime, "python");
        assert_eq!(loaded.ports, vec![8000]);
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
}
