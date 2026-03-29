use crate::core::config;
use crate::core::manifest;
use crate::core::models::project::{CreateProjectInput, Project, ProjectStatus, RuntimeConfig};
use crate::core::storage;
use tauri::command;

/// Devuelve la lista de todos los proyectos registrados en Delixon
#[command]
pub async fn list_projects() -> Result<Vec<Project>, String> {
    storage::load_projects().map_err(|e| e.to_string())
}

/// Devuelve un proyecto por su ID
#[command]
pub async fn get_project(id: String) -> Result<Project, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    projects
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", id))
}

/// Crea un nuevo proyecto a partir de los datos del input
#[command]
pub async fn create_project(input: CreateProjectInput) -> Result<Project, String> {
    let mut projects = storage::load_projects().map_err(|e| e.to_string())?;

    // Verificar que no exista un proyecto con el mismo path
    if projects.iter().any(|p| p.path == input.path) {
        return Err(format!(
            "Ya existe un proyecto registrado en esa ruta: {}",
            input.path
        ));
    }

    // Validar path: si no existe, crearlo; si existe pero no es directorio, error
    let path = std::path::Path::new(&input.path);
    if path.exists() && !path.is_dir() {
        return Err(format!(
            "La ruta existe pero no es un directorio: {}",
            input.path
        ));
    }
    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| format!("Error creando directorio del proyecto: {}", e))?;
    }

    let now = chrono::Utc::now().to_rfc3339();
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: input.name,
        path: input.path,
        description: input.description,
        runtimes: input.runtimes,
        status: ProjectStatus::Active,
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: input.template_id,
        tags: input.tags.unwrap_or_default(),
    };

    projects.push(project.clone());
    storage::save_projects(&projects).map_err(|e| e.to_string())?;

    // Generar manifest automaticamente
    let m = manifest::generate_manifest_from_project(&project);
    let _ = manifest::save_manifest(&project.path, &m);

    Ok(project)
}

/// Abre un proyecto en el editor configurado (por defecto VSCode)
#[command]
pub async fn open_project(id: String) -> Result<(), String> {
    let mut projects = storage::load_projects().map_err(|e| e.to_string())?;

    let project = projects
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", id))?;

    // Verificar que la carpeta exista
    let project_path = project.path.clone();
    let path = std::path::Path::new(&project_path);
    if !path.exists() || !path.is_dir() {
        return Err(format!(
            "La carpeta del proyecto no existe: {}",
            project_path
        ));
    }

    // Actualizar last_opened_at
    project.last_opened_at = Some(chrono::Utc::now().to_rfc3339());
    project.status = ProjectStatus::Active;
    storage::save_projects(&projects).map_err(|e| e.to_string())?;

    // Abrir en editor configurado (default: code, validado contra whitelist)
    let editor = config::load_config()
        .map(|c| c.default_editor)
        .unwrap_or_else(|_| "code".to_string());

    if !crate::core::utils::platform::ALLOWED_EDITORS.contains(&editor.as_str()) {
        return Err(format!("Editor '{}' no esta en la lista de editores permitidos", editor));
    }

    let editor_bin = crate::core::utils::platform::find_editor_in_path(&editor)
        .ok_or_else(|| format!("Editor '{}' no encontrado en PATH", editor))?;

    std::process::Command::new(&editor_bin)
        .arg(&project_path)
        .spawn()
        .map_err(|e| format!("Error abriendo {}: {}", editor, e))?;

    Ok(())
}

/// Actualiza los campos de un proyecto existente
#[command]
pub async fn update_project(
    id: String,
    name: Option<String>,
    description: Option<String>,
    runtimes: Option<Vec<RuntimeConfig>>,
    status: Option<ProjectStatus>,
    tags: Option<Vec<String>>,
) -> Result<Project, String> {
    let mut projects = storage::load_projects().map_err(|e| e.to_string())?;

    let project = projects
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", id))?;

    if let Some(n) = name {
        project.name = n;
    }
    if let Some(d) = description {
        project.description = Some(d);
    }
    if let Some(r) = runtimes {
        project.runtimes = r;
    }
    if let Some(s) = status {
        project.status = s;
    }
    if let Some(t) = tags {
        project.tags = t;
    }

    let updated = project.clone();
    storage::save_projects(&projects).map_err(|e| e.to_string())?;

    Ok(updated)
}

/// Elimina un proyecto del registro de Delixon (no borra los archivos del disco)
#[command]
pub async fn delete_project(id: String) -> Result<(), String> {
    let mut projects = storage::load_projects().map_err(|e| e.to_string())?;
    let original_len = projects.len();

    projects.retain(|p| p.id != id);

    if projects.len() == original_len {
        return Err(format!("Proyecto no encontrado: {}", id));
    }

    storage::save_projects(&projects).map_err(|e| e.to_string())?;

    // Limpiar env vars del proyecto eliminado
    let _ = storage::delete_env_vars(&id);

    Ok(())
}
