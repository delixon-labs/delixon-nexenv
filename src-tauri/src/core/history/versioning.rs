use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::core::error::NexenvError;
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
    pub profile_changed: Option<(String, String)>,
    pub editor_changed: Option<(Option<String>, Option<String>)>,
}

fn snapshots_dir(project_id: &str) -> Result<PathBuf, NexenvError> {
    let base = get_data_dir().ok_or_else(|| {
        NexenvError::InvalidConfig("No se pudo determinar directorio de datos".to_string())
    })?;
    let dir = base.join("snapshots").join(project_id);
    ensure_dir(&dir)?;
    Ok(dir)
}

pub fn save_snapshot(project_id: &str, project_path: &str) -> Result<Snapshot, NexenvError> {
    let m = manifest::load_manifest(project_path)?
        .ok_or_else(|| NexenvError::InvalidConfig("No hay manifest".to_string()))?;

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

pub fn list_snapshots(project_id: &str) -> Result<Vec<Snapshot>, NexenvError> {
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

pub fn diff_snapshots(project_id: &str, v1: u32, v2: u32) -> Result<SnapshotDiff, NexenvError> {
    let snapshots = list_snapshots(project_id)?;
    let s1 = snapshots.iter().find(|s| s.version == v1).ok_or_else(|| {
        NexenvError::InvalidConfig(format!("Snapshot v{} no encontrado", v1))
    })?;
    let s2 = snapshots.iter().find(|s| s.version == v2).ok_or_else(|| {
        NexenvError::InvalidConfig(format!("Snapshot v{} no encontrado", v2))
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

    let profile_changed = if s1.manifest.profile != s2.manifest.profile {
        Some((s1.manifest.profile.clone(), s2.manifest.profile.clone()))
    } else {
        None
    };

    let editor_changed = if s1.manifest.editor != s2.manifest.editor {
        Some((s1.manifest.editor.clone(), s2.manifest.editor.clone()))
    } else {
        None
    };

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

pub fn rollback_snapshot(project_id: &str, project_path: &str, version: u32) -> Result<(), NexenvError> {
    let snapshots = list_snapshots(project_id)?;
    let snapshot = snapshots.iter().find(|s| s.version == version).ok_or_else(|| {
        NexenvError::InvalidConfig(format!("Snapshot v{} no encontrado", version))
    })?;

    manifest::save_manifest(project_path, &snapshot.manifest)?;
    Ok(())
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RollbackPreview {
    pub target_version: u32,
    pub target_timestamp: String,
    pub current_manifest_exists: bool,
    pub added_techs: Vec<String>,
    pub removed_techs: Vec<String>,
    pub added_recipes: Vec<String>,
    pub removed_recipes: Vec<String>,
    pub profile_changed: Option<(String, String)>,
    pub editor_changed: Option<(Option<String>, Option<String>)>,
    pub name_changed: Option<(String, String)>,
    pub runtime_changed: Option<(String, String)>,
}

/// Calcula el cambio que aplicaria un rollback al snapshot `version`,
/// sin tocar el manifest. El frontend lo muestra al usuario para confirmar.
pub fn preview_rollback(
    project_id: &str,
    project_path: &str,
    version: u32,
) -> Result<RollbackPreview, NexenvError> {
    let snapshots = list_snapshots(project_id)?;
    let target = snapshots.iter().find(|s| s.version == version).ok_or_else(|| {
        NexenvError::InvalidConfig(format!("Snapshot v{} no encontrado", version))
    })?;

    let current = manifest::load_manifest(project_path).unwrap_or(None);

    let (added_techs, removed_techs, added_recipes, removed_recipes,
         profile_changed, editor_changed, name_changed, runtime_changed) = match &current {
        Some(cur) => {
            let added_techs: Vec<String> = target.manifest.technologies.iter()
                .filter(|t| !cur.technologies.contains(t)).cloned().collect();
            let removed_techs: Vec<String> = cur.technologies.iter()
                .filter(|t| !target.manifest.technologies.contains(t)).cloned().collect();
            let added_recipes: Vec<String> = target.manifest.recipes_applied.iter()
                .filter(|r| !cur.recipes_applied.contains(r)).cloned().collect();
            let removed_recipes: Vec<String> = cur.recipes_applied.iter()
                .filter(|r| !target.manifest.recipes_applied.contains(r)).cloned().collect();
            let profile = if cur.profile != target.manifest.profile {
                Some((cur.profile.clone(), target.manifest.profile.clone()))
            } else { None };
            let editor = if cur.editor != target.manifest.editor {
                Some((cur.editor.clone(), target.manifest.editor.clone()))
            } else { None };
            let name = if cur.name != target.manifest.name {
                Some((cur.name.clone(), target.manifest.name.clone()))
            } else { None };
            let runtime = if cur.runtime != target.manifest.runtime {
                Some((cur.runtime.clone(), target.manifest.runtime.clone()))
            } else { None };
            (added_techs, removed_techs, added_recipes, removed_recipes,
             profile, editor, name, runtime)
        }
        None => (
            target.manifest.technologies.clone(),
            Vec::new(),
            target.manifest.recipes_applied.clone(),
            Vec::new(),
            None, None, None, None,
        ),
    };

    Ok(RollbackPreview {
        target_version: target.version,
        target_timestamp: target.timestamp.clone(),
        current_manifest_exists: current.is_some(),
        added_techs, removed_techs, added_recipes, removed_recipes,
        profile_changed, editor_changed, name_changed, runtime_changed,
    })
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

    #[test]
    fn test_diff_snapshots() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();
        let pid = "test-snap-diff";

        // v1: nodejs only
        let m1 = ProjectManifest {
            name: "test".to_string(),
            technologies: vec!["nodejs".to_string()],
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m1).unwrap();
        save_snapshot(pid, project_path).unwrap();

        // v2: nodejs + react + recipe applied
        let m2 = ProjectManifest {
            name: "test".to_string(),
            technologies: vec!["nodejs".to_string(), "react".to_string()],
            recipes_applied: vec!["testing-vitest".to_string()],
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m2).unwrap();
        save_snapshot(pid, project_path).unwrap();

        let diff = diff_snapshots(pid, 1, 2).unwrap();
        assert!(diff.added_techs.contains(&"react".to_string()));
        assert!(diff.removed_techs.is_empty());
        assert!(diff.added_recipes.contains(&"testing-vitest".to_string()));

        let _ = std::fs::remove_dir_all(snapshots_dir(pid).unwrap());
    }

    #[test]
    fn test_rollback_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();
        let pid = "test-snap-rollback";

        // Save v1 with nodejs
        let m1 = ProjectManifest {
            name: "original".to_string(),
            runtime: "node".to_string(),
            technologies: vec!["nodejs".to_string()],
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m1).unwrap();
        save_snapshot(pid, project_path).unwrap();

        // Change manifest
        let m2 = ProjectManifest {
            name: "changed".to_string(),
            runtime: "python".to_string(),
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m2).unwrap();

        // Rollback to v1
        rollback_snapshot(pid, project_path, 1).unwrap();
        let restored = manifest::load_manifest(project_path).unwrap().unwrap();
        assert_eq!(restored.name, "original");
        assert_eq!(restored.runtime, "node");

        let _ = std::fs::remove_dir_all(snapshots_dir(pid).unwrap());
    }

    #[test]
    fn test_preview_rollback_shows_diff_against_current() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();
        let pid = "test-snap-preview";

        let m1 = ProjectManifest {
            name: "v1".to_string(),
            runtime: "node".to_string(),
            technologies: vec!["nodejs".to_string()],
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m1).unwrap();
        save_snapshot(pid, project_path).unwrap();

        let m2 = ProjectManifest {
            name: "v2".to_string(),
            runtime: "python".to_string(),
            technologies: vec!["python".to_string(), "fastapi".to_string()],
            recipes_applied: vec!["docker".to_string()],
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m2).unwrap();

        let preview = preview_rollback(pid, project_path, 1).unwrap();
        assert_eq!(preview.target_version, 1);
        assert!(preview.current_manifest_exists);
        assert!(preview.added_techs.contains(&"nodejs".to_string()));
        assert!(preview.removed_techs.contains(&"python".to_string()));
        assert!(preview.removed_recipes.contains(&"docker".to_string()));
        assert_eq!(preview.name_changed, Some(("v2".into(), "v1".into())));
        assert_eq!(preview.runtime_changed, Some(("python".into(), "node".into())));

        let _ = std::fs::remove_dir_all(snapshots_dir(pid).unwrap());
    }

    #[test]
    fn test_preview_rollback_missing_snapshot_errors() {
        let dir = tempfile::tempdir().unwrap();
        let pid = "test-snap-preview-missing";
        let result = preview_rollback(pid, dir.path().to_str().unwrap(), 99);
        assert!(result.is_err());
        let _ = std::fs::remove_dir_all(snapshots_dir(pid).unwrap());
    }

    #[test]
    fn test_snapshot_version_increments() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();
        let pid = "test-snap-incr";

        let m = ProjectManifest {
            name: "test".to_string(),
            ..Default::default()
        };
        manifest::save_manifest(project_path, &m).unwrap();

        let s1 = save_snapshot(pid, project_path).unwrap();
        let s2 = save_snapshot(pid, project_path).unwrap();
        let s3 = save_snapshot(pid, project_path).unwrap();

        assert_eq!(s1.version, 1);
        assert_eq!(s2.version, 2);
        assert_eq!(s3.version, 3);

        let list = list_snapshots(pid).unwrap();
        assert_eq!(list.len(), 3);

        let _ = std::fs::remove_dir_all(snapshots_dir(pid).unwrap());
    }
}
