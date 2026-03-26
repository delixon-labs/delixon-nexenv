use crate::core::doctor::{self, DoctorReport};
use crate::core::health::{self, HealthReport};
use crate::core::ports::{self, PortConflict, PortInfo};
use crate::core::storage;
use tauri::command;

/// Ejecuta health checks para un proyecto
#[command]
pub async fn check_project_health(project_id: String) -> Result<HealthReport, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    health::check_project_health(project).map_err(|e| e.to_string())
}

/// Ejecuta diagnostico del sistema
#[command]
pub async fn run_doctor() -> Result<DoctorReport, String> {
    doctor::run_doctor().map_err(|e| e.to_string())
}

/// Detecta conflictos de puertos entre proyectos
#[command]
pub async fn detect_port_conflicts() -> Result<Vec<PortConflict>, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    ports::detect_port_conflicts(&projects).map_err(|e| e.to_string())
}

/// Lista puertos usados por proyectos
#[command]
pub async fn list_project_ports() -> Result<Vec<PortInfo>, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    ports::list_project_ports(&projects).map_err(|e| e.to_string())
}
