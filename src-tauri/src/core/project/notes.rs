use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::core::error::DelixonError;
use crate::core::utils::fs::{ensure_dir, write_private};
use crate::core::utils::platform::get_data_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectNote {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

fn notes_file(project_id: &str) -> Result<PathBuf, DelixonError> {
    let base = get_data_dir().ok_or_else(|| {
        DelixonError::InvalidConfig("No se pudo determinar directorio de datos".to_string())
    })?;
    let dir = base.join("notes");
    ensure_dir(&dir)?;
    Ok(dir.join(format!("{}.json", project_id)))
}

pub fn get_notes(project_id: &str) -> Result<Vec<ProjectNote>, DelixonError> {
    let file = notes_file(project_id)?;
    if !file.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(&file)?;
    let notes: Vec<ProjectNote> = serde_json::from_str(&data)?;
    Ok(notes)
}

pub fn add_note(project_id: &str, text: &str) -> Result<ProjectNote, DelixonError> {
    let mut notes = get_notes(project_id)?;
    let note = ProjectNote {
        id: uuid::Uuid::new_v4().to_string(),
        text: text.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    notes.push(note.clone());

    let file = notes_file(project_id)?;
    let data = serde_json::to_string_pretty(&notes)?;
    write_private(&file, &data)?;

    Ok(note)
}

pub fn delete_note(project_id: &str, note_id: &str) -> Result<(), DelixonError> {
    let mut notes = get_notes(project_id)?;
    notes.retain(|n| n.id != note_id);

    let file = notes_file(project_id)?;
    let data = serde_json::to_string_pretty(&notes)?;
    write_private(&file, &data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_notes() {
        let project_id = "test-notes-1";
        let note = add_note(project_id, "primera nota").unwrap();
        assert_eq!(note.text, "primera nota");

        let notes = get_notes(project_id).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].text, "primera nota");

        // Cleanup
        let _ = std::fs::remove_file(notes_file(project_id).unwrap());
    }

    #[test]
    fn test_delete_note() {
        let project_id = "test-notes-2";
        let note = add_note(project_id, "to delete").unwrap();
        delete_note(project_id, &note.id).unwrap();

        let notes = get_notes(project_id).unwrap();
        assert!(notes.is_empty());

        // Cleanup
        let _ = std::fs::remove_file(notes_file(project_id).unwrap());
    }

    #[test]
    fn test_get_notes_empty() {
        let notes = get_notes("nonexistent-notes-xyz").unwrap();
        assert!(notes.is_empty());
    }
}
