use crate::core::notes::ProjectNote;
use crate::core::store;
use tauri::command;

#[command]
pub async fn get_notes(project_id: String) -> Result<Vec<ProjectNote>, String> {
    store::get().get_notes(&project_id).map_err(|e| e.to_string())
}

#[command]
pub async fn add_note(project_id: String, text: String) -> Result<ProjectNote, String> {
    store::get().add_note(&project_id, &text).map_err(|e| e.to_string())
}

#[command]
pub async fn delete_note(project_id: String, note_id: String) -> Result<(), String> {
    store::get().delete_note(&project_id, &note_id).map_err(|e| e.to_string())
}
