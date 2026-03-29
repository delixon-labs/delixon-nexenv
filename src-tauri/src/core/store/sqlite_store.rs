use std::collections::HashMap;
use std::sync::Mutex;

use rusqlite::{params, Connection};

use crate::core::error::DelixonError;
use crate::core::history::env::{EnvDiff, EnvSnapshot, RuntimeSnapshot};
use crate::core::history::versioning::{Snapshot, SnapshotDiff};
use crate::core::manifest::ProjectManifest;
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::project::config::DelixonConfig;
use crate::core::project::notes::ProjectNote;
use crate::core::store::traits::*;

use super::migrations;

pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    pub fn new(path: &str) -> Result<Self, DelixonError> {
        let conn = Connection::open(path)
            .map_err(|e| DelixonError::Database(e.to_string()))?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| DelixonError::Database(e.to_string()))?;
        let store = SqliteStore { conn: Mutex::new(conn) };
        store.run_migrations()?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self, DelixonError> {
        Self::new(":memory:")
    }

    fn run_migrations(&self) -> Result<(), DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        migrations::run_migrations(&conn)
    }

    fn db_err(e: rusqlite::Error) -> DelixonError {
        DelixonError::Database(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// ProjectStore
// ---------------------------------------------------------------------------

impl ProjectStore for SqliteStore {
    fn list_projects(&self) -> Result<Vec<Project>, DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, path, description, status, created_at, last_opened_at, template_id FROM projects ORDER BY name"
        ).map_err(Self::db_err)?;

        let project_rows: Vec<(String, String, String, Option<String>, String, String, Option<String>, Option<String>)> = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?,
                    row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?,
                ))
            })
            .map_err(Self::db_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::db_err)?;

        let mut projects = Vec::with_capacity(project_rows.len());
        for (id, name, path, description, status, created_at, last_opened_at, template_id) in project_rows {
            let runtimes = conn
                .prepare("SELECT runtime, version FROM project_runtimes WHERE project_id = ?1")
                .map_err(Self::db_err)?
                .query_map([&id], |row| {
                    Ok(RuntimeConfig { runtime: row.get(0)?, version: row.get(1)? })
                })
                .map_err(Self::db_err)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(Self::db_err)?;

            let tags = conn
                .prepare("SELECT tag FROM project_tags WHERE project_id = ?1 ORDER BY tag")
                .map_err(Self::db_err)?
                .query_map([&id], |row| row.get(0))
                .map_err(Self::db_err)?
                .collect::<Result<Vec<String>, _>>()
                .map_err(Self::db_err)?;

            let status_enum = match status.as_str() {
                "idle" => ProjectStatus::Idle,
                "archived" => ProjectStatus::Archived,
                _ => ProjectStatus::Active,
            };

            projects.push(Project {
                id, name, path, description, runtimes, status: status_enum,
                created_at, last_opened_at, template_id, tags,
            });
        }

        Ok(projects)
    }

    fn save_projects(&self, projects: &[Project]) -> Result<(), DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;

        conn.execute_batch("BEGIN TRANSACTION;").map_err(Self::db_err)?;

        // Limpiar tablas hijas primero (CASCADE no aplica en DELETE FROM projects)
        conn.execute("DELETE FROM project_runtimes", []).map_err(Self::db_err)?;
        conn.execute("DELETE FROM project_tags", []).map_err(Self::db_err)?;
        conn.execute("DELETE FROM projects", []).map_err(Self::db_err)?;

        for p in projects {
            let status_str = match p.status {
                ProjectStatus::Active => "active",
                ProjectStatus::Idle => "idle",
                ProjectStatus::Archived => "archived",
            };

            conn.execute(
                "INSERT INTO projects (id, name, path, description, status, created_at, last_opened_at, template_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![p.id, p.name, p.path, p.description, status_str, p.created_at, p.last_opened_at, p.template_id],
            ).map_err(Self::db_err)?;

            for rt in &p.runtimes {
                conn.execute(
                    "INSERT INTO project_runtimes (project_id, runtime, version) VALUES (?1, ?2, ?3)",
                    params![p.id, rt.runtime, rt.version],
                ).map_err(Self::db_err)?;
            }

            for tag in &p.tags {
                conn.execute(
                    "INSERT INTO project_tags (project_id, tag) VALUES (?1, ?2)",
                    params![p.id, tag],
                ).map_err(Self::db_err)?;
            }
        }

        conn.execute_batch("COMMIT;").map_err(Self::db_err)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ConfigStore
// ---------------------------------------------------------------------------

