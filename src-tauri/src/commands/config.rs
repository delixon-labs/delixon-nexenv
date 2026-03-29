use crate::core::config::DelixonConfig;
use crate::core::store;
use tauri::command;

/// Obtiene la configuracion global de Delixon
#[command]
pub async fn get_config() -> Result<DelixonConfig, String> {
    store::get().load_config().map_err(|e| e.to_string())
}

/// Guarda la configuracion global de Delixon
#[command]
pub async fn set_config(config: DelixonConfig) -> Result<(), String> {
    store::get().save_config(&config).map_err(|e| e.to_string())
}
