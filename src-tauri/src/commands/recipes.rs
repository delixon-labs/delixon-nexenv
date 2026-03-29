use crate::core::recipes::{self, Recipe, RecipeApplyResult, RecipePreview};
use crate::core::store;
use tauri::command;

#[command]
pub async fn list_recipes() -> Vec<Recipe> {
    recipes::list_recipes()
}

#[command]
pub async fn preview_recipe(project_id: String, recipe_id: String) -> Result<RecipePreview, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    recipes::preview_recipe(&project.path, &recipe_id).map_err(|e| e.to_string())
}

#[command]
pub async fn apply_recipe(project_id: String, recipe_id: String) -> Result<RecipeApplyResult, String> {
    let projects = store::get().list_projects().map_err(|e| e.to_string())?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Proyecto no encontrado: {}", project_id))?;

    recipes::apply_recipe(&project.path, &recipe_id).map_err(|e| e.to_string())
}
