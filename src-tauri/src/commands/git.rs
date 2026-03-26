use crate::core::git::{self, GitCommit, GitStatus};
use crate::core::storage;
use tauri::command;

#[command]
pub async fn git_status(project_id: String) -> Result<GitStatus, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    git::git_status(&project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn git_log(project_id: String, count: u32) -> Result<Vec<GitCommit>, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    git::git_log(&project.path, count).map_err(|e| e.to_string())
}
