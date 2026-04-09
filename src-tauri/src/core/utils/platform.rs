/// Devuelve la ruta base de datos de Nexenv segun el SO
pub fn get_data_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|p| p.join("nexenv"))
}

/// Editores permitidos (whitelist de seguridad) — fuente unica de verdad
pub const ALLOWED_EDITORS: &[&str] = &[
    "code", "code-insiders", "cursor", "zed", "subl", "atom", "nvim",
    "vim", "nano", "emacs", "gedit", "kate", "mousepad", "pluma",
    "webstorm", "phpstorm", "idea", "clion", "goland", "rustrover",
    "fleet", "lapce", "helix",
];

/// Editores con nombre legible para la UI
pub const EDITOR_LABELS: &[(&str, &str)] = &[
    ("code", "VS Code"),
    ("code-insiders", "VS Code Insiders"),
    ("cursor", "Cursor"),
    ("zed", "Zed"),
    ("subl", "Sublime Text"),
    ("atom", "Atom"),
    ("nvim", "Neovim"),
    ("vim", "Vim"),
    ("nano", "Nano"),
    ("emacs", "Emacs"),
    ("webstorm", "WebStorm"),
    ("phpstorm", "PhpStorm"),
    ("idea", "IntelliJ IDEA"),
    ("clion", "CLion"),
    ("goland", "GoLand"),
    ("rustrover", "RustRover"),
    ("fleet", "Fleet"),
    ("lapce", "Lapce"),
    ("helix", "Helix"),
];

/// Retorna los editores de la whitelist que estan instalados en el sistema
pub fn detect_installed_editors() -> Vec<(String, String)> {
    EDITOR_LABELS
        .iter()
        .filter(|(cmd, _)| find_editor_in_path(cmd).is_some())
        .map(|(cmd, label)| (cmd.to_string(), label.to_string()))
        .collect()
}

/// Busca un editor en PATH, probando variantes `.cmd` y `.exe` en Windows.
/// Retorna `Some(path)` si lo encuentra, `None` si no.
pub fn find_editor_in_path(editor: &str) -> Option<std::path::PathBuf> {
    if let Ok(path) = which::which(editor) {
        return Some(path);
    }

    #[cfg(target_os = "windows")]
    {
        for ext in &["cmd", "exe", "bat"] {
            let with_ext = format!("{}.{}", editor, ext);
            if let Ok(path) = which::which(&with_ext) {
                return Some(path);
            }
        }
    }

    None
}
