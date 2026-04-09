use serde::Serialize;
use std::process::Command;

use crate::core::error::NexenvError;
use crate::core::manifest;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectProcess {
    pub pid: u32,
    pub name: String,
    pub port: Option<u16>,
}

pub fn list_processes_on_ports(project_path: &str) -> Result<Vec<ProjectProcess>, NexenvError> {
    let manifest = manifest::load_manifest(project_path)?;
    let ports = match manifest {
        Some(m) => m.ports,
        None => return Ok(Vec::new()),
    };

    let mut processes = Vec::new();
    for port in &ports {
        if let Some(proc) = find_process_on_port(*port) {
            processes.push(proc);
        }
    }
    Ok(processes)
}

pub fn kill_process(pid: u32) -> Result<(), NexenvError> {
    #[cfg(unix)]
    {
        Command::new("kill")
            .arg(pid.to_string())
            .output()?;
    }
    #[cfg(windows)]
    {
        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output()?;
    }
    Ok(())
}

fn find_process_on_port(port: u16) -> Option<ProjectProcess> {
    #[cfg(unix)]
    {
        let output = Command::new("lsof")
            .args(["-i", &format!(":{}", port), "-t", "-sTCP:LISTEN"])
            .output()
            .ok()?;

        if output.status.success() {
            let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let pid: u32 = pid_str.lines().next()?.parse().ok()?;

            let name = Command::new("ps")
                .args(["-p", &pid.to_string(), "-o", "comm="])
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "unknown".to_string());

            return Some(ProjectProcess {
                pid,
                name,
                port: Some(port),
            });
        }
    }
    #[cfg(windows)]
    {
        let output = Command::new("netstat")
            .args(["-ano", "-p", "TCP"])
            .output()
            .ok()?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.contains(&format!(":{}", port)) && line.contains("LISTENING") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(pid_str) = parts.last() {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            return Some(ProjectProcess {
                                pid,
                                name: "process".to_string(),
                                port: Some(port),
                            });
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_processes_no_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let result = list_processes_on_ports(dir.path().to_str().unwrap()).unwrap();
        assert!(result.is_empty());
    }
}
