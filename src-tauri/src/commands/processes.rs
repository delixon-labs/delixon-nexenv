use crate::core::processes::{self, ProjectProcess};
use crate::core::store;
use tauri::command;

#[command]
pub async fn list_project_processes(project_id: String) -> Result<Vec<ProjectProcess>, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    processes::list_processes_on_ports(&project.path).map_err(|e| e.to_string())
}

#[command]
pub async fn kill_process(pid: u32) -> Result<(), String> {
    processes::kill_process(pid).map_err(|e| e.to_string())
}
