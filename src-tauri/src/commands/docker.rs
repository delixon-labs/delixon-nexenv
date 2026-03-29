use crate::core::docker::{self, DockerComposeStatus};
use crate::core::store;
use tauri::command;

#[command]
pub async fn docker_status(project_id: String) -> Result<DockerComposeStatus, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    docker::compose_status(&project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn docker_up(project_id: String) -> Result<String, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    docker::compose_up(&project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn docker_down(project_id: String) -> Result<String, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    docker::compose_down(&project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn docker_logs(project_id: String, lines: u32) -> Result<String, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    docker::compose_logs(&project.path, lines).map_err(|e| e.to_string())
}
