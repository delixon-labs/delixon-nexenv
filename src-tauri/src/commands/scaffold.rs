use crate::core::models::project::Project;
use crate::core::scaffold::{self, ScaffoldConfig, ScaffoldPreview};
use tauri::command;

#[command]
pub async fn preview_scaffold(config: ScaffoldConfig) -> Result<ScaffoldPreview, String> {
    Ok(scaffold::preview_scaffold(&config))
}

#[command]
pub async fn generate_scaffold(config: ScaffoldConfig) -> Result<Project, String> {
    let result = scaffold::generate_project(&config).map_err(|e| e.to_string())?;
    scaffold::register_scaffolded_project(&config, &result).map_err(|e| e.to_string())
}
