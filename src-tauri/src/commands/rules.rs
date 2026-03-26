use crate::core::rules::{self, ValidationResult};
use tauri::command;

/// Valida una combinacion de tecnologias
#[command]
pub async fn validate_stack(technology_ids: Vec<String>) -> Result<ValidationResult, String> {
    Ok(rules::validate_stack(&technology_ids))
}
