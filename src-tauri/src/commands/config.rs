use crate::core::config::{self, DelixonConfig};
use tauri::command;

/// Obtiene la configuracion global de Delixon
#[command]
pub async fn get_config() -> Result<DelixonConfig, String> {
    config::load_config().map_err(|e| e.to_string())
}

/// Guarda la configuracion global de Delixon
#[command]
pub async fn set_config(config: DelixonConfig) -> Result<(), String> {
    config::save_config(&config).map_err(|e| e.to_string())
}
