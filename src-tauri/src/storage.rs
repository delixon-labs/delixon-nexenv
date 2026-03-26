use std::collections::HashMap;
use std::path::PathBuf;

use crate::models::project::Project;
use crate::utils::fs::ensure_dir;
use crate::utils::platform::get_data_dir;

fn data_dir() -> Result<PathBuf, String> {
    get_data_dir().ok_or_else(|| "No se pudo determinar el directorio de datos".to_string())
}

fn projects_file() -> Result<PathBuf, String> {
    Ok(data_dir()?.join("projects.json"))
}

fn envs_dir() -> Result<PathBuf, String> {
    Ok(data_dir()?.join("envs"))
}

/// Inicializa el directorio de datos si no existe
pub fn init_data_dir() -> Result<(), String> {
    let dir = data_dir()?;
    ensure_dir(&dir).map_err(|e| format!("Error creando directorio de datos: {}", e))?;
    ensure_dir(&dir.join("envs")).map_err(|e| format!("Error creando directorio envs: {}", e))?;
    Ok(())
}

// --- Proyectos ---

pub fn load_projects() -> Result<Vec<Project>, String> {
    let path = projects_file()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = std::fs::read_to_string(&path)
        .map_err(|e| format!("Error leyendo projects.json: {}", e))?;
    serde_json::from_str(&data)
        .map_err(|e| format!("Error parseando projects.json: {}", e))
}

pub fn save_projects(projects: &[Project]) -> Result<(), String> {
    init_data_dir()?;
    let path = projects_file()?;
    let data = serde_json::to_string_pretty(projects)
        .map_err(|e| format!("Error serializando proyectos: {}", e))?;
    std::fs::write(&path, data)
        .map_err(|e| format!("Error escribiendo projects.json: {}", e))
}

// --- Variables de entorno por proyecto ---

pub fn load_env_vars(project_id: &str) -> Result<HashMap<String, String>, String> {
    let path = envs_dir()?.join(format!("{}.json", project_id));
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let data = std::fs::read_to_string(&path)
        .map_err(|e| format!("Error leyendo env vars: {}", e))?;
    serde_json::from_str(&data)
        .map_err(|e| format!("Error parseando env vars: {}", e))
}

pub fn save_env_vars(project_id: &str, vars: &HashMap<String, String>) -> Result<(), String> {
    init_data_dir()?;
    let path = envs_dir()?.join(format!("{}.json", project_id));
    let data = serde_json::to_string_pretty(vars)
        .map_err(|e| format!("Error serializando env vars: {}", e))?;
    std::fs::write(&path, data)
        .map_err(|e| format!("Error escribiendo env vars: {}", e))
}

pub fn delete_env_vars(project_id: &str) -> Result<(), String> {
    let path = envs_dir()?.join(format!("{}.json", project_id));
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Error eliminando env vars: {}", e))?;
    }
    Ok(())
}
