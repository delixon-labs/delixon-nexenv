use crate::core::versioning::{self, RollbackPreview, Snapshot, SnapshotDiff};
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
pub async fn preview_rollback(
    project_id: String,
    version: u32,
) -> Result<RollbackPreview, crate::core::errors::UiError> {
    use crate::core::errors::UiError;
    let projects = store::get().list_projects().map_err(|e| {
        UiError::new(format!("preview rollback v{}", version))
            .detecto("no se pudo leer el listado de proyectos")
            .fallo(e.to_string())
            .hacer("verifica los permisos del data dir de Nexenv")
    })?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| {
            UiError::new(format!("preview rollback v{}", version))
                .detecto(format!("no existe el proyecto con id '{}'", project_id))
                .fallo("Project not found")
                .hacer("vuelve al dashboard y selecciona un proyecto valido")
        })?;
    versioning::preview_rollback(&project_id, &project.path, version).map_err(|e| {
        UiError::new(format!("preview rollback v{}", version))
            .detecto(format!("proyecto: '{}'", project.name))
            .fallo(e.to_string())
            .hacer("verifica que el snapshot existe y que el manifest sea legible")
    })
}

#[command]
pub async fn rollback_snapshot(
    project_id: String,
    version: u32,
) -> Result<(), crate::core::errors::UiError> {
    use crate::core::errors::UiError;
    let projects = store::get().list_projects().map_err(|e| {
        UiError::new(format!("rollback al snapshot v{}", version))
            .detecto("no se pudo leer el listado de proyectos")
            .fallo(e.to_string())
            .hacer("verifica los permisos del data dir de Nexenv")
    })?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| {
            UiError::new(format!("rollback al snapshot v{}", version))
                .detecto(format!("no existe el proyecto con id '{}'", project_id))
                .fallo("Project not found")
                .hacer("vuelve al dashboard y selecciona un proyecto valido")
        })?;

    versioning::rollback_snapshot(&project_id, &project.path, version).map_err(|e| {
        UiError::new(format!("rollback al snapshot v{}", version))
            .detecto(format!("proyecto: '{}' (v{})", project.name, version))
            .fallo(e.to_string())
            .hacer("verifica que el snapshot existe y que la carpeta del proyecto es escribible")
    })
}
