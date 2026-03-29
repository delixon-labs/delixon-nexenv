use crate::core::scripts::{self, ScriptResult};
use crate::core::store;
use tauri::command;

#[command]
pub async fn list_project_scripts(project_id: String) -> Result<Vec<(String, String)>, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    scripts::list_scripts(&project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn run_project_script(project_id: String, script_name: String) -> Result<ScriptResult, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    scripts::run_script(&project.path, &script_name).map_err(|e| e.to_string())
}