impl ConfigStore for SqliteStore {
    fn load_config(&self) -> Result<DelixonConfig, DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT version, data_dir, default_editor, theme, language, auto_check_updates FROM config WHERE id = 1"
        ).map_err(Self::db_err)?;

        stmt.query_row([], |row| {
            Ok(DelixonConfig {
                version: row.get(0)?,
                data_dir: row.get(1)?,
                default_editor: row.get(2)?,
                theme: row.get(3)?,
                language: row.get(4)?,
                auto_check_updates: row.get::<_, i32>(5)? != 0,
            })
        }).map_err(Self::db_err)
    }

    fn save_config(&self, config: &DelixonConfig) -> Result<(), DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        conn.execute(
            "UPDATE config SET version = ?1, data_dir = ?2, default_editor = ?3, theme = ?4, language = ?5, auto_check_updates = ?6 WHERE id = 1",
            params![config.version, config.data_dir, config.default_editor, config.theme, config.language, config.auto_check_updates as i32],
        ).map_err(Self::db_err)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// NoteStore
// ---------------------------------------------------------------------------

impl NoteStore for SqliteStore {
    fn get_notes(&self, project_id: &str) -> Result<Vec<ProjectNote>, DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, text, created_at FROM notes WHERE project_id = ?1 ORDER BY created_at DESC"
        ).map_err(Self::db_err)?;

        let notes: Vec<ProjectNote> = stmt.query_map([project_id], |row| {
            Ok(ProjectNote {
                id: row.get(0)?,
                text: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .map_err(Self::db_err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(Self::db_err)?;
        Ok(notes)
    }

    fn add_note(&self, project_id: &str, text: &str) -> Result<ProjectNote, DelixonError> {
        let note = ProjectNote {
            id: uuid::Uuid::new_v4().to_string(),
            text: text.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO notes (id, project_id, text, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![note.id, project_id, note.text, note.created_at],
        ).map_err(Self::db_err)?;
        Ok(note)
    }

    fn delete_note(&self, project_id: &str, note_id: &str) -> Result<(), DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        conn.execute(
            "DELETE FROM notes WHERE id = ?1 AND project_id = ?2",
            params![note_id, project_id],
        ).map_err(Self::db_err)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// EnvVarStore
// ---------------------------------------------------------------------------

impl EnvVarStore for SqliteStore {
    fn load_env_vars(&self, project_id: &str) -> Result<HashMap<String, String>, DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT key, value FROM env_vars WHERE project_id = ?1"
        ).map_err(Self::db_err)?;

        let mut vars = HashMap::new();
        let rows = stmt.query_map([project_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).map_err(Self::db_err)?;

        for row in rows {
            let (key, value) = row.map_err(Self::db_err)?;
            vars.insert(key, value);
        }
        Ok(vars)
    }

    fn save_env_vars(&self, project_id: &str, vars: &HashMap<String, String>) -> Result<(), DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        conn.execute("DELETE FROM env_vars WHERE project_id = ?1", [project_id]).map_err(Self::db_err)?;
        for (key, value) in vars {
            conn.execute(
                "INSERT INTO env_vars (project_id, key, value) VALUES (?1, ?2, ?3)",
                params![project_id, key, value],
            ).map_err(Self::db_err)?;
        }
        Ok(())
    }

    fn delete_env_vars(&self, project_id: &str) -> Result<(), DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        conn.execute("DELETE FROM env_vars WHERE project_id = ?1", [project_id]).map_err(Self::db_err)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// SnapshotStore — delega lectura de manifest al filesystem (es del proyecto)
// ---------------------------------------------------------------------------

impl SnapshotStore for SqliteStore {
    fn save_snapshot(&self, project_id: &str, project_path: &str) -> Result<Snapshot, DelixonError> {
        let manifest = crate::core::manifest::load_manifest(project_path)?
            .ok_or_else(|| DelixonError::InvalidManifest("No se encontro manifest para snapshot".to_string()))?;

        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;

        let next_version: u32 = conn
            .prepare("SELECT COALESCE(MAX(version), 0) + 1 FROM snapshots WHERE project_id = ?1")
            .map_err(Self::db_err)?
            .query_row([project_id], |row| row.get(0))
            .map_err(Self::db_err)?;

        let timestamp = chrono::Utc::now().to_rfc3339();
        let manifest_json = serde_json::to_string(&manifest)?;

        conn.execute(
            "INSERT INTO snapshots (project_id, version, timestamp, manifest) VALUES (?1, ?2, ?3, ?4)",
            params![project_id, next_version, timestamp, manifest_json],
        ).map_err(Self::db_err)?;

        Ok(Snapshot { version: next_version, timestamp, manifest })
    }

    fn list_snapshots(&self, project_id: &str) -> Result<Vec<Snapshot>, DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT version, timestamp, manifest FROM snapshots WHERE project_id = ?1 ORDER BY version"
        ).map_err(Self::db_err)?;

        let snapshots: Vec<Snapshot> = stmt.query_map([project_id], |row| {
            let manifest_json: String = row.get(2)?;
            let manifest: ProjectManifest = serde_json::from_str(&manifest_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(Snapshot {
                version: row.get(0)?,
                timestamp: row.get(1)?,
                manifest,
            })
        })
        .map_err(Self::db_err)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(Self::db_err)?;
        Ok(snapshots)
    }

    fn diff_snapshots(&self, project_id: &str, v1: u32, v2: u32) -> Result<SnapshotDiff, DelixonError> {
        let snapshots = self.list_snapshots(project_id)?;

        let s1 = snapshots.iter().find(|s| s.version == v1)
            .ok_or_else(|| DelixonError::InvalidManifest(format!("Snapshot v{} no encontrado", v1)))?;
        let s2 = snapshots.iter().find(|s| s.version == v2)
            .ok_or_else(|| DelixonError::InvalidManifest(format!("Snapshot v{} no encontrado", v2)))?;

        let added_techs: Vec<String> = s2.manifest.technologies.iter()
            .filter(|t| !s1.manifest.technologies.contains(t))
            .cloned().collect();
        let removed_techs: Vec<String> = s1.manifest.technologies.iter()
            .filter(|t| !s2.manifest.technologies.contains(t))
            .cloned().collect();
        let added_recipes: Vec<String> = s2.manifest.recipes_applied.iter()
            .filter(|r| !s1.manifest.recipes_applied.contains(r))
            .cloned().collect();

        let profile_changed = if s1.manifest.profile != s2.manifest.profile {
            Some((s1.manifest.profile.clone(), s2.manifest.profile.clone()))
        } else { None };

        let editor_changed = if s1.manifest.editor != s2.manifest.editor {
            Some((s1.manifest.editor.clone(), s2.manifest.editor.clone()))
        } else { None };

        Ok(SnapshotDiff {
            from_version: v1,
            to_version: v2,
            added_techs,
            removed_techs,
            added_recipes,
            profile_changed,
            editor_changed,
        })
    }

    fn rollback_snapshot(&self, _project_id: &str, project_path: &str, version: u32) -> Result<(), DelixonError> {
        let snapshots = self.list_snapshots(_project_id)?;
        let snapshot = snapshots.iter().find(|s| s.version == version)
            .ok_or_else(|| DelixonError::InvalidManifest(format!("Snapshot v{} no encontrado", version)))?;

        crate::core::manifest::save_manifest(project_path, &snapshot.manifest)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// EnvSnapshotStore — detección de runtimes sigue siendo de filesystem
// ---------------------------------------------------------------------------

impl EnvSnapshotStore for SqliteStore {
    fn take_env_snapshot(&self, project_id: &str, project_path: &str) -> Result<EnvSnapshot, DelixonError> {
        // Detectar runtimes y deps hash desde el filesystem (igual que JsonStore)
        let snapshot = crate::core::history::env::take_snapshot(project_id, project_path)?;

        // Guardar en SQLite
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        let runtimes_json = serde_json::to_string(&snapshot.runtimes)?;

        conn.execute(
            "INSERT OR REPLACE INTO env_snapshots (project_id, timestamp, runtimes, deps_hash) VALUES (?1, ?2, ?3, ?4)",
            params![project_id, snapshot.timestamp, runtimes_json, snapshot.deps_hash],
        ).map_err(Self::db_err)?;

        Ok(snapshot)
    }

    fn load_env_snapshot(&self, project_id: &str) -> Result<Option<EnvSnapshot>, DelixonError> {
        let conn = self.conn.lock().map_err(|e| DelixonError::Database(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT timestamp, runtimes, deps_hash FROM env_snapshots WHERE project_id = ?1"
        ).map_err(Self::db_err)?;

        let result = stmt.query_row([project_id], |row| {
            let runtimes_json: String = row.get(1)?;
            let runtimes: Vec<RuntimeSnapshot> = serde_json::from_str(&runtimes_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            Ok(EnvSnapshot {
                timestamp: row.get(0)?,
                runtimes,
                deps_hash: row.get(2)?,
            })
        });

        match result {
            Ok(snapshot) => Ok(Some(snapshot)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Self::db_err(e)),
        }
    }

    fn diff_env_snapshot(&self, project_id: &str, project_path: &str) -> Result<Option<EnvDiff>, DelixonError> {
        // Cargar snapshot anterior desde SQLite
        let prev = match self.load_env_snapshot(project_id)? {
            Some(s) => s,
            None => return Ok(None),
        };

        // Tomar snapshot actual desde filesystem (no lo guarda, solo compara)
        let current = crate::core::history::env::take_snapshot(project_id, project_path)?;

        let mut changed_runtimes = Vec::new();
        for old_rt in &prev.runtimes {
            if let Some(new_rt) = current.runtimes.iter().find(|r| r.name == old_rt.name) {
                if old_rt.version != new_rt.version {
                    changed_runtimes.push(crate::core::history::env::RuntimeChange {
                        name: old_rt.name.clone(),
                        old_version: old_rt.version.clone(),
                        new_version: new_rt.version.clone(),
                    });
                }
            }
        }

        let deps_changed = prev.deps_hash != current.deps_hash;

        if changed_runtimes.is_empty() && !deps_changed {
            return Ok(None);
        }

        Ok(Some(EnvDiff { changed_runtimes, deps_changed }))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_store() -> SqliteStore {
        SqliteStore::in_memory().unwrap()
    }

    fn make_project(suffix: &str) -> Project {
        Project {
            id: format!("test-sqlite-{}", suffix),
            name: format!("SQLite Test {}", suffix),
            path: format!("/tmp/sqlite-test-{}", suffix),
            description: Some("test".to_string()),
            runtimes: vec![RuntimeConfig { runtime: "node".to_string(), version: "20".to_string() }],
            status: ProjectStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            last_opened_at: None,
            template_id: None,
            tags: vec!["test".to_string(), "sqlite".to_string()],
        }
    }

    #[test]
    fn test_projects_crud() {
        let store = make_store();
        let proj = make_project("crud");

        // Save
        store.save_projects(&[proj.clone()]).unwrap();

        // List
        let projects = store.list_projects().unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, proj.id);
        assert_eq!(projects[0].name, proj.name);
        assert_eq!(projects[0].runtimes.len(), 1);
        assert_eq!(projects[0].tags, vec!["sqlite", "test"]); // sorted

        // Update (save all)
        let mut updated = proj.clone();
        updated.name = "Updated".to_string();
        store.save_projects(&[updated.clone()]).unwrap();
        let projects = store.list_projects().unwrap();
        assert_eq!(projects[0].name, "Updated");

        // Delete (save empty)
        store.save_projects(&[]).unwrap();
        let projects = store.list_projects().unwrap();
        assert!(projects.is_empty());
    }

    #[test]
    fn test_config_crud() {
        let store = make_store();

        let config = store.load_config().unwrap();
        assert_eq!(config.default_editor, "code");
        assert_eq!(config.theme, "dark");

        let mut updated = config.clone();
        updated.default_editor = "vim".to_string();
        updated.theme = "light".to_string();
        store.save_config(&updated).unwrap();

        let loaded = store.load_config().unwrap();
        assert_eq!(loaded.default_editor, "vim");
        assert_eq!(loaded.theme, "light");
    }

    #[test]
    fn test_notes_crud() {
        let store = make_store();
        let proj = make_project("notes");
        store.save_projects(&[proj.clone()]).unwrap();

        // Add
        let note = store.add_note(&proj.id, "Test note").unwrap();
        assert_eq!(note.text, "Test note");

        // Get
        let notes = store.get_notes(&proj.id).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].text, "Test note");

        // Delete
        store.delete_note(&proj.id, &note.id).unwrap();
        let notes = store.get_notes(&proj.id).unwrap();
        assert!(notes.is_empty());
    }

    #[test]
    fn test_env_vars_crud() {
        let store = make_store();
        let proj = make_project("env");
        store.save_projects(&[proj.clone()]).unwrap();

        let mut vars = HashMap::new();
        vars.insert("API_KEY".to_string(), "secret".to_string());
        vars.insert("DB_HOST".to_string(), "localhost".to_string());
        store.save_env_vars(&proj.id, &vars).unwrap();

        let loaded = store.load_env_vars(&proj.id).unwrap();
        assert_eq!(loaded.get("API_KEY").unwrap(), "secret");
        assert_eq!(loaded.get("DB_HOST").unwrap(), "localhost");

        store.delete_env_vars(&proj.id).unwrap();
        let loaded = store.load_env_vars(&proj.id).unwrap();
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_cascade_delete() {
        let store = make_store();
        let proj = make_project("cascade");
        store.save_projects(&[proj.clone()]).unwrap();

        // Add env vars and notes
        let mut vars = HashMap::new();
        vars.insert("KEY".to_string(), "val".to_string());
        store.save_env_vars(&proj.id, &vars).unwrap();
        store.add_note(&proj.id, "note").unwrap();

        // Delete project (save empty list)
        store.save_projects(&[]).unwrap();

        // env_vars and notes should be cascade-deleted
        // (env_vars won't cascade because we DELETE FROM projects, not the project table)
        // Actually CASCADE works on DELETE FROM projects
        let notes = store.get_notes(&proj.id).unwrap();
        assert!(notes.is_empty());
        let vars = store.load_env_vars(&proj.id).unwrap();
        assert!(vars.is_empty());
    }

    #[test]
    fn test_config_default_values() {
        let store = make_store();
        let config = store.load_config().unwrap();
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.default_editor, "code");
        assert_eq!(config.theme, "dark");
        assert_eq!(config.language, "es");
        assert!(config.auto_check_updates);
    }
}
