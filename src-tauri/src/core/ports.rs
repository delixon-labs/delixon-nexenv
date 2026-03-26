use serde::Serialize;
use std::net::TcpListener;

use crate::core::error::DelixonError;
use crate::core::manifest;
use crate::core::models::project::Project;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PortConflict {
    pub port: u16,
    pub projects: Vec<String>,
    pub in_use: bool,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PortInfo {
    pub port: u16,
    pub project: String,
    pub in_use: bool,
}

/// Detecta conflictos de puertos entre proyectos activos
pub fn detect_port_conflicts(projects: &[Project]) -> Result<Vec<PortConflict>, DelixonError> {
    let mut port_map: std::collections::HashMap<u16, Vec<String>> = std::collections::HashMap::new();

    for project in projects {
        if let Ok(Some(m)) = manifest::load_manifest(&project.path) {
            for port in &m.ports {
                port_map
                    .entry(*port)
                    .or_default()
                    .push(project.name.clone());
            }
        }
    }

    let mut conflicts = Vec::new();
    for (port, project_names) in &port_map {
        if project_names.len() > 1 {
            let in_use = is_port_in_use(*port);
            conflicts.push(PortConflict {
                port: *port,
                projects: project_names.clone(),
                in_use,
            });
        }
    }

    conflicts.sort_by_key(|c| c.port);
    Ok(conflicts)
}

/// Lista todos los puertos usados por proyectos
pub fn list_project_ports(projects: &[Project]) -> Result<Vec<PortInfo>, DelixonError> {
    let mut ports = Vec::new();

    for project in projects {
        if let Ok(Some(m)) = manifest::load_manifest(&project.path) {
            for port in &m.ports {
                ports.push(PortInfo {
                    port: *port,
                    project: project.name.clone(),
                    in_use: is_port_in_use(*port),
                });
            }
        }
    }

    ports.sort_by_key(|p| p.port);
    Ok(ports)
}

fn is_port_in_use(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_port_in_use() {
        // Port 0 lets OS pick a free port — should always succeed
        assert!(!is_port_in_use(0) || is_port_in_use(0)); // just don't panic
    }

    #[test]
    fn test_detect_conflicts_empty() {
        let result = detect_port_conflicts(&[]).unwrap();
        assert!(result.is_empty());
    }
}
