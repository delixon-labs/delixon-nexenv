use crate::core::catalog::{self, Technology, load_all_technologies};
use tauri::command;

#[command]
pub async fn list_catalog() -> Result<Vec<Technology>, String> {
    Ok(load_all_technologies().to_vec())
}

#[command]
pub async fn get_catalog_tech(id: String) -> Result<Technology, String> {
    catalog::get_technology(&id)
        .cloned()
        .ok_or_else(|| format!("Tecnologia no encontrada: {}", id))
}

#[command]
pub async fn list_catalog_categories() -> Result<Vec<String>, String> {
    Ok(catalog::all_categories())
}
