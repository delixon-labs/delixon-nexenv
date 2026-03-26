use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::core::error::DelixonError;
use crate::core::manifest::{self, ProjectManifest};
use crate::core::utils::fs::{ensure_dir, write_private};
use crate::core::utils::platform::get_data_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub version: u32,
    pub timestamp: String,
    pub manifest: ProjectManifest,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDiff {
    pub from_version: u32,
    pub to_version: u32,
    pub added_techs: Vec<String>,
    pub removed_techs: Vec<String>,
    pub added_recipes: Vec<String>,
}

fn snapshots_dir(project_id: &str) -> Result<PathBuf, DelixonError> {
    let base = get_data_dir().ok_or_else(|| {
        DelixonError::InvalidConfig("No se pudo determinar directorio de datos".to_string())
    })?;
    let dir = base.join("snapshots").join(project_id);
    ensure_dir(&dir)?;
    Ok(dir)
}

pub fn save_snapshot(project_id: &str, project_path: &str) -> Result<Snapshot, DelixonError> {
    let m = manifest::load_manifest(project_path)?
        .ok_or_else(|| DelixonError::InvalidConfig("No hay manifest".to_string()))?;

    let dir = snapshots_dir(project_id)?;
    let existing = list_snapshots(project_id)?;
    let next_version = existing.last().map(|s| s.version + 1).unwrap_or(1);

    let snapshot = Snapshot {
        version: next_version,
        timestamp: chrono::Utc::now().to_rfc3339(),
        manifest: m,
    };

    let file = dir.join(format!("v{}.json", next_version));
    let data = serde_json::to_string_pretty(&snapshot)?;
    write_private(&file, &data)?;

    Ok(snapshot)
}

pub fn list_snapshots(project_id: &str) -> Result<Vec<Snapshot>, DelixonError> {
    let dir = snapshots_dir(project_id)?;
    let mut snapshots = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(data) = std::fs::read_to_string(&path) {
                    if let Ok(snapshot) = serde_json::from_str::<Snapshot>(&data) {
                        snapshots.push(snapshot);
                    }
                }
            }
        }
    }

    snapshots.sort_by_key(|s| s.version);
    Ok(snapshots)
}

pub fn diff_snapshots(project_id: &str, v1: u32, v2: u32) -> Result<SnapshotDiff, DelixonError> {
    let snapshots = list_snapshots(project_id)?;
    let s1 = snapshots.iter().find(|s| s.version == v1).ok_or_else(|| {
        DelixonError::InvalidConfig(format!("Snapshot v{} no encontrado", v1))
    })?;
    let s2 = snapshots.iter().find(|s| s.version == v2).ok_or_else(|| {
        DelixonError::InvalidConfig(format!("Snapshot v{} no encontrado", v2))
    })?;

    let added_techs: Vec<String> = s2
        .manifest
        .technologies
        .iter()
        .filter(|t| !s1.manifest.technologies.contains(t))
        .cloned()
        .collect();

    let removed_techs: Vec<String> = s1
        .manifest
        .technologies
        .iter()
        .filter(|t| !s2.manifest.technologies.contains(t))
        .cloned()
        .collect();

    let added_recipes: Vec<String> = s2
        .manifest
        .recipes_applied
        .iter()
        .filter(|r| !s1.manifest.recipes_applied.contains(r))
        .cloned()
        .collect();

    Ok(SnapshotDiff {
        from_version: v1,
        to_version: v2,
        added_techs,
        removed_techs,
        added_recipes,
    })
}

pub fn rollback_snapshot(project_id: &str, project_path: &str, version: u32) -> Result<(), DelixonError> {
    let snapshots = list_snapshots(project_id)?;
    let snapshot = snapshots.iter().find(|s| s.version == version).ok_or_else(|| {
        DelixonError::InvalidConfig(format!("Snapshot v{} no encontrado", version))
    })?;

    manifest::save_manifest(project_path, &snapshot.manifest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::manifest::ProjectManifest;

    #[test]
    fn test_save_and_list_snapshots() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();

        let m = ProjectManifest {
            name: "test".to_string(),
            runtime: "node".to_string(),
            technologies: vec!["nodejs".to_string()],
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m).unwrap();

        let snap = save_snapshot("test-snap-1", project_path).unwrap();
        assert_eq!(snap.version, 1);

        let list = list_snapshots("test-snap-1").unwrap();
        assert_eq!(list.len(), 1);

        // Cleanup
        let _ = std::fs::remove_dir_all(snapshots_dir("test-snap-1").unwrap());
    }

    #[test]
    fn test_list_snapshots_empty() {
        let list = list_snapshots("nonexistent-snap-project").unwrap();
        assert!(list.is_empty());
        let _ = std::fs::remove_dir_all(snapshots_dir("nonexistent-snap-project").unwrap());
    }
}
