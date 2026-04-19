use crate::core::storage;
use crate::core::store;
use std::process::Command;
use tauri::command;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Rechaza paths con control chars que podrian romper parsers downstream
/// (CMD, PowerShell, shells POSIX) o permitir inyectar args adicionales.
fn path_has_control_chars(path: &str) -> bool {
    path.chars().any(|c| c.is_control())
}

/// Abre una terminal en la carpeta del proyecto con el entorno correcto cargado.
/// Inyecta los bin paths de los runtimes declarados (nvm/fnm/pyenv/rustup) en PATH.
#[command]
pub async fn open_terminal(project_id: String) -> Result<(), String> {
    let total = std::time::Instant::now();
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    if path_has_control_chars(&project.path) {
        return Err("Ruta del proyecto invalida: contiene caracteres de control".to_string());
    }

    let path = std::path::Path::new(&project.path);
    if !path.exists() || !path.is_dir() {
        return Err(format!(
            "La carpeta del proyecto no existe: {}",
            project.path
        ));
    }

    let mut env_vars = store::get().load_env_vars(&project_id).unwrap_or_default();

    if let Ok(history_path) = storage::get_history_path(&project_id) {
        let _ = storage::init_data_dir();
        let hp = history_path.to_string_lossy().to_string();
        env_vars.insert("HISTFILE".to_string(), hp.clone());
        env_vars.insert("PSReadLineHistorySavePath".to_string(), hp);
    }

    let activation = crate::core::runtime::activate::activate(&project.runtimes);
    if !activation.bin_paths.is_empty() {
        let current = std::env::var("PATH").unwrap_or_default();
        env_vars.insert("PATH".to_string(), activation.prefix_path(&current));
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

    println!(
        "[nexenv] open_terminal id={} runtimes={} activation_ms={} total_ms={}",
        project_id,
        project.runtimes.len(),
        activation.elapsed_ms,
        total.elapsed().as_millis()
    );

    Ok(())
}

// --- Windows terminals ---

fn try_windows_terminal(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    if path_has_control_chars(project_path) || which::which("wt").is_err() {
        return false;
    }
    let mut cmd = Command::new("wt");
    cmd.arg("-d").arg(project_path);
    for (k, v) in env_vars {
        cmd.env(k, v);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd.spawn().is_ok()
}

fn try_powershell(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    if path_has_control_chars(project_path) {
        return false;
    }
    let ps = if which::which("pwsh").is_ok() {
        "pwsh"
    } else if which::which("powershell").is_ok() {
        "powershell"
    } else {
        return false;
    };

    // Usamos `start /D <path>` para que la nueva ventana de PowerShell
    // arranque en el directorio correcto sin pasar el path dentro de un
    // comando -Command (evita cualquier riesgo de inyeccion via el path).
    let mut cmd = Command::new("cmd");
    cmd.arg("/c")
        .arg("start")
        .arg("")
        .arg("/D")
        .arg(project_path)
        .arg(ps)
        .arg("-NoExit");
    for (k, v) in env_vars {
        cmd.env(k, v);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd.spawn().is_ok()
}

fn try_cmd(project_path: &str, env_vars: &std::collections::HashMap<String, String>) -> bool {
    if path_has_control_chars(project_path) {
        return false;
    }
    // `start /D <path> cmd /K` abre una ventana nueva de cmd en el dir
    // sin pasar el path por una cadena concatenada.
    let mut cmd = Command::new("cmd");
    cmd.arg("/c")
        .arg("start")
        .arg("")
        .arg("/D")
        .arg(project_path)
        .arg("cmd")
        .arg("/K");
    for (k, v) in env_vars {
        cmd.env(k, v);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(CREATE_NO_WINDOW);
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

    // xterm fallback: xterm no tiene --working-directory, pero hereda el
    // cwd del proceso padre — seteamos cwd via Command::current_dir para
    // evitar concatenar el path en un string de shell.
    if which::which("xterm").is_ok() {
        let mut cmd = Command::new("xterm");
        cmd.current_dir(project_path);
        for (k, v) in env_vars {
            cmd.env(k, v);
        }
        if cmd.spawn().is_ok() {
            return true;
        }
    }

    false
}

use crate::core::utils::platform::detect_installed_editors;

/// Abre el proyecto en un editor de codigo (solo editores de la whitelist)
#[command]
pub async fn open_in_editor(project_path: String, editor: Option<String>) -> Result<(), String> {
    let path = std::path::Path::new(&project_path);
    if !path.exists() || !path.is_dir() {
        return Err(format!("La carpeta no existe: {}", project_path));
    }

    let editor_cmd = editor.unwrap_or_else(|| "code".to_string());
    crate::core::utils::editor::open_in_editor(&project_path, &editor_cmd)
}

/// Retorna los editores instalados en el sistema (cmd, label)
#[command]
pub async fn list_installed_editors() -> Result<Vec<(String, String)>, String> {
    Ok(detect_installed_editors())
}
