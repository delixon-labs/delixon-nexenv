use tauri::command;

/// Abre una terminal en la carpeta del proyecto con el entorno correcto cargado
#[command]
pub async fn open_terminal(project_path: String, shell: Option<String>) -> Result<(), String> {
    // TODO: Fase 1 — detectar terminal disponible y abrirla con el entorno del proyecto
    let _ = (project_path, shell);
    Ok(())
}
