/// Devuelve la ruta base de datos de Delixon segun el SO
pub fn get_data_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|p| p.join("delixon"))
}

/// Editores permitidos (whitelist de seguridad) — fuente unica de verdad
pub const ALLOWED_EDITORS: &[&str] = &[
    "code", "code-insiders", "cursor", "zed", "subl", "atom", "nvim",
    "vim", "nano", "emacs", "gedit", "kate", "mousepad", "pluma",
    "webstorm", "phpstorm", "idea", "clion", "goland", "rustrover",
    "fleet", "lapce", "helix",
];
