use std::path::Path;

use super::platform::{find_editor_in_path, ALLOWED_EDITORS};

/// Busca un archivo .code-workspace en el directorio del proyecto
pub fn find_workspace_file(project_path: &str) -> Option<String> {
    let dir = Path::new(project_path);
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".code-workspace") {
                    return Some(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    None
}

/// Valida un editor contra la whitelist, busca el binario en PATH,
/// detecta si hay .code-workspace, y abre el proyecto.
/// Centraliza la lógica de commands/projects.rs, commands/shell.rs y bin/cli.rs.
pub fn open_in_editor(project_path: &str, editor: &str) -> Result<(), String> {
    if !ALLOWED_EDITORS.contains(&editor) {
        return Err(format!(
            "Editor '{}' no permitido. Editores disponibles: {}",
            editor,
            ALLOWED_EDITORS.join(", ")
        ));
    }

    let editor_bin = find_editor_in_path(editor)
        .ok_or_else(|| format!("Editor '{}' no encontrado en PATH", editor))?;

    // Si existe un .code-workspace, abrir ese en vez de la carpeta
    let open_target = find_workspace_file(project_path)
        .unwrap_or_else(|| project_path.to_string());

    std::process::Command::new(&editor_bin)
        .arg(&open_target)
        .spawn()
        .map_err(|e| format!("Error abriendo {}: {}", editor, e))?;

    Ok(())
}
