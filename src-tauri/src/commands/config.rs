use crate::core::config::NexenvConfig;
use crate::core::store;
use tauri::command;

/// Obtiene la configuracion global de Nexenv
#[command]
pub async fn get_config() -> Result<NexenvConfig, String> {
    store::get().load_config().map_err(|e| e.to_string())
}

/// Guarda la configuracion global de Nexenv
#[command]
pub async fn set_config(config: NexenvConfig) -> Result<(), String> {
    store::get().save_config(&config).map_err(|e| e.to_string())
}
