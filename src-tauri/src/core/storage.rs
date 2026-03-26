use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::error::DelixonError;
use crate::core::models::project::Project;
use crate::core::utils::fs::{ensure_dir, write_private};
use crate::core::utils::platform::get_data_dir;

fn data_dir() -> Result<PathBuf, DelixonError> {
    get_data_dir().ok_or_else(|| {
        DelixonError::InvalidConfig("No se pudo determinar el directorio de datos".to_string())
    })
}

fn projects_file() -> Result<PathBuf, DelixonError> {
    Ok(data_dir()?.join("projects.json"))
}

fn envs_dir() -> Result<PathBuf, DelixonError> {
    Ok(data_dir()?.join("envs"))
}

pub fn history_dir() -> Result<PathBuf, DelixonError> {
    Ok(data_dir()?.join("history"))
}

pub fn get_history_path(project_id: &str) -> Result<PathBuf, DelixonError> {
    Ok(history_dir()?.join(format!("{}.txt", project_id)))
}

/// Inicializa el directorio de datos si no existe
pub fn init_data_dir() -> Result<(), DelixonError> {
    let dir = data_dir()?;
    ensure_dir(&dir)?;
    ensure_dir(&dir.join("envs"))?;
    ensure_dir(&dir.join("history"))?;
    Ok(())
}

// --- Proyectos ---

pub fn load_projects() -> Result<Vec<Project>, DelixonError> {
    let path = projects_file()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = std::fs::read_to_string(&path)?;
    let projects: Vec<Project> = serde_json::from_str(&data)?;
    Ok(projects)
}

pub fn save_projects(projects: &[Project]) -> Result<(), DelixonError> {
    init_data_dir()?;
    let path = projects_file()?;
    let data = serde_json::to_string_pretty(projects)?;
    write_private(&path, &data)?;
    Ok(())
}

// --- Variables de entorno por proyecto ---

pub fn load_env_vars(project_id: &str) -> Result<HashMap<String, String>, DelixonError> {
    let path = envs_dir()?.join(format!("{}.json", project_id));
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let data = std::fs::read_to_string(&path)?;
    let vars: HashMap<String, String> = serde_json::from_str(&data)?;
    Ok(vars)
}

pub fn save_env_vars(
    project_id: &str,
    vars: &HashMap<String, String>,
) -> Result<(), DelixonError> {
    init_data_dir()?;
    let path = envs_dir()?.join(format!("{}.json", project_id));
    let data = serde_json::to_string_pretty(vars)?;
    write_private(&path, &data)?;
    Ok(())
}

pub fn delete_env_vars(project_id: &str) -> Result<(), DelixonError> {
    let path = envs_dir()?.join(format!("{}.json", project_id));
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};

    fn make_test_project(suffix: &str) -> Project {
        Project {
            id: format!("test-storage-{}", suffix),
            name: format!("Test Project {}", suffix),
            path: format!("/tmp/delixon-test-{}", suffix),
            description: Some("test project".to_string()),
            runtimes: vec![RuntimeConfig {
                runtime: "node".to_string(),
                version: "20".to_string(),
            }],
            status: ProjectStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            last_opened_at: None,
            template_id: None,
            tags: vec!["test".to_string()],
        }
    }

    /// Helper: remove a test project from the saved list by id
    fn cleanup_project(id: &str) {
        if let Ok(projects) = load_projects() {
            let filtered: Vec<_> = projects.into_iter().filter(|p| p.id != id).collect();
            let _ = save_projects(&filtered);
        }
    }

    #[test]
    fn test_init_data_dir() {
        // First call creates dirs
        init_data_dir().expect("init_data_dir should succeed");
        // Second call is idempotent
        init_data_dir().expect("init_data_dir should be idempotent");

        let dir = data_dir().unwrap();
        assert!(dir.join("envs").exists());
        assert!(dir.join("history").exists());
    }

    #[test]
    fn test_save_load_projects_roundtrip() {
        let proj = make_test_project("roundtrip");

        // Load existing, append ours, save
        let mut projects = load_projects().unwrap_or_default();
        projects.retain(|p| p.id != proj.id);
        projects.push(proj.clone());
        save_projects(&projects).expect("save should succeed");

        let loaded = load_projects().expect("load should succeed");
        let found = loaded.iter().find(|p| p.id == proj.id);
        assert!(found.is_some(), "saved project should be loadable");
        let found = found.unwrap();
        assert_eq!(found.name, proj.name);
        assert_eq!(found.path, proj.path);
        assert_eq!(found.status, ProjectStatus::Active);

        cleanup_project(&proj.id);
    }

    #[test]
    fn test_load_projects_empty() {
        // load_projects must not panic even if the file is valid
        let result = load_projects();
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_load_env_vars_roundtrip() {
        let id = "test-env-roundtrip";
        let mut vars = HashMap::new();
        vars.insert("API_KEY".to_string(), "secret123".to_string());
        vars.insert("DB_HOST".to_string(), "localhost".to_string());

        save_env_vars(id, &vars).expect("save env vars should succeed");
        let loaded = load_env_vars(id).expect("load env vars should succeed");
        assert_eq!(loaded.get("API_KEY").unwrap(), "secret123");
        assert_eq!(loaded.get("DB_HOST").unwrap(), "localhost");

        // cleanup
        let _ = delete_env_vars(id);
    }

    #[test]
    fn test_load_env_vars_nonexistent() {
        let result = load_env_vars("nonexistent-project-xyz-999");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_delete_env_vars() {
        let id = "test-env-delete";
        let mut vars = HashMap::new();
        vars.insert("KEY".to_string(), "val".to_string());
        save_env_vars(id, &vars).unwrap();

        delete_env_vars(id).expect("delete should succeed");
        let loaded = load_env_vars(id).unwrap();
        assert!(loaded.is_empty(), "after delete, env vars should be empty");
    }

    #[test]
    fn test_delete_env_vars_nonexistent() {
        let result = delete_env_vars("nonexistent-env-xyz-999");
        assert!(result.is_ok(), "deleting nonexistent env vars should not error");
    }

    #[test]
    fn test_history_dir() {
        let dir = history_dir().expect("history_dir should return Ok");
        assert!(dir.to_str().unwrap().contains("history"));
    }

    #[test]
    fn test_get_history_path() {
        let path = get_history_path("my-project-123").expect("should return Ok");
        let path_str = path.to_str().unwrap();
        assert!(path_str.contains("my-project-123"));
        assert!(path_str.ends_with(".txt"));
    }
}
