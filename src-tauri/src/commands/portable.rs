use crate::core::models::project::Project;
use crate::core::portable;
use tauri::command;

/// Exporta un proyecto como JSON portable
#[command]
pub async fn export_project(project_id: String) -> Result<String, String> {
    portable::export_project(&project_id).map_err(|e| e.to_string())
}

/// Devuelve un resumen del import sin tocar disco ni store
#[command]
pub async fn preview_import(
    json: String,
    target_path: String,
) -> Result<crate::core::project::portable::ImportPreview, crate::core::errors::UiError> {
    use crate::core::errors::UiError;
    crate::core::project::portable::preview_import(&json, &target_path).map_err(|e| {
        UiError::new("preview de import")
            .detecto(format!("destino: '{}'", target_path))
            .fallo(e.to_string())
            .hacer("verifica que el JSON sea valido y que la ruta destino sea absoluta")
    })
}

/// Importa un proyecto desde JSON portable
#[command]
pub async fn import_project(
    json: String,
    target_path: String,
) -> Result<Project, crate::core::errors::UiError> {
    use crate::core::errors::UiError;
    portable::import_project(&json, &target_path).map_err(|e| {
        UiError::new("importar proyecto desde archivo .nexenv")
            .detecto(format!("destino: '{}'", target_path))
            .fallo(e.to_string())
            .hacer("verifica que el JSON sea valido, que la carpeta destino sea accesible y que no exista un proyecto con el mismo id")
    })
}
