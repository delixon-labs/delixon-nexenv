use crate::core::models::project::Project;
use crate::core::templates;
use tauri::command;

/// Crea un proyecto nuevo a partir de una plantilla
#[command]
pub async fn create_from_template(
    template_id: String,
    path: String,
    name: String,
) -> Result<Project, String> {
    templates::create_from_template(&template_id, &path, &name).map_err(|e| e.to_string())
}
