use crate::core::manifest::{self, ProjectManifest};
use crate::core::storage;
use tauri::command;

/// Obtiene el manifest de un proyecto
#[command]
pub async fn get_manifest(project_id: String) -> Result<Option<ProjectManifest>, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    manifest::load_manifest(&project.path).map_err(|e| e.to_string())
}

/// Regenera el manifest de un proyecto desde su estado actual
#[command]
pub async fn regenerate_manifest(project_id: String) -> Result<ProjectManifest, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    let new_manifest = manifest::generate_manifest_from_project(project);
    manifest::save_manifest(&project.path, &new_manifest).map_err(|e| e.to_string())?;
    Ok(new_manifest)
}
