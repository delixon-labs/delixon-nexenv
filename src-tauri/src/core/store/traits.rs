use std::collections::HashMap;

use crate::core::error::NexenvError;
use crate::core::history::env::{EnvDiff, EnvSnapshot};
use crate::core::history::versioning::{Snapshot, SnapshotDiff};
use crate::core::models::project::Project;
use crate::core::project::config::NexenvConfig;
use crate::core::project::notes::ProjectNote;

pub trait ProjectStore: Send + Sync {
    fn list_projects(&self) -> Result<Vec<Project>, NexenvError>;
    fn save_projects(&self, projects: &[Project]) -> Result<(), NexenvError>;
}

pub trait ConfigStore: Send + Sync {
    fn load_config(&self) -> Result<NexenvConfig, NexenvError>;
    fn save_config(&self, config: &NexenvConfig) -> Result<(), NexenvError>;
}

pub trait NoteStore: Send + Sync {
    fn get_notes(&self, project_id: &str) -> Result<Vec<ProjectNote>, NexenvError>;
    fn add_note(&self, project_id: &str, text: &str) -> Result<ProjectNote, NexenvError>;
    fn delete_note(&self, project_id: &str, note_id: &str) -> Result<(), NexenvError>;
}

pub trait EnvVarStore: Send + Sync {
    fn load_env_vars(&self, project_id: &str) -> Result<HashMap<String, String>, NexenvError>;
    fn save_env_vars(
        &self,
        project_id: &str,
        vars: &HashMap<String, String>,
    ) -> Result<(), NexenvError>;
    fn delete_env_vars(&self, project_id: &str) -> Result<(), NexenvError>;
}

pub trait SnapshotStore: Send + Sync {
    fn save_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<Snapshot, NexenvError>;
    fn list_snapshots(&self, project_id: &str) -> Result<Vec<Snapshot>, NexenvError>;
    fn diff_snapshots(
        &self,
        project_id: &str,
        v1: u32,
        v2: u32,
    ) -> Result<SnapshotDiff, NexenvError>;
    fn rollback_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
        version: u32,
    ) -> Result<(), NexenvError>;
}

pub trait EnvSnapshotStore: Send + Sync {
    fn take_env_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<EnvSnapshot, NexenvError>;
    fn load_env_snapshot(&self, project_id: &str) -> Result<Option<EnvSnapshot>, NexenvError>;
    fn diff_env_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<Option<EnvDiff>, NexenvError>;
}

/// Super-trait que combina todos los stores
pub trait Store:
    ProjectStore + ConfigStore + NoteStore + EnvVarStore + SnapshotStore + EnvSnapshotStore
{
}

impl<T> Store for T where
    T: ProjectStore + ConfigStore + NoteStore + EnvVarStore + SnapshotStore + EnvSnapshotStore
{
}
