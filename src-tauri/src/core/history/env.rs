use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

use crate::core::error::DelixonError;
use crate::core::utils::fs::{ensure_dir, write_private};
use crate::core::utils::platform::get_data_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnvSnapshot {
    pub timestamp: String,
    pub runtimes: Vec<RuntimeSnapshot>,
    pub deps_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeSnapshot {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnvDiff {
    pub changed_runtimes: Vec<RuntimeChange>,
    pub deps_changed: bool,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeChange {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
}

fn snapshot_file(project_id: &str) -> Result<PathBuf, DelixonError> {
    let base = get_data_dir().ok_or_else(|| {
        DelixonError::InvalidConfig("No se pudo determinar directorio de datos".to_string())
    })?;
    let dir = base.join("env_snapshots");
    ensure_dir(&dir)?;
    Ok(dir.join(format!("{}.json", project_id)))
}

pub fn take_snapshot(project_id: &str, project_path: &str) -> Result<EnvSnapshot, DelixonError> {
    let runtimes = detect_runtime_versions();
    let deps_hash = compute_deps_hash(project_path);

    let snapshot = EnvSnapshot {
        timestamp: chrono::Utc::now().to_rfc3339(),
        runtimes,
        deps_hash,
    };

    let file = snapshot_file(project_id)?;
    let data = serde_json::to_string_pretty(&snapshot)?;
    write_private(&file, &data)?;

    Ok(snapshot)
}

pub fn load_snapshot(project_id: &str) -> Result<Option<EnvSnapshot>, DelixonError> {
    let file = snapshot_file(project_id)?;
    if !file.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&file)?;
    let snapshot: EnvSnapshot = serde_json::from_str(&data)?;
    Ok(Some(snapshot))
}

pub fn diff_snapshot(project_id: &str, project_path: &str) -> Result<Option<EnvDiff>, DelixonError> {
    let prev = match load_snapshot(project_id)? {
        Some(s) => s,
        None => return Ok(None),
    };

    let current_runtimes = detect_runtime_versions();
    let current_deps_hash = compute_deps_hash(project_path);

    let mut changed_runtimes = Vec::new();
    for current in &current_runtimes {
        if let Some(old) = prev.runtimes.iter().find(|r| r.name == current.name) {
            if old.version != current.version {
                changed_runtimes.push(RuntimeChange {
                    name: current.name.clone(),
                    old_version: old.version.clone(),
                    new_version: current.version.clone(),
                });
            }
        }
    }

    let deps_changed = prev.deps_hash != current_deps_hash;

    Ok(Some(EnvDiff {
        changed_runtimes,
        deps_changed,
    }))
}

fn detect_runtime_versions() -> Vec<RuntimeSnapshot> {
    let checks = [
        ("node", &["--version"] as &[&str]),
        ("python", &["--version"]),
        ("rustc", &["--version"]),
        ("go", &["version"]),
    ];

    let mut runtimes = Vec::new();
    for (cmd, args) in &checks {
        if let Ok(output) = Command::new(cmd).args(*args).output() {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                runtimes.push(RuntimeSnapshot {
                    name: cmd.to_string(),
                    version,
                });
            }
        }
    }
    runtimes
}

fn compute_deps_hash(project_path: &str) -> String {
    use std::path::Path;
    let lock_files = [
        "package-lock.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        "Cargo.lock",
        "go.sum",
        "poetry.lock",
        "Pipfile.lock",
    ];

    let path = Path::new(project_path);
    let mut combined = String::new();
    for lock in &lock_files {
        let full = path.join(lock);
        if full.exists() {
            if let Ok(meta) = std::fs::metadata(&full) {
                combined.push_str(&format!("{}:{}", lock, meta.len()));
            }
        }
    }

    if combined.is_empty() {
        return "none".to_string();
    }

    // Simple hash: use length + first chars
    format!("h{:x}", combined.len() * 31 + combined.chars().take(20).map(|c| c as usize).sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_and_load_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let project_path = dir.path().to_str().unwrap();

        let snap = take_snapshot("test-env-snap", project_path).unwrap();
        assert!(!snap.timestamp.is_empty());

        let loaded = load_snapshot("test-env-snap").unwrap();
        assert!(loaded.is_some());

        // Cleanup
        let _ = std::fs::remove_file(snapshot_file("test-env-snap").unwrap());
    }

    #[test]
    fn test_load_snapshot_nonexistent() {
        let loaded = load_snapshot("nonexistent-env-snap-xyz").unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_compute_deps_hash_empty() {
        let dir = tempfile::tempdir().unwrap();
        let hash = compute_deps_hash(dir.path().to_str().unwrap());
        assert_eq!(hash, "none");
    }
}
