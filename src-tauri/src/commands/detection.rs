use crate::core::detection::{self, DetectedStack};
use tauri::command;

/// Detecta automaticamente el stack de un proyecto existente
#[command]
pub async fn detect_project_stack(path: String) -> Result<DetectedStack, String> {
    detection::detect_stack(&path).map_err(|e| e.to_string())
}
