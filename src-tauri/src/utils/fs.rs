use std::path::Path;

/// Crea una carpeta y todas sus carpetas padre si no existen
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Comprueba si una ruta existe y es un directorio
pub fn is_dir(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Comprueba si una ruta existe y es un archivo
pub fn is_file(path: &Path) -> bool {
    path.exists() && path.is_file()
}
