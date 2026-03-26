use crate::core::storage;
use std::process::Command;
use tauri::command;

/// Abre una terminal en la carpeta del proyecto con el entorno correcto cargado
#[command]
pub async fn open_terminal(project_id: String) -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
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
    let mut env_vars = storage::load_env_vars(&project_id).unwrap_or_default();

    // Terminal history isolation (bash, zsh, PowerShell)
    if let Ok(history_path) = storage::get_history_path(&project_id) {
        let _ = storage::init_data_dir();
        let hp = history_path.to_string_lossy().to_string();
        // bash y zsh usan HISTFILE
        env_vars.insert("HISTFILE".to_string(), hp.clone());
        // PowerShell usa PSReadLineHistorySavePath
        env_vars.insert("PSReadLineHistorySavePath".to_string(), hp);
    }

    if cfg!(target_os = "windows") {
        // Windows: intentar Windows Terminal (wt), PowerShell, luego cmd
        let launched = try_windows_terminal(&project.path, &env_vars)
            || try_powershell(&project.path, &env_vars)
            || try_cmd(&project.path, &env_vars);

        if !launched {
            return Err("No se encontro una terminal en Windows".to_string());
        }
    } else if cfg!(target_os = "macos") {
        // macOS: intentar Terminal.app, iTerm2, luego los Linux genericos
        let launched = try_macos_terminal(&project.path, &env_vars)
            || try_linux_terminals(&project.path, &env_vars);

        if !launched {
            return Err("No se encontro un emulador de terminal en macOS".to_string());
        }
    } else {
        // Linux: lista amplia de emuladores
        if !try_linux_terminals(&project.path, &env_vars) {
            return Err("No se encontro un emulador de terminal instalado. Soportados: x-terminal-emulator, gnome-terminal, xfce4-terminal, konsole, mate-terminal, tilix, alacritty, kitty, wezterm, foot, terminator, sakura, lxterminal, xterm".to_string());
        }
    }

    Ok(())
}

// --- Windows terminals ---

fn try_windows_terminal(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    // Windows Terminal (wt.exe)
    if which::which("wt").is_ok() {
        let mut cmd = Command::new("wt");
        cmd.arg("-d").arg(project_path);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }
    false
}

fn try_powershell(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    // PowerShell
    let ps = if which::which("pwsh").is_ok() {
        "pwsh"
    } else if which::which("powershell").is_ok() {
        "powershell"
    } else {
        return false;
    };

    let safe_path = project_path.replace('\'', "''");
    let mut cmd = Command::new("cmd");
    cmd.arg("/c").arg("start").arg(ps).arg("-NoExit").arg("-Command")
        .arg(format!("Set-Location '{}'", safe_path));
    for (k, v) in env_vars {
        cmd.env(k, v);
    }
    cmd.spawn().is_ok()
}

fn try_cmd(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    let safe_path = project_path.replace('"', "");
    let mut cmd = Command::new("cmd");
    cmd.arg("/c").arg("start").arg("cmd").arg("/k")
        .arg(format!("cd /d \"{}\"", safe_path));
    for (k, v) in env_vars {
        cmd.env(k, v);
    }
    cmd.spawn().is_ok()
}

// --- macOS terminal ---

fn try_macos_terminal(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    // Try open -a Terminal
    let mut cmd = Command::new("open");
    cmd.arg("-a").arg("Terminal").arg(project_path);
    for (k, v) in env_vars {
        cmd.env(k, v);
    }
    cmd.spawn().is_ok()
}

// --- Linux terminals ---

fn try_linux_terminals(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    // Terminales con --working-directory
    let wd_terminals = [
        "x-terminal-emulator",
        "gnome-terminal",
        "xfce4-terminal",
        "mate-terminal",
        "tilix",
        "terminator",
        "sakura",
        "lxterminal",
    ];

    for term in &wd_terminals {
        if which::which(term).is_ok() {
            let mut cmd = Command::new(term);
            cmd.arg("--working-directory").arg(project_path);
            for (k, v) in env_vars {
                cmd.env(k, v);
            }
            if cmd.spawn().is_ok() {
                return true;
            }
        }
    }

    // konsole usa --workdir
    if which::which("konsole").is_ok() {
        let mut cmd = Command::new("konsole");
        cmd.arg("--workdir").arg(project_path);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }

    // alacritty usa --working-directory
    if which::which("alacritty").is_ok() {
        let mut cmd = Command::new("alacritty");
        cmd.arg("--working-directory").arg(project_path);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }

    // kitty usa --directory
    if which::which("kitty").is_ok() {
        let mut cmd = Command::new("kitty");
        cmd.arg("--directory").arg(project_path);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }

    // wezterm usa start --cwd
    if which::which("wezterm").is_ok() {
        let mut cmd = Command::new("wezterm");
        cmd.arg("start").arg("--cwd").arg(project_path);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }

    // foot usa --working-directory (Wayland)
    if which::which("foot").is_ok() {
        let mut cmd = Command::new("foot");
        cmd.arg("--working-directory").arg(project_path);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }

    // xterm fallback
    if which::which("xterm").is_ok() {
        let safe_path = project_path.replace('\'', "'\\''");
        let shell_cmd = format!("cd '{}' && exec $SHELL", safe_path);
        let mut cmd = Command::new("xterm");
        cmd.arg("-e").arg("sh").arg("-c").arg(&shell_cmd);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }

    false
}

use crate::core::utils::platform::ALLOWED_EDITORS;

/// Abre el proyecto en un editor de codigo (solo editores de la whitelist)
#[command]
pub async fn open_in_editor(project_path: String, editor: Option<String>) -> Result<(), String> {
    let path = std::path::Path::new(&project_path);
    if !path.exists() || !path.is_dir() {
        return Err(format!("La carpeta no existe: {}", project_path));
    }

    let editor_cmd = editor.unwrap_or_else(|| "code".to_string());

    if !ALLOWED_EDITORS.contains(&editor_cmd.as_str()) {
        return Err(format!(
            "Editor '{}' no permitido. Editores disponibles: {}",
            editor_cmd,
            ALLOWED_EDITORS.join(", ")
        ));
    }

    Command::new(&editor_cmd)
        .arg(&project_path)
        .spawn()
        .map_err(|e| format!("Error abriendo {}: {}", editor_cmd, e))?;

    Ok(())
}
