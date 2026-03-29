use std::path::Path;

/// Crea una carpeta y todas sus carpetas padre si no existen
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Asegura que ciertas entradas existan en el .gitignore del proyecto.
/// Si el archivo no existe, lo crea. Si ya contiene la entrada, la ignora.
pub fn ensure_gitignore_entries(project_path: &Path, entries: &[&str]) -> std::io::Result<()> {
    let gitignore_path = project_path.join(".gitignore");
    let existing = std::fs::read_to_string(&gitignore_path).unwrap_or_default();

    let mut to_add: Vec<&str> = entries
        .iter()
        .filter(|entry| !existing.lines().any(|line| line.trim() == **entry))
        .copied()
        .collect();

    if to_add.is_empty() {
        return Ok(());
    }

    let mut content = existing;
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    if !content.is_empty() {
        content.push_str("\n# Delixon (generado automaticamente)\n");
    }
    for entry in &to_add {
        content.push_str(entry);
        content.push('\n');
    }

    std::fs::write(&gitignore_path, content)
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
