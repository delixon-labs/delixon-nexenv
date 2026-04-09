use serde::Serialize;
use std::path::Path;
use std::process::Command;

use crate::core::error::NexenvError;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitStatus {
    pub branch: String,
    pub is_clean: bool,
    pub modified_files: u32,
    pub untracked_files: u32,
    pub ahead: u32,
    pub behind: u32,
    pub has_remote: bool,
    pub last_commit: Option<GitCommit>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitCommit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

pub fn git_status(project_path: &str) -> Result<GitStatus, NexenvError> {
    let path = Path::new(project_path);
    if !path.join(".git").exists() {
        return Err(NexenvError::InvalidConfig(
            "No es un repositorio Git".to_string(),
        ));
    }

    let branch = run_git(project_path, &["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_else(|_| "unknown".to_string());

    let status_output = run_git(project_path, &["status", "--porcelain"])?;
    let modified_files = status_output
        .lines()
        .filter(|l| !l.starts_with("??"))
        .count() as u32;
    let untracked_files = status_output
        .lines()
        .filter(|l| l.starts_with("??"))
        .count() as u32;
    let is_clean = status_output.trim().is_empty();

    let has_remote = run_git(project_path, &["remote"]).map(|r| !r.trim().is_empty()).unwrap_or(false);

    let (ahead, behind) = if has_remote {
        let ab = run_git(
            project_path,
            &["rev-list", "--left-right", "--count", &format!("{}...@{{u}}", branch)],
        )
        .unwrap_or_default();
        let parts: Vec<&str> = ab.trim().split('\t').collect();
        if parts.len() == 2 {
            (
                parts[0].parse().unwrap_or(0),
                parts[1].parse().unwrap_or(0),
            )
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };

    let last_commit = git_last_commit(project_path).ok();

    Ok(GitStatus {
        branch,
        is_clean,
        modified_files,
        untracked_files,
        ahead,
        behind,
        has_remote,
        last_commit,
    })
}

pub fn git_log(project_path: &str, count: u32) -> Result<Vec<GitCommit>, NexenvError> {
    let output = run_git(
        project_path,
        &[
            "log",
            &format!("-{}", count),
            "--format=%H%n%s%n%an%n%ai",
            "--no-color",
        ],
    )?;

    let lines: Vec<&str> = output.lines().collect();
    let mut commits = Vec::new();

    for chunk in lines.chunks(4) {
        if chunk.len() >= 4 {
            commits.push(GitCommit {
                hash: chunk[0].to_string(),
                message: chunk[1].to_string(),
                author: chunk[2].to_string(),
                date: chunk[3].to_string(),
            });
        }
    }

    Ok(commits)
}

fn git_last_commit(project_path: &str) -> Result<GitCommit, NexenvError> {
    let commits = git_log(project_path, 1)?;
    commits.into_iter().next().ok_or_else(|| {
        NexenvError::InvalidConfig("No hay commits".to_string())
    })
}

fn run_git(project_path: &str, args: &[&str]) -> Result<String, NexenvError> {
    let output = Command::new("git")
        .args(args)
        .current_dir(project_path)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(NexenvError::InvalidConfig(format!("Git error: {}", err)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_status_no_git() {
        let dir = tempfile::tempdir().unwrap();
        let result = git_status(dir.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_git_status_with_repo() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        Command::new("git").args(["init"]).current_dir(path).output().unwrap();
        Command::new("git").args(["config", "user.email", "test@test.com"]).current_dir(path).output().unwrap();
        Command::new("git").args(["config", "user.name", "Test"]).current_dir(path).output().unwrap();
        std::fs::write(dir.path().join("test.txt"), "hello").unwrap();
        Command::new("git").args(["add", "."]).current_dir(path).output().unwrap();
        Command::new("git").args(["commit", "-m", "init"]).current_dir(path).output().unwrap();

        let status = git_status(path).unwrap();
        assert!(!status.branch.is_empty());
        assert!(status.is_clean);
        assert!(status.last_commit.is_some());
    }

    #[test]
    fn test_git_log() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        Command::new("git").args(["init"]).current_dir(path).output().unwrap();
        Command::new("git").args(["config", "user.email", "test@test.com"]).current_dir(path).output().unwrap();
        Command::new("git").args(["config", "user.name", "Test"]).current_dir(path).output().unwrap();
        std::fs::write(dir.path().join("a.txt"), "a").unwrap();
        Command::new("git").args(["add", "."]).current_dir(path).output().unwrap();
        Command::new("git").args(["commit", "-m", "first"]).current_dir(path).output().unwrap();
        std::fs::write(dir.path().join("b.txt"), "b").unwrap();
        Command::new("git").args(["add", "."]).current_dir(path).output().unwrap();
        Command::new("git").args(["commit", "-m", "second"]).current_dir(path).output().unwrap();

        let commits = git_log(path, 5).unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].message, "second");
    }
}
