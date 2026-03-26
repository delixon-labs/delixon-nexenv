use std::path::Path;

/// Crea una carpeta y todas sus carpetas padre si no existen
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Escribe un archivo con permisos restrictivos (600 en Unix)
pub fn write_private(path: &Path, data: &str) -> std::io::Result<()> {
    std::fs::write(path, data)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(path, perms)?;
    }

    Ok(())
}
