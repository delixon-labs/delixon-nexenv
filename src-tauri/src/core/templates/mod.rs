mod docker_compose;
mod fullstack_react_python;
mod node_express;
mod python_django;
mod python_fastapi;
mod react_vite;
mod registry;
mod rust_cli;

use crate::core::error::NexenvError;
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::store;
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
) -> Result<Project, NexenvError> {
    let template = all_templates()
        .into_iter()
        .find(|t| t.id == template_id)
        .ok_or_else(|| NexenvError::TemplateNotFound(template_id.to_string()))?;

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

    let mut projects = store::get().list_projects()?;
    projects.push(project.clone());
    store::get().save_projects(&projects)?;

    let manifest = crate::core::manifest::generate_manifest_from_project(&project);
    let _ = crate::core::manifest::save_manifest(project_path, &manifest);

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::store;
    use serial_test::serial;
    use std::path::PathBuf;

    /// Elimina un proyecto del projects.json por su path
    fn cleanup_by_path(path: &str) {
        if let Ok(projects) = store::get().list_projects() {
            for p in projects.iter().filter(|p| p.path == path) {
                let _ = store::get().delete_env_vars(&p.id);
            }
            let filtered: Vec<_> = projects.into_iter().filter(|p| p.path != path).collect();
            let _ = store::get().save_projects(&filtered);
        }
    }

    /// Helper de smoke test por template. Comprueba:
    /// 1. `create_from_template` devuelve Ok
    /// 2. Todos los archivos esperados existen
    /// 3. El manifest Nexenv se genero (es prerequisito para que open/recipes funcionen)
    /// 4. La sustitucion de {{project_name}} ocurrio (si el template lo usa)
    fn run_template_smoke(template_id: &str, name: &str, expected: &[&str]) -> PathBuf {
        crate::core::store::init_test_store();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(name);
        let path_str = path.to_str().unwrap().to_string();

        let result = create_from_template(template_id, &path_str, name);
        assert!(
            result.is_ok(),
            "create_from_template({}) fallo: {:?}",
            template_id,
            result.err()
        );

        for f in expected {
            assert!(
                path.join(f).exists(),
                "falta archivo '{}' en template '{}'",
                f,
                template_id
            );
        }

        let manifest = crate::core::manifest::load_manifest(&path_str)
            .expect("load_manifest no debe fallar")
            .unwrap_or_else(|| panic!("manifest Nexenv no generado para '{}'", template_id));
        assert_eq!(manifest.name, name, "manifest.name debe coincidir con el nombre");

        // Mantener tempdir vivo retornando su path (el dir se borra al salir del scope).
        let kept = dir.keep();
        let owned_path = kept.join(name);
        cleanup_by_path(&path_str);
        owned_path
    }

    #[test]
    fn test_all_templates_exist() {
        let templates = all_templates();
        assert_eq!(templates.len(), 7);
    }

    // --- 7 smoke tests por template ---

    #[test]
    #[serial(disk)]
    fn test_node_express_template_generates_files() {
        let path = run_template_smoke(
            "node-express",
            "test-project",
            &["package.json", ".gitignore", "README.md", "src/index.js"],
        );
        let pkg = std::fs::read_to_string(path.join("package.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&pkg)
            .expect("package.json debe ser JSON valido");
        assert_eq!(parsed.get("name").and_then(|v| v.as_str()), Some("test-project"));
        assert!(parsed.get("scripts").is_some(), "package.json debe tener 'scripts'");
    }

    #[test]
    #[serial(disk)]
    fn test_react_vite_template_generates_files() {
        let path = run_template_smoke(
            "react-vite",
            "my-react-app",
            &["package.json", "src/main.tsx", "src/App.tsx", "vite.config.ts"],
        );
        let pkg = std::fs::read_to_string(path.join("package.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&pkg)
            .expect("package.json debe ser JSON valido");
        let deps = parsed.get("dependencies").or_else(|| parsed.get("devDependencies"))
            .expect("package.json debe declarar dependencies o devDependencies");
        assert!(deps.is_object());
    }

    #[test]
    #[serial(disk)]
    fn test_python_fastapi_template() {
        let path = run_template_smoke(
            "python-fastapi",
            "my-api",
            &["requirements.txt", "app/main.py"],
        );
        let reqs = std::fs::read_to_string(path.join("requirements.txt")).unwrap();
        assert!(!reqs.trim().is_empty(), "requirements.txt no puede estar vacio");
        assert!(reqs.to_lowercase().contains("fastapi"),
            "requirements.txt de FastAPI debe declarar fastapi: {}", reqs);
    }

    #[test]
    #[serial(disk)]
    fn test_python_django_template() {
        let path = run_template_smoke(
            "python-django",
            "my-django",
            &["requirements.txt", "manage.py", ".gitignore", "README.md"],
        );
        let reqs = std::fs::read_to_string(path.join("requirements.txt")).unwrap();
        assert!(reqs.to_lowercase().contains("django"),
            "requirements.txt de Django debe declarar django: {}", reqs);
        let manage = std::fs::read_to_string(path.join("manage.py")).unwrap();
        assert!(manage.contains("DJANGO_SETTINGS_MODULE") || manage.contains("django"),
            "manage.py debe ser un launcher Django");
    }

    #[test]
    #[serial(disk)]
    fn test_fullstack_react_python_template() {
        let path = run_template_smoke(
            "fullstack-react-python",
            "my-fullstack",
            &[
                "frontend/package.json",
                "frontend/src/App.tsx",
                "backend/requirements.txt",
                "backend/app/main.py",
            ],
        );
        let frontend_pkg = std::fs::read_to_string(path.join("frontend/package.json")).unwrap();
        let _: serde_json::Value = serde_json::from_str(&frontend_pkg)
            .expect("frontend/package.json debe ser JSON valido");
        let backend_reqs = std::fs::read_to_string(path.join("backend/requirements.txt")).unwrap();
        assert!(!backend_reqs.trim().is_empty(), "backend/requirements.txt no puede estar vacio");
    }

    #[test]
    #[serial(disk)]
    fn test_rust_cli_template() {
        let path = run_template_smoke(
            "rust-cli",
            "my-rust-cli",
            &["Cargo.toml", "src/main.rs", ".gitignore", "README.md"],
        );
        let cargo = std::fs::read_to_string(path.join("Cargo.toml")).unwrap();
        assert!(cargo.contains("[package]"), "Cargo.toml debe tener seccion [package]");
        assert!(cargo.contains("name ="), "Cargo.toml debe declarar 'name ='");
        let main_rs = std::fs::read_to_string(path.join("src/main.rs")).unwrap();
        assert!(main_rs.contains("fn main"), "src/main.rs debe tener fn main");
    }

    #[test]
    #[serial(disk)]
    fn test_docker_compose_template() {
        let path = run_template_smoke(
            "docker-compose",
            "my-stack",
            &["docker-compose.yml", ".gitignore", "README.md", ".env.example"],
        );
        let compose = std::fs::read_to_string(path.join("docker-compose.yml")).unwrap();
        let parsed: serde_yml::Value = serde_yml::from_str(&compose)
            .expect("docker-compose.yml debe ser YAML valido");
        assert!(parsed.get("services").is_some(),
            "docker-compose.yml debe tener clave 'services'");
    }

    // --- Parametrizado: cobertura de TODOS los templates en una sola pasada ---

    #[test]
    #[serial(disk)]
    fn test_every_template_generates_a_valid_nexenv_manifest() {
        for tmpl in all_templates() {
            crate::core::store::init_test_store();
            let dir = tempfile::tempdir().unwrap();
            let name = format!("smoke-{}", tmpl.id);
            let path = dir.path().join(&name);
            let path_str = path.to_str().unwrap().to_string();

            let result = create_from_template(tmpl.id, &path_str, &name);
            assert!(
                result.is_ok(),
                "template '{}' fallo en generacion: {:?}",
                tmpl.id,
                result.err()
            );

            let manifest = crate::core::manifest::load_manifest(&path_str)
                .expect("load_manifest no debe fallar");
            assert!(
                manifest.is_some(),
                "template '{}' no genero manifest Nexenv",
                tmpl.id
            );

            cleanup_by_path(&path_str);
        }
    }

    #[test]
    #[serial(disk)]
    fn test_invalid_template_returns_error() {
        crate::core::store::init_test_store();
        let dir = tempfile::tempdir().unwrap();
        let result = create_from_template("nonexistent", dir.path().to_str().unwrap(), "test");
        assert!(result.is_err());
    }
}
