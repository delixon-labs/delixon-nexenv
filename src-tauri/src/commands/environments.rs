use std::collections::HashMap;
use tauri::command;

/// Obtiene las variables de entorno de un proyecto
#[command]
pub async fn get_env_vars(project_id: String) -> Result<HashMap<String, String>, String> {
    // TODO: Fase 1 — leer variables de entorno del proyecto
    let _ = project_id;
    Ok(HashMap::new())
}

/// Establece las variables de entorno de un proyecto
#[command]
pub async fn set_env_vars(
    project_id: String,
    vars: HashMap<String, String>,
) -> Result<(), String> {
    // TODO: Fase 1 — guardar variables de entorno del proyecto
    let _ = (project_id, vars);
    Ok(())
}
