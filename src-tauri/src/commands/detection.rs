use crate::core::detection::{self, DetectedStack};
use crate::core::manifest;
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::store;
use tauri::command;

#[command]
pub async fn detect_project_stack(path: String) -> Result<DetectedStack, String> {
    detection::detect_stack(&path).map_err(|e| e.to_string())
}

#[command]
pub async fn scan_and_register(path: String, name: String) -> Result<Project, String> {
    let stack = detection::detect_stack(&path).map_err(|e| e.to_string())?;

    let runtimes: Vec<RuntimeConfig> = stack.runtimes.clone();
    let tags: Vec<String> = stack.tags.clone();
    let now = chrono::Utc::now().to_rfc3339();

    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        path: path.clone(),
        description: None,
        runtimes,
        status: ProjectStatus::Active,
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: None,
        tags,
    };

    let mut projects = store::get().list_projects().map_err(|e| e.to_string())?;
    projects.push(project.clone());
    store::get().save_projects(&projects).map_err(|e| e.to_string())?;

    let m = manifest::generate_manifest_from_project(&project);
    let _ = manifest::save_manifest(&path, &m);

    Ok(project)
}
