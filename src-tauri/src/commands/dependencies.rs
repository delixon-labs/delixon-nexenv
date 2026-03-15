use tauri::command;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub is_shared: bool,
}

/// Detecta si una dependencia ya está disponible en el sistema o en la caché de Delixon
#[command]
pub async fn check_dependency(name: String, version: String) -> Result<Option<String>, String> {
    // TODO: Fase 2 — buscar en sistema y caché, devolver ruta si existe
    let _ = (name, version);
    Ok(None)
}
