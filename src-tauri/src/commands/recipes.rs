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
pub async fn apply_recipe(
    project_id: String,
    recipe_id: String,
) -> Result<RecipeApplyResult, crate::core::errors::UiError> {
    use crate::core::errors::UiError;
    let projects = store::get().list_projects().map_err(|e| {
        UiError::new(format!("aplicar recipe '{}'", recipe_id))
            .detecto("no se pudo leer el listado de proyectos")
            .fallo(e.to_string())
            .hacer("verifica los permisos del data dir de Nexenv")
    })?;
    let project = projects
        .iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| {
            UiError::new(format!("aplicar recipe '{}'", recipe_id))
                .detecto(format!("no existe el proyecto con id '{}'", project_id))
                .fallo("Project not found")
                .hacer("vuelve al dashboard y selecciona un proyecto valido")
        })?;

    recipes::apply_recipe(&project.path, &recipe_id).map_err(|e| {
        UiError::new(format!("aplicar recipe '{}'", recipe_id))
            .detecto(format!("proyecto: '{}'", project.name))
            .fallo(e.to_string())
            .hacer("revisa los archivos generados por el preview; si conflictan, hazles backup antes de reintentar")
    })
}
