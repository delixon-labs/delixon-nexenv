use crate::storage;
use std::process::Command;
use tauri::command;

/// Abre una terminal en la carpeta del proyecto con el entorno correcto cargado
#[command]
pub async fn open_terminal(project_id: String) -> Result<(), String> {
    let projects = storage::load_projects()?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    let path = std::path::Path::new(&project.path);
    if !path.exists() || !path.is_dir() {
        return Err(format!(
            "La carpeta del proyecto no existe: {}",
            project.path
        ));
    }

    // Cargar env vars del proyecto
    let env_vars = storage::load_env_vars(&project_id).unwrap_or_default();

    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/c").arg("start").arg("cmd").arg("/k");
        cmd.current_dir(&project.path);
        for (k, v) in &env_vars {
            cmd.env(k, v);
        }
        cmd.spawn()
            .map_err(|e| format!("Error abriendo terminal Windows: {}", e))?;
    } else {
        // Linux: intentar terminales en orden de prioridad
        let xterm_cmd = format!("cd '{}' && $SHELL", project.path);
        let terminals = [
            ("x-terminal-emulator", vec!["--working-directory", project.path.as_str()]),
            ("gnome-terminal", vec!["--working-directory", project.path.as_str()]),
            ("xfce4-terminal", vec!["--working-directory", project.path.as_str()]),
            ("konsole", vec!["--workdir", project.path.as_str()]),
            ("xterm", vec!["-e", xterm_cmd.as_str()]),
        ];

        let mut launched = false;
        for (term, args) in &terminals {
            if which::which(term).is_ok() {
                let mut cmd = Command::new(term);
                cmd.args(args);
                for (k, v) in &env_vars {
                    cmd.env(k, v);
                }
                if cmd.spawn().is_ok() {
                    launched = true;
                    break;
                }
            }
        }

        if !launched {
            return Err("No se encontro un emulador de terminal instalado".to_string());
        }
    }

    Ok(())
}

/// Abre el proyecto en un editor de codigo
#[command]
pub async fn open_in_editor(project_path: String, editor: Option<String>) -> Result<(), String> {
    let path = std::path::Path::new(&project_path);
    if !path.exists() || !path.is_dir() {
        return Err(format!("La carpeta no existe: {}", project_path));
    }

    let editor_cmd = editor.unwrap_or_else(|| "code".to_string());

    Command::new(&editor_cmd)
        .arg(&project_path)
        .spawn()
        .map_err(|e| format!("Error abriendo {}: {}", editor_cmd, e))?;

    Ok(())
}
