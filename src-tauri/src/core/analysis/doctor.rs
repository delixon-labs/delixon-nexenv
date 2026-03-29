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
    pub group: String,
    pub name: String,
    pub ok: bool,
    pub message: String,
}

pub fn run_doctor() -> Result<DoctorReport, DelixonError> {
    let mut checks = Vec::new();

    // --- Grupo: Sistema ---
    match get_data_dir() {
        Some(dir) => {
            let exists = dir.exists();
            checks.push(DoctorCheck {
                group: "Sistema".to_string(),
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
                group: "Sistema".to_string(),
                name: "Directorio de datos".to_string(),
                ok: false,
                message: "No se pudo determinar".to_string(),
            });
        }
    }

    match config::load_config() {
        Ok(cfg) => {
            checks.push(DoctorCheck {
                group: "Sistema".to_string(),
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
                group: "Sistema".to_string(),
                name: "Configuracion".to_string(),
                ok: false,
                message: format!("Error: {}", e),
            });
        }
    }

    match storage::load_projects() {
        Ok(projects) => {
            checks.push(DoctorCheck {
                group: "Sistema".to_string(),
                name: "Proyectos".to_string(),
                ok: true,
                message: format!("{} registrados", projects.len()),
            });
        }
        Err(e) => {
            checks.push(DoctorCheck {
                group: "Sistema".to_string(),
                name: "Proyectos".to_string(),
                ok: false,
                message: format!("Error: {}", e),
            });
        }
    }

    // --- Grupo: Runtimes ---
    let projects = storage::load_projects().unwrap_or_default();
    let used_runtimes: Vec<String> = projects
        .iter()
        .flat_map(|p| p.runtimes.iter().map(|r| r.runtime.to_lowercase()))
        .collect();

    let runtime_checks = [
        ("node", "--version"),
        ("python", "--version"),
        ("rustc", "--version"),
        ("go", "version"),
    ];

    for (cmd, arg) in &runtime_checks {
        let found = which::which(cmd).is_ok();
        let needed = used_runtimes.iter().any(|r| r == cmd || (r == "rust" && *cmd == "rustc"));
        let tag = if needed { "requerido" } else { "no requerido" };
        let version = if found {
            let ver = std::process::Command::new(cmd)
                .arg(arg)
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "?".to_string());
            format!("{} ({})", ver, tag)
        } else if needed {
            "no encontrado (requerido)".to_string()
        } else {
            "no instalado (no requerido)".to_string()
        };

        checks.push(DoctorCheck {
            group: "Runtimes".to_string(),
            name: cmd.to_string(),
            ok: found || !needed,
            message: version,
        });
    }

    // --- Grupo: Herramientas ---
    let docker_found = which::which("docker").is_ok();
    checks.push(DoctorCheck {
        group: "Herramientas".to_string(),
        name: "Docker".to_string(),
        ok: docker_found,
        message: if docker_found {
            "Disponible".to_string()
        } else {
            "No encontrado".to_string()
        },
    });

    let git_found = which::which("git").is_ok();
    checks.push(DoctorCheck {
        group: "Herramientas".to_string(),
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

    let default_editor = config::load_config()
        .map(|c| c.default_editor)
        .unwrap_or_else(|_| "code".to_string());

    // Mostrar todos los editores instalados
    let installed = crate::core::utils::platform::detect_installed_editors();
    if installed.is_empty() {
        checks.push(DoctorCheck {
            group: "Herramientas".to_string(),
            name: "Editores".to_string(),
            ok: false,
            message: "Ningun editor encontrado en PATH".to_string(),
        });
    } else {
        for (cmd, label) in &installed {
            let is_default = *cmd == default_editor;
            let tag = if is_default { " (configurado)" } else { "" };
            let path = crate::core::utils::platform::find_editor_in_path(cmd)
                .map(|p| format!("{}", p.display()))
                .unwrap_or_default();
            checks.push(DoctorCheck {
                group: "Herramientas".to_string(),
                name: format!("{}{}", label, tag),
                ok: true,
                message: path,
            });
        }
        // Verificar que el editor configurado este instalado
        if !installed.iter().any(|(cmd, _)| *cmd == default_editor) {
            checks.push(DoctorCheck {
                group: "Herramientas".to_string(),
                name: format!("Editor configurado ({})", default_editor),
                ok: false,
                message: "No encontrado en PATH".to_string(),
            });
        }
    }

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
