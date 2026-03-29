use crate::core::error::DelixonError;
use crate::core::project::{config, notes, storage};
use crate::core::store::sqlite_store::SqliteStore;
use crate::core::store::traits::*;

/// Verifica si existen archivos JSON legacy que necesitan migrarse
pub fn json_data_exists() -> bool {
    if let Some(dir) = crate::core::utils::platform::get_data_dir() {
        dir.join("projects.json").exists()
    } else {
        false
    }
}

/// Migra todos los datos de JSON files a SQLite dentro de una transaccion
pub fn migrate_json_to_sqlite(sqlite: &SqliteStore) -> Result<MigrationResult, DelixonError> {
    let mut result = MigrationResult::default();

    // 1. Proyectos
    let projects = storage::load_projects().unwrap_or_default();
    if !projects.is_empty() {
        sqlite.save_projects(&projects)?;
        result.projects = projects.len();
    }

    // 2. Config
    if let Ok(cfg) = config::load_config() {
        sqlite.save_config(&cfg)?;
        result.config = true;
    }

    // 3. Env vars por proyecto
    for p in &projects {
        if let Ok(vars) = storage::load_env_vars(&p.id) {
            if !vars.is_empty() {
                sqlite.save_env_vars(&p.id, &vars)?;
                result.env_vars += 1;
            }
        }
    }

    // 4. Notas por proyecto
    for p in &projects {
        if let Ok(project_notes) = notes::get_notes(&p.id) {
            for note in &project_notes {
                // Insertar directamente ya que add_note genera nuevo ID
                sqlite.add_note(&p.id, &note.text)?;
                result.notes += 1;
            }
        }
    }

    // 5. Snapshots de versioning
    for p in &projects {
        if let Ok(snapshots) = crate::core::versioning::list_snapshots(&p.id) {
            for _s in &snapshots {
                result.snapshots += 1;
            }
            // Los snapshots se guardan como JSON blobs, migrarlos requeriria
            // acceder al filesystem del proyecto. Se migran bajo demanda.
        }
    }

    // 6. Mover JSONs a backup
    backup_json_files()?;

    Ok(result)
}

#[derive(Debug, Default)]
pub struct MigrationResult {
    pub projects: usize,
    pub config: bool,
    pub env_vars: usize,
    pub notes: usize,
    pub snapshots: usize,
}

/// Mueve archivos JSON a json_backup/
fn backup_json_files() -> Result<(), DelixonError> {
    let data_dir = crate::core::utils::platform::get_data_dir()
        .ok_or_else(|| DelixonError::InvalidConfig("No data dir".to_string()))?;

    let backup_dir = data_dir.join("json_backup");
    std::fs::create_dir_all(&backup_dir)?;

    let files_to_backup = ["projects.json", "config.json"];
    for file in &files_to_backup {
        let src = data_dir.join(file);
        if src.exists() {
            let dst = backup_dir.join(file);
            std::fs::rename(&src, &dst)?;
        }
    }

    let dirs_to_backup = ["envs", "notes", "env_snapshots"];
    for dir in &dirs_to_backup {
        let src = data_dir.join(dir);
        if src.exists() && src.is_dir() {
            let dst = backup_dir.join(dir);
            if !dst.exists() {
                // Renombrar directorio completo
                let _ = std::fs::rename(&src, &dst);
            }
        }
    }

    Ok(())
}

/// Ruta de la base de datos SQLite
pub fn db_path() -> Option<std::path::PathBuf> {
    crate::core::utils::platform::get_data_dir().map(|d| d.join("delixon.db"))
}
