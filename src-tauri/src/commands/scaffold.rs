use crate::core::scaffold::{self, ScaffoldConfig, ScaffoldPreview};
use crate::core::models::project::{Project, ProjectStatus};
use crate::core::storage;
use tauri::command;

#[command]
pub async fn preview_scaffold(config: ScaffoldConfig) -> Result<ScaffoldPreview, String> {
    Ok(scaffold::preview_scaffold(&config))
}

#[command]
pub async fn generate_scaffold(config: ScaffoldConfig) -> Result<Project, String> {
    let result = scaffold::generate_project(&config).map_err(|e| e.to_string())?;

    let now = chrono::Utc::now().to_rfc3339();
    let runtimes = result
        .manifest
        .technologies
        .iter()
        .filter_map(|t| {
            let all = crate::core::catalog::load_all_technologies();
            all.iter()
                .find(|tech| tech.id == *t && tech.category == "runtime")
                .map(|tech| crate::core::models::project::RuntimeConfig {
                    runtime: tech.id.clone(),
                    version: tech.default_version.clone(),
                })
        })
        .collect();

    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: config.name.clone(),
        path: config.path.clone(),
        description: None,
        runtimes,
        status: ProjectStatus::Active,
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: None,
        tags: config.technologies.clone(),
    };

    let mut projects = storage::load_projects().map_err(|e| e.to_string())?;
    projects.push(project.clone());
    storage::save_projects(&projects).map_err(|e| e.to_string())?;

    Ok(project)
}
