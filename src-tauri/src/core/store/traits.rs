use std::collections::HashMap;

use crate::core::error::DelixonError;
use crate::core::history::env::{EnvDiff, EnvSnapshot};
use crate::core::history::versioning::{Snapshot, SnapshotDiff};
use crate::core::models::project::Project;
use crate::core::project::config::DelixonConfig;
use crate::core::project::notes::ProjectNote;

pub trait ProjectStore: Send + Sync {
    fn list_projects(&self) -> Result<Vec<Project>, DelixonError>;
    fn save_projects(&self, projects: &[Project]) -> Result<(), DelixonError>;
}

pub trait ConfigStore: Send + Sync {
    fn load_config(&self) -> Result<DelixonConfig, DelixonError>;
    fn save_config(&self, config: &DelixonConfig) -> Result<(), DelixonError>;
}

pub trait NoteStore: Send + Sync {
    fn get_notes(&self, project_id: &str) -> Result<Vec<ProjectNote>, DelixonError>;
    fn add_note(&self, project_id: &str, text: &str) -> Result<ProjectNote, DelixonError>;
    fn delete_note(&self, project_id: &str, note_id: &str) -> Result<(), DelixonError>;
}

pub trait EnvVarStore: Send + Sync {
    fn load_env_vars(&self, project_id: &str) -> Result<HashMap<String, String>, DelixonError>;
    fn save_env_vars(
        &self,
        project_id: &str,
        vars: &HashMap<String, String>,
    ) -> Result<(), DelixonError>;
    fn delete_env_vars(&self, project_id: &str) -> Result<(), DelixonError>;
}

pub trait SnapshotStore: Send + Sync {
    fn save_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<Snapshot, DelixonError>;
    fn list_snapshots(&self, project_id: &str) -> Result<Vec<Snapshot>, DelixonError>;
    fn diff_snapshots(
        &self,
        project_id: &str,
        v1: u32,
        v2: u32,
    ) -> Result<SnapshotDiff, DelixonError>;
    fn rollback_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
        version: u32,
    ) -> Result<(), DelixonError>;
}

pub trait EnvSnapshotStore: Send + Sync {
    fn take_env_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<EnvSnapshot, DelixonError>;
    fn load_env_snapshot(&self, project_id: &str) -> Result<Option<EnvSnapshot>, DelixonError>;
    fn diff_env_snapshot(
        &self,
        project_id: &str,
        project_path: &str,
    ) -> Result<Option<EnvDiff>, DelixonError>;
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
