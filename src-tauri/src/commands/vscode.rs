use crate::core::detection;
use crate::core::manifest;
use crate::core::store;
use crate::core::vscode::{self, VscodeGenerationResult};
use tauri::command;

/// Genera archivos de configuracion VS Code para el proyecto
/// (.code-workspace, tasks.json, launch.json, extensions.json)
#[command]
pub async fn generate_vscode_workspace(project_id: String) -> Result<VscodeGenerationResult, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    // Cargar manifest existente o generar uno desde el proyecto
    let m = manifest::load_manifest(&project.path)
        .map_err(|e| e.to_string())?
        .unwrap_or_else(|| manifest::generate_manifest_from_project(project));

    // Detectar stack (opcional — no falla si no puede)
    let stack = detection::detect_stack(&project.path).ok();

    vscode::generate_all(project, &m, stack.as_ref()).map_err(|e| e.to_string())
}
