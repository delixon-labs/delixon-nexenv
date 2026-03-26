use std::path::Path;

/// Crea una carpeta y todas sus carpetas padre si no existen
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}
