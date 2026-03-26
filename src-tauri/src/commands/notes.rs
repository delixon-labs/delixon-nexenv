use crate::core::notes::{self, ProjectNote};
use tauri::command;

#[command]
pub async fn get_notes(project_id: String) -> Result<Vec<ProjectNote>, String> {
    notes::get_notes(&project_id).map_err(|e| e.to_string())
}

#[command]
pub async fn add_note(project_id: String, text: String) -> Result<ProjectNote, String> {
    notes::add_note(&project_id, &text).map_err(|e| e.to_string())
}

#[command]
pub async fn delete_note(project_id: String, note_id: String) -> Result<(), String> {
    notes::delete_note(&project_id, &note_id).map_err(|e| e.to_string())
}
