use std::collections::HashMap;

use crate::core::error::NexenvError;
use crate::core::history::env::{self as env_snapshots, EnvDiff, EnvSnapshot};
use crate::core::history::versioning::{self, Snapshot, SnapshotDiff};
use crate::core::models::project::Project;
use crate::core::project::config::{self, NexenvConfig};
use crate::core::project::notes::{self, ProjectNote};
use crate::core::project::storage;
use crate::core::store::traits::*;

/// Backend JSON que delega a las funciones existentes de storage/config/notes/etc.
pub struct JsonStore;

impl Default for JsonStore {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonStore {
    pub fn new() -> Self {
        let _ = storage::init_data_dir();
        JsonStore
    }
}

impl ProjectStore for JsonStore {
    fn list_projects(&self) -> Result<Vec<Project>, NexenvError> {
        storage::load_projects()
    }
    fn save_projects(&self, projects: &[Project]) -> Result<(), NexenvError> {
        storage::save_projects(projects)
    }
}

impl ConfigStore for JsonStore {
    fn load_config(&self) -> Result<NexenvConfig, NexenvError> {
        config::load_config()
    }
    fn save_config(&self, cfg: &NexenvConfig) -> Result<(), NexenvError> {
        config::save_config(cfg)
    }
}

impl NoteStore for JsonStore {
    fn get_notes(&self, project_id: &str) -> Result<Vec<ProjectNote>, NexenvError> {
        notes::get_notes(project_id)
    }
    fn add_note(&self, project_id: &str, text: &str) -> Result<ProjectNote, NexenvError> {
        notes::add_note(project_id, text)
    }
    fn delete_note(&self, project_id: &str, note_id: &str) -> Result<(), NexenvError> {
        notes::delete_note(project_id, note_id)
    }
}

impl EnvVarStore for JsonStore {
    fn load_env_vars(&self, project_id: &str) -> Result<HashMap<String, String>, NexenvError> {
        storage::load_env_vars(project_id)
    }
    fn save_env_vars(
        &self,
        project_id: &str,
        vars: &HashMap<String, String>,
    ) -> Result<(), NexenvError> {
        storage::save_env_vars(project_id, vars)
    }
    fn delete_env_vars(&self, project_id: &str) -> Result<(), NexenvError> {
        storage::delete_env_vars(project_id)
    }
}

impl SnapshotStore for JsonStore {
    fn save_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<Snapshot, NexenvError> {
        versioning::save_snapshot(project_id, project_path)
    }
    fn list_snapshots(&self, project_id: &str) -> Result<Vec<Snapshot>, NexenvError> {
        versioning::list_snapshots(project_id)
    }
    fn diff_snapshots(
        &self,
        project_id: &str,
        v1: u32,
        v2: u32,
    ) -> Result<SnapshotDiff, NexenvError> {
        versioning::diff_snapshots(project_id, v1, v2)
    }
    fn rollback_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
        version: u32,
    ) -> Result<(), NexenvError> {
        versioning::rollback_snapshot(project_id, project_path, version)
    }
}

impl EnvSnapshotStore for JsonStore {
    fn take_env_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<EnvSnapshot, NexenvError> {
        env_snapshots::take_snapshot(project_id, project_path)
    }
    fn load_env_snapshot(&self, project_id: &str) -> Result<Option<EnvSnapshot>, NexenvError> {
        env_snapshots::load_snapshot(project_id)
    }
    fn diff_env_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<Option<EnvDiff>, NexenvError> {
        env_snapshots::diff_snapshot(project_id, project_path)
    }
}
