use crate::core::snapshots::{self, EnvDiff, EnvSnapshot};
use crate::core::storage;
use tauri::command;

#[command]
pub async fn take_env_snapshot(project_id: String) -> Result<EnvSnapshot, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    snapshots::take_snapshot(&project_id, &project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn diff_env_snapshot(project_id: String) -> Result<Option<EnvDiff>, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    snapshots::diff_snapshot(&project_id, &project.path).map_err(|e| e.to_string())
}
