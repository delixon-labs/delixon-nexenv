use serde::Serialize;

use crate::core::config;
use crate::core::error::DelixonError;
use crate::core::storage;
use crate::core::utils::platform::get_data_dir;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DoctorReport {
    pub checks: Vec<DoctorCheck>,
    pub overall_ok: bool,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DoctorCheck {
    pub name: String,
    pub ok: bool,
    pub message: String,
}

pub fn run_doctor() -> Result<DoctorReport, DelixonError> {
    let mut checks = Vec::new();

    // 1. Data dir
    match get_data_dir() {
        Some(dir) => {
            let exists = dir.exists();
            checks.push(DoctorCheck {
                name: "Directorio de datos".to_string(),
                ok: exists,
                message: if exists {
                    format!("{}", dir.display())
                } else {
                    "No existe (se creara al primer uso)".to_string()
                },
            });
        }
        None => {
            checks.push(DoctorCheck {
                name: "Directorio de datos".to_string(),
                ok: false,
                message: "No se pudo determinar".to_string(),
            });
        }
    }

    // 2. Config
    match config::load_config() {
        Ok(cfg) => {
            checks.push(DoctorCheck {
                name: "Configuracion".to_string(),
                ok: true,
                message: format!(
                    "editor={}, tema={}, idioma={}",
                    cfg.default_editor, cfg.theme, cfg.language
                ),
            });
        }
        Err(e) => {
            checks.push(DoctorCheck {
                name: "Configuracion".to_string(),
                ok: false,
                message: format!("Error: {}", e),
            });
        }
    }

    // 3. Projects count
    match storage::load_projects() {
        Ok(projects) => {
            checks.push(DoctorCheck {
                name: "Proyectos".to_string(),
                ok: true,
                message: format!("{} registrados", projects.len()),
            });
        }
        Err(e) => {
            checks.push(DoctorCheck {
                name: "Proyectos".to_string(),
                ok: false,
                message: format!("Error: {}", e),
            });
        }
    }

    // 4. Runtimes
    let runtime_checks = [
        ("node", "--version"),
        ("python", "--version"),
        ("rustc", "--version"),
        ("go", "version"),
    ];

    for (cmd, arg) in &runtime_checks {
        let found = which::which(cmd).is_ok();
        let version = if found {
            std::process::Command::new(cmd)
                .arg(arg)
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "?".to_string())
        } else {
            "no encontrado".to_string()
        };

        checks.push(DoctorCheck {
            name: format!("Runtime: {}", cmd),
            ok: found,
            message: version,
        });
    }

    // 5. Docker
    let docker_found = which::which("docker").is_ok();
    checks.push(DoctorCheck {
        name: "Docker".to_string(),
        ok: docker_found,
        message: if docker_found {
            "Disponible".to_string()
        } else {
            "No encontrado".to_string()
        },
    });

    // 6. Git
    let git_found = which::which("git").is_ok();
    checks.push(DoctorCheck {
        name: "Git".to_string(),
        ok: git_found,
        message: if git_found {
            std::process::Command::new("git")
                .arg("--version")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "disponible".to_string())
        } else {
            "No encontrado".to_string()
        },
    });

    // 7. Editor
    let editor = config::load_config()
        .map(|c| c.default_editor)
        .unwrap_or_else(|_| "code".to_string());
    let editor_found = which::which(&editor).is_ok();
    checks.push(DoctorCheck {
        name: format!("Editor ({})", editor),
        ok: editor_found,
        message: if editor_found {
            "Disponible en PATH".to_string()
        } else {
            "No encontrado en PATH".to_string()
        },
    });

    let overall_ok = checks.iter().all(|c| c.ok);

    Ok(DoctorReport { checks, overall_ok })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_doctor() {
        let report = run_doctor().unwrap();
        assert!(!report.checks.is_empty());
        // At minimum, data dir and config should work
        assert!(report.checks.iter().any(|c| c.name == "Directorio de datos"));
        assert!(report.checks.iter().any(|c| c.name == "Configuracion"));
    }
}
