use crate::core::error::DelixonError;
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::storage;
use crate::core::utils::fs::ensure_dir;
use std::path::Path;

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

fn all_templates() -> Vec<TemplateInfo> {
    vec![
        TemplateInfo {
            id: "node-express",
            name: "Node.js + Express",
            runtimes: &["node"],
            tags: &["backend", "api", "rest"],
            files: node_express_files,
        },
        TemplateInfo {
            id: "react-vite",
            name: "React + Vite",
            runtimes: &["node"],
            tags: &["frontend", "spa", "react"],
            files: react_vite_files,
        },
        TemplateInfo {
            id: "python-fastapi",
            name: "Python + FastAPI",
            runtimes: &["python"],
            tags: &["backend", "api", "python"],
            files: python_fastapi_files,
        },
        TemplateInfo {
            id: "python-django",
            name: "Python + Django",
            runtimes: &["python"],
            tags: &["backend", "fullstack", "python"],
            files: python_django_files,
        },
        TemplateInfo {
            id: "fullstack-react-python",
            name: "React + FastAPI",
            runtimes: &["node", "python"],
            tags: &["fullstack", "monorepo"],
            files: fullstack_react_python_files,
        },
        TemplateInfo {
            id: "rust-cli",
            name: "Rust CLI",
            runtimes: &["rust"],
            tags: &["cli", "rust", "tool"],
            files: rust_cli_files,
        },
        TemplateInfo {
            id: "docker-compose",
            name: "Docker Compose Stack",
            runtimes: &[],
            tags: &["docker", "devops", "infra"],
            files: docker_compose_files,
        },
    ]
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

    // Generar archivos del template
    let files = (template.files)();
    for file in &files {
        let content = file.content
            .replace("{{project_name}}", project_name)
            .replace("{{project-name}}", &project_name.replace('_', "-"));

        let file_path = base.join(file.path);
        if let Some(parent) = file_path.parent() {
            ensure_dir(parent)?;
        }
        std::fs::write(&file_path, content)?;
    }

    // Registrar proyecto en Delixon
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

    // Generar manifest automaticamente
    let manifest = crate::core::manifest::generate_manifest_from_project(&project);
    let _ = crate::core::manifest::save_manifest(project_path, &manifest);

    Ok(project)
}

// --- Template file generators ---

fn node_express_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "package.json", content: include_str!("files/node_express/package.json") },
        TemplateFile { path: ".gitignore", content: include_str!("files/node_express/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/node_express/README.md") },
        TemplateFile { path: "src/index.js", content: include_str!("files/node_express/src_index.js") },
    ]
}

fn react_vite_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "package.json", content: include_str!("files/react_vite/package.json") },
        TemplateFile { path: ".gitignore", content: include_str!("files/react_vite/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/react_vite/README.md") },
        TemplateFile { path: "index.html", content: include_str!("files/react_vite/index.html") },
        TemplateFile { path: "src/main.tsx", content: include_str!("files/react_vite/src_main.tsx") },
        TemplateFile { path: "src/App.tsx", content: include_str!("files/react_vite/src_App.tsx") },
        TemplateFile { path: "tsconfig.json", content: include_str!("files/react_vite/tsconfig.json") },
        TemplateFile { path: "vite.config.ts", content: include_str!("files/react_vite/vite.config.ts") },
    ]
}

fn python_fastapi_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "requirements.txt", content: include_str!("files/python_fastapi/requirements.txt") },
        TemplateFile { path: ".gitignore", content: include_str!("files/python_fastapi/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/python_fastapi/README.md") },
        TemplateFile { path: "app/main.py", content: include_str!("files/python_fastapi/app_main.py") },
    ]
}

fn python_django_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "requirements.txt", content: include_str!("files/python_django/requirements.txt") },
        TemplateFile { path: ".gitignore", content: include_str!("files/python_django/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/python_django/README.md") },
        TemplateFile { path: "manage.py", content: include_str!("files/python_django/manage.py") },
    ]
}

fn fullstack_react_python_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "README.md", content: include_str!("files/fullstack_react_python/README.md") },
        TemplateFile { path: ".gitignore", content: include_str!("files/fullstack_react_python/.gitignore") },
        TemplateFile { path: "frontend/package.json", content: include_str!("files/fullstack_react_python/frontend_package.json") },
        TemplateFile { path: "frontend/src/App.tsx", content: include_str!("files/fullstack_react_python/frontend_src_App.tsx") },
        TemplateFile { path: "backend/requirements.txt", content: include_str!("files/fullstack_react_python/backend_requirements.txt") },
        TemplateFile { path: "backend/app/main.py", content: include_str!("files/fullstack_react_python/backend_app_main.py") },
    ]
}

fn rust_cli_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "Cargo.toml", content: include_str!("files/rust_cli/Cargo.toml") },
        TemplateFile { path: ".gitignore", content: include_str!("files/rust_cli/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/rust_cli/README.md") },
        TemplateFile { path: "src/main.rs", content: include_str!("files/rust_cli/src_main.rs") },
    ]
}

fn docker_compose_files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "docker-compose.yml", content: include_str!("files/docker_compose/docker-compose.yml") },
        TemplateFile { path: ".gitignore", content: include_str!("files/docker_compose/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/docker_compose/README.md") },
        TemplateFile { path: ".env.example", content: include_str!("files/docker_compose/.env.example") },
    ]
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

        // Verify placeholder replacement
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
