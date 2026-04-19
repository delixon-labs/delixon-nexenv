use crate::core::manifest;
use crate::core::models::project::{CreateProjectInput, Project, ProjectStatus, RuntimeConfig};
use crate::core::store;
use tauri::command;

/// Devuelve la lista de todos los proyectos registrados en Nexenv
#[command]
pub async fn list_projects() -> Result<Vec<Project>, String> {
    store::get().list_projects().map_err(|e| e.to_string())
}

/// Devuelve un proyecto por su ID
#[command]
pub async fn get_project(id: String) -> Result<Project, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    projects
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", id))
}

/// Crea un nuevo proyecto a partir de los datos del input
#[command]
pub async fn create_project(input: CreateProjectInput) -> Result<Project, String> {
    let mut projects = store::get().list_projects().map_err(|e| e.to_string())?;

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
    store::get().save_projects(&projects).map_err(|e| e.to_string())?;

    // Generar manifest automaticamente
    let m = manifest::generate_manifest_from_project(&project);
    let _ = manifest::save_manifest(&project.path, &m);

    Ok(project)
}

/// Abre un proyecto en el editor configurado (por defecto VSCode).
/// Activa los runtimes declarados en el manifest (nvm/fnm/pyenv/rustup) anteponiendo
/// sus directorios bin al PATH del proceso hijo.
#[command]
pub async fn open_project(id: String) -> Result<(), crate::core::errors::UiError> {
    use crate::core::errors::UiError;
    let total = std::time::Instant::now();
    let mut projects = store::get().list_projects().map_err(|e| {
        UiError::new("abrir proyecto")
            .detecto("no se pudo leer el listado de proyectos")
            .fallo(e.to_string())
            .hacer("verifica los permisos del data dir de Nexenv y reinicia la app")
    })?;

    let project = projects
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| {
            UiError::new("abrir proyecto")
                .detecto(format!("no existe el proyecto con id '{}'", id))
                .fallo("Project not found")
                .hacer("vuelve al dashboard y selecciona un proyecto valido")
        })?;

    let project_path = project.path.clone();
    let path = std::path::Path::new(&project_path);
    if !path.exists() || !path.is_dir() {
        return Err(UiError::new("abrir proyecto")
            .detecto(format!("la carpeta del proyecto no existe en disco: {}", project_path))
            .fallo("path inexistente o no es directorio")
            .hacer("revisa si moviste o eliminaste la carpeta; actualiza la ruta en el proyecto"));
    }

    let runtimes = project.runtimes.clone();

    project.last_opened_at = Some(chrono::Utc::now().to_rfc3339());
    project.status = ProjectStatus::Active;
    store::get().save_projects(&projects).map_err(|e| {
        UiError::new("abrir proyecto")
            .detecto("no se pudo guardar el estado del proyecto")
            .fallo(e.to_string())
            .hacer("verifica los permisos del data dir de Nexenv")
    })?;

    let editor = store::get()
        .load_config()
        .map(|c| c.default_editor)
        .unwrap_or_else(|_| "code".to_string());

    let activation = crate::core::runtime::activate::activate(&runtimes);
    let mut env = std::collections::HashMap::new();
    if !activation.bin_paths.is_empty() {
        let current = std::env::var("PATH").unwrap_or_default();
        env.insert("PATH".to_string(), activation.prefix_path(&current));
    }

    crate::core::utils::editor::open_in_editor_with_env(&project_path, &editor, &env)
        .map_err(|e| {
            UiError::new("abrir proyecto en el editor")
                .detecto(format!("editor configurado: '{}'", editor))
                .fallo(e)
                .hacer(format!("instala '{}' o cambia el editor por defecto en Settings", editor))
        })?;

    let total_ms = total.elapsed().as_millis();
    println!(
        "[nexenv] open_project id={} runtimes={} activation_ms={} total_ms={}",
        id,
        runtimes.len(),
        activation.elapsed_ms,
        total_ms
    );

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
    let mut projects = store::get().list_projects().map_err(|e| e.to_string())?;

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
    store::get().save_projects(&projects).map_err(|e| e.to_string())?;

    Ok(updated)
}

/// Elimina un proyecto del registro de Nexenv (no borra los archivos del disco)
#[command]
pub async fn delete_project(id: String) -> Result<(), String> {
    let mut projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let original_len = projects.len();

    projects.retain(|p| p.id != id);

    if projects.len() == original_len {
        return Err(format!("Proyecto no encontrado: {}", id));
    }

    store::get().save_projects(&projects).map_err(|e| e.to_string())?;

    // Limpiar env vars del proyecto eliminado
    let _ = store::get().delete_env_vars(&id);

    Ok(())
}
