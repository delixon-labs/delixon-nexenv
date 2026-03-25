use std::collections::HashMap;
use tauri::command;

use crate::storage;

/// Obtiene las variables de entorno de un proyecto
#[command]
pub async fn get_env_vars(project_id: String) -> Result<HashMap<String, String>, String> {
    storage::load_env_vars(&project_id)
}

/// Establece las variables de entorno de un proyecto
#[command]
pub async fn set_env_vars(
    project_id: String,
    vars: HashMap<String, String>,
) -> Result<(), String> {
    // Verificar que el proyecto existe
    let projects = storage::load_projects()?;
    if !projects.iter().any(|p| p.id == project_id) {
        return Err(format!("Proyecto no encontrado: {}", project_id));
    }
    storage::save_env_vars(&project_id, &vars)
}
