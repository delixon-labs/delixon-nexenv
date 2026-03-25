use crate::models::project::{CreateProjectInput, Project, RuntimeConfig};
use crate::storage;
use tauri::command;

/// Devuelve la lista de todos los proyectos registrados en Delixon
#[command]
pub async fn list_projects() -> Result<Vec<Project>, String> {
    storage::load_projects()
}

/// Devuelve un proyecto por su ID
#[command]
pub async fn get_project(id: String) -> Result<Project, String> {
    let projects = storage::load_projects()?;
    projects
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", id))
}

/// Crea un nuevo proyecto a partir de los datos del input
#[command]
pub async fn create_project(input: CreateProjectInput) -> Result<Project, String> {
    let mut projects = storage::load_projects()?;

    // Verificar que no exista un proyecto con el mismo path
    if projects.iter().any(|p| p.path == input.path) {
        return Err(format!(
            "Ya existe un proyecto registrado en esa ruta: {}",
            input.path
        ));
    }

    let now = chrono::Utc::now().to_rfc3339();
    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: input.name,
        path: input.path,
        description: input.description,
        runtimes: input.runtimes,
        status: "active".to_string(),
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: input.template_id,
        tags: input.tags.unwrap_or_default(),
    };

    projects.push(project.clone());
    storage::save_projects(&projects)?;

    Ok(project)
}

/// Abre un proyecto en el editor configurado (por defecto VSCode)
#[command]
pub async fn open_project(id: String) -> Result<(), String> {
    let mut projects = storage::load_projects()?;

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
    project.status = "active".to_string();
    storage::save_projects(&projects)?;

    // Abrir en VSCode
    let editor = "code";
    std::process::Command::new(editor)
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
    status: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Project, String> {
    let mut projects = storage::load_projects()?;

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
    storage::save_projects(&projects)?;

    Ok(updated)
}

/// Elimina un proyecto del registro de Delixon (no borra los archivos del disco)
#[command]
pub async fn delete_project(id: String) -> Result<(), String> {
    let mut projects = storage::load_projects()?;
    let original_len = projects.len();

    projects.retain(|p| p.id != id);

    if projects.len() == original_len {
        return Err(format!("Proyecto no encontrado: {}", id));
    }

    storage::save_projects(&projects)?;

    // Limpiar env vars del proyecto eliminado
    let _ = storage::delete_env_vars(&id);

    Ok(())
}
