use crate::models::project::{CreateProjectInput, Project};
use tauri::command;

/// Devuelve la lista de todos los proyectos registrados en Delixon
#[command]
pub async fn list_projects() -> Result<Vec<Project>, String> {
    // TODO: Fase 1 — leer proyectos desde el almacenamiento local
    Ok(vec![])
}

/// Crea un nuevo proyecto a partir de los datos del input
#[command]
pub async fn create_project(input: CreateProjectInput) -> Result<Project, String> {
    // TODO: Fase 1 — crear carpeta, configurar entorno, registrar proyecto
    Err(format!("create_project no implementado aún: {:?}", input.name))
}

/// Abre un proyecto en VSCode con el entorno correcto cargado
#[command]
pub async fn open_project(id: String) -> Result<(), String> {
    // TODO: Fase 1 — activar entorno del proyecto y abrir VSCode
    Err(format!("open_project no implementado aún: {}", id))
}

/// Elimina un proyecto del registro de Delixon (no borra los archivos)
#[command]
pub async fn delete_project(id: String) -> Result<(), String> {
    // TODO: Fase 1 — eliminar registro del proyecto
    Err(format!("delete_project no implementado aún: {}", id))
}
