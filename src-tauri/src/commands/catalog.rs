use crate::core::catalog::{self, Technology};
use tauri::command;

/// Lista todas las tecnologias del catalogo
#[command]
pub async fn list_catalog() -> Result<Vec<Technology>, String> {
    Ok(catalog::load_all_technologies())
}

/// Obtiene una tecnologia por su ID
#[command]
pub async fn get_catalog_tech(id: String) -> Result<Technology, String> {
    catalog::get_technology(&id).ok_or_else(|| format!("Tecnologia no encontrada: {}", id))
}

/// Lista las categorias disponibles
#[command]
pub async fn list_catalog_categories() -> Result<Vec<String>, String> {
    Ok(catalog::all_categories())
}
