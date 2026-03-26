use crate::core::storage;
use crate::core::vscode;
use tauri::command;

/// Genera un archivo .code-workspace para el proyecto
#[command]
pub async fn generate_vscode_workspace(project_id: String) -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    vscode::write_workspace(project).map_err(|e| e.to_string())
}
