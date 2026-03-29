use crate::core::versioning::{self, Snapshot, SnapshotDiff};
use crate::core::store;
use tauri::command;

#[command]
pub async fn save_snapshot(project_id: String) -> Result<Snapshot, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    versioning::save_snapshot(&project_id, &project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn list_snapshots(project_id: String) -> Result<Vec<Snapshot>, String> {
    versioning::list_snapshots(&project_id).map_err(|e| e.to_string())
}

#[command]
pub async fn diff_snapshots(project_id: String, v1: u32, v2: u32) -> Result<SnapshotDiff, String> {
    versioning::diff_snapshots(&project_id, v1, v2).map_err(|e| e.to_string())
}

#[command]
pub async fn rollback_snapshot(project_id: String, version: u32) -> Result<(), String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    versioning::rollback_snapshot(&project_id, &project.path, version).map_err(|e| e.to_string())
}
