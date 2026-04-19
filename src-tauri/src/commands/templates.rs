use crate::core::models::project::Project;
use crate::core::templates;
use tauri::command;

/// Crea un proyecto nuevo a partir de una plantilla
#[command]
pub async fn create_from_template(
    template_id: String,
    path: String,
    name: String,
) -> Result<Project, crate::core::errors::UiError> {
    use crate::core::errors::UiError;
    templates::create_from_template(&template_id, &path, &name).map_err(|e| {
        UiError::new(format!("crear proyecto '{}' desde template '{}'", name, template_id))
            .detecto(format!("ruta destino: '{}'", path))
            .fallo(e.to_string())
            .hacer("verifica que el template existe, que la carpeta destino sea escribible y que no contenga archivos en conflicto")
    })
}
