use serde::Serialize;
use std::path::Path;
use std::process::Command;

use crate::core::error::DelixonError;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DockerComposeStatus {
    pub has_compose: bool,
    pub services: Vec<DockerService>,
    pub compose_file: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DockerService {
    pub name: String,
    pub status: String,
    pub ports: String,
}

pub fn compose_status(project_path: &str) -> Result<DockerComposeStatus, DelixonError> {
    let path = Path::new(project_path);
    let compose_file = find_compose_file(path);

    if compose_file.is_empty() {
        return Ok(DockerComposeStatus {
            has_compose: false,
            services: Vec::new(),
            compose_file: String::new(),
        });
    }

    let services = parse_running_services(project_path);

    Ok(DockerComposeStatus {
        has_compose: true,
        services,
        compose_file,
    })
}

pub fn compose_up(project_path: &str) -> Result<String, DelixonError> {
    run_compose(project_path, &["up", "-d"])
}

pub fn compose_down(project_path: &str) -> Result<String, DelixonError> {
    run_compose(project_path, &["down"])
}

pub fn compose_logs(project_path: &str, lines: u32) -> Result<String, DelixonError> {
    run_compose(project_path, &["logs", "--tail", &lines.to_string()])
}

fn find_compose_file(path: &Path) -> String {
    let candidates = [
        "docker-compose.yml",
        "docker-compose.yaml",
        "compose.yml",
        "compose.yaml",
    ];
    for name in &candidates {
        if path.join(name).exists() {
            return name.to_string();
        }
    }
    String::new()
}

fn parse_running_services(project_path: &str) -> Vec<DockerService> {
    let output = Command::new("docker")
        .args(["compose", "ps", "--format", "{{.Name}}\t{{.Status}}\t{{.Ports}}"])
        .current_dir(project_path)
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            text.lines()
                .filter(|l| !l.is_empty())
                .map(|line| {
                    let parts: Vec<&str> = line.splitn(3, '\t').collect();
                    DockerService {
                        name: parts.first().unwrap_or(&"").to_string(),
                        status: parts.get(1).unwrap_or(&"").to_string(),
                        ports: parts.get(2).unwrap_or(&"").to_string(),
                    }
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn run_compose(project_path: &str, args: &[&str]) -> Result<String, DelixonError> {
    let output = Command::new("docker")
        .arg("compose")
        .args(args)
        .current_dir(project_path)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(format!("{}{}", stdout, stderr))
    } else {
        Err(DelixonError::InvalidConfig(format!(
            "docker compose error: {}",
            stderr
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_status_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let status = compose_status(dir.path().to_str().unwrap()).unwrap();
        assert!(!status.has_compose);
        assert!(status.services.is_empty());
    }

    #[test]
    fn test_compose_status_with_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("docker-compose.yml"), "services:\n  db:\n    image: postgres:16\n").unwrap();
        let status = compose_status(dir.path().to_str().unwrap()).unwrap();
        assert!(status.has_compose);
        assert_eq!(status.compose_file, "docker-compose.yml");
    }

    #[test]
    fn test_find_compose_file() {
        let dir = tempfile::tempdir().unwrap();
        assert!(find_compose_file(dir.path()).is_empty());

        std::fs::write(dir.path().join("compose.yaml"), "services:").unwrap();
        assert_eq!(find_compose_file(dir.path()), "compose.yaml");
    }
}
