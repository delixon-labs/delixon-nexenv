use crate::core::models::project::Project;
use crate::core::portable;
use tauri::command;

/// Exporta un proyecto como JSON portable
#[command]
pub async fn export_project(project_id: String) -> Result<String, String> {
    portable::export_project(&project_id).map_err(|e| e.to_string())
}

/// Importa un proyecto desde JSON portable
#[command]
pub async fn import_project(json: String, target_path: String) -> Result<Project, String> {
    portable::import_project(&json, &target_path).map_err(|e| e.to_string())
}
