/// Devuelve el sistema operativo actual como string
pub fn get_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    }
}

/// Devuelve la ruta base de datos de Delixon según el SO
pub fn get_data_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|p| p.join("delixon"))
}

/// Devuelve el separador de PATH del SO
pub fn path_separator() -> &'static str {
    if cfg!(target_os = "windows") { ";" } else { ":" }
}
