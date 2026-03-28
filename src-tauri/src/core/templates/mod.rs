mod docker_compose;
mod fullstack_react_python;
mod node_express;
mod python_django;
mod python_fastapi;
mod react_vite;
mod registry;
mod rust_cli;

use crate::core::error::DelixonError;
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::storage;
use crate::core::utils::fs::ensure_dir;
use std::path::Path;

pub use registry::all_templates;

pub struct TemplateFile {
    pub path: &'static str,
    pub content: &'static str,
}

pub struct TemplateInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub runtimes: &'static [&'static str],
    pub tags: &'static [&'static str],
    pub files: fn() -> Vec<TemplateFile>,
}

/// Crea un proyecto a partir de una plantilla
pub fn create_from_template(
    template_id: &str,
    project_path: &str,
    project_name: &str,
) -> Result<Project, DelixonError> {
    let template = all_templates()
        .into_iter()
        .find(|t| t.id == template_id)
        .ok_or_else(|| DelixonError::TemplateNotFound(template_id.to_string()))?;

    let base = Path::new(project_path);
    ensure_dir(base)?;

    let files = (template.files)();
    for file in &files {
        let content = file
            .content
            .replace("{{project_name}}", project_name)
            .replace("{{project-name}}", &project_name.replace('_', "-"));

        let file_path = base.join(file.path);
        if let Some(parent) = file_path.parent() {
            ensure_dir(parent)?;
        }
        std::fs::write(&file_path, content)?;
    }

    let runtimes: Vec<RuntimeConfig> = template
        .runtimes
        .iter()
        .map(|r| RuntimeConfig {
            runtime: r.to_string(),
            version: String::new(),
        })
        .collect();

    let tags: Vec<String> = template.tags.iter().map(|t| t.to_string()).collect();

    let now = chrono::Utc::now().to_rfc3339();
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: project_name.to_string(),
        path: project_path.to_string(),
        description: Some(format!("Creado desde plantilla: {}", template.name)),
        runtimes,
        status: ProjectStatus::Active,
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: Some(template_id.to_string()),
        tags,
    };

    let mut projects = storage::load_projects()?;
    projects.push(project.clone());
    storage::save_projects(&projects)?;

    let manifest = crate::core::manifest::generate_manifest_from_project(&project);
    let _ = crate::core::manifest::save_manifest(project_path, &manifest);

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_templates_exist() {
        let templates = all_templates();
        assert_eq!(templates.len(), 7);
    }

    #[test]
    fn test_node_express_template_generates_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-project");
        let result = create_from_template("node-express", path.to_str().unwrap(), "test-project");
        assert!(result.is_ok());
        assert!(path.join("package.json").exists());
        assert!(path.join(".gitignore").exists());
        assert!(path.join("README.md").exists());
        assert!(path.join("src/index.js").exists());

        let pkg = std::fs::read_to_string(path.join("package.json")).unwrap();
        assert!(pkg.contains("test-project"));
    }

    #[test]
    fn test_react_vite_template_generates_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("my-react-app");
        let result = create_from_template("react-vite", path.to_str().unwrap(), "my-react-app");
        assert!(result.is_ok(), "Failed: {:?}", result.err());
        assert!(path.join("package.json").exists());
        assert!(path.join("src/main.tsx").exists());
        assert!(path.join("src/App.tsx").exists());
        assert!(path.join("vite.config.ts").exists());
    }

    #[test]
    fn test_python_fastapi_template() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("my-api");
        let result = create_from_template("python-fastapi", path.to_str().unwrap(), "my-api");
        assert!(result.is_ok());
        assert!(path.join("requirements.txt").exists());
        assert!(path.join("app/main.py").exists());
    }

    #[test]
    fn test_invalid_template_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let result = create_from_template("nonexistent", dir.path().to_str().unwrap(), "test");
        assert!(result.is_err());
    }
}
