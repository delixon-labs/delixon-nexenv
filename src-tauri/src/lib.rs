#[cfg(feature = "tauri-app")]
mod commands;
pub mod core;
pub mod enterprise;

#[cfg(feature = "tauri-app")]
use commands::{
    catalog, config, detection, docker, environments, git, health, manifest, notes,
    portable, processes, projects, recipes, rules, runtimes, scaffold, scripts, shell,
    snapshots, templates, versioning, vscode,
};

#[cfg(feature = "tauri-app")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicializar store global: SQLite con fallback a JSON
    core::store::init(init_store());

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init());

    #[cfg(feature = "auto-updater")]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .invoke_handler(tauri::generate_handler![
            // Projects
            projects::list_projects,
            projects::list_orphan_projects,
            projects::cleanup_orphan_projects,
            projects::get_project,
            projects::create_project,
            projects::open_project,
            projects::update_project,
            projects::delete_project,
            // Environment
            environments::get_env_vars,
            environments::set_env_vars,
            // Runtimes
            runtimes::detect_runtimes,
            // Shell
            shell::open_terminal,
            shell::open_in_editor,
            shell::list_installed_editors,
            // Config
            config::get_config,
            config::set_config,
            // Detection
            detection::detect_project_stack,
            detection::scan_and_register,
            // Templates
            templates::create_from_template,
            // Portable
            portable::export_project,
            portable::preview_import,
            portable::import_project,
            // VSCode
            vscode::generate_vscode_workspace,
            // Manifest
            manifest::get_manifest,
            manifest::regenerate_manifest,
            // Catalog
            catalog::list_catalog,
            catalog::get_catalog_tech,
            catalog::list_catalog_categories,
            // Rules
            rules::validate_stack,
            // Health
            health::check_project_health,
            health::run_doctor,
            health::detect_port_conflicts,
            health::list_project_ports,
            // Scaffold
            scaffold::preview_scaffold,
            scaffold::generate_scaffold,
            // Recipes
            recipes::list_recipes,
            recipes::preview_recipe,
            recipes::apply_recipe,
            // Git
            git::git_status,
            git::git_log,
            // Docker
            docker::docker_status,
            docker::docker_up,
            docker::docker_down,
            docker::docker_logs,
            // Processes
            processes::list_project_processes,
            processes::kill_process,
            // Scripts
            scripts::list_project_scripts,
            scripts::run_project_script,
            // Versioning
            versioning::save_snapshot,
            versioning::list_snapshots,
            versioning::diff_snapshots,
            versioning::preview_rollback,
            versioning::rollback_snapshot,
            // Snapshots
            snapshots::take_env_snapshot,
            snapshots::diff_env_snapshot,
            // Notes
            notes::get_notes,
            notes::add_note,
            notes::delete_note,
        ])
        .run(tauri::generate_context!())
        .expect("Error al iniciar Nexenv");
}

#[cfg(feature = "tauri-app")]
fn init_store() -> std::sync::Arc<dyn core::store::Store> {
    use core::store::{json_store::JsonStore, migration, sqlite_store::SqliteStore};

    // Intentar SQLite
    if let Some(path) = migration::db_path() {
        // Crear data dir si no existe
        let _ = core::project::storage::init_data_dir();

        match SqliteStore::new(path.to_str().unwrap_or("nexenv.db")) {
            Ok(sqlite) => {
                // Migrar JSON si existen datos legacy
                if migration::json_data_exists() {
                    match migration::migrate_json_to_sqlite(&sqlite) {
                        Ok(result) => {
                            eprintln!(
                                "[nexenv] Migrados {} proyectos, {} env vars, {} notas de JSON a SQLite",
                                result.projects, result.env_vars, result.notes
                            );
                        }
                        Err(e) => {
                            eprintln!("[nexenv] Error migrando JSON a SQLite: {}. Usando JSON.", e);
                            return std::sync::Arc::new(JsonStore::new());
                        }
                    }
                }
                return std::sync::Arc::new(sqlite);
            }
            Err(e) => {
                eprintln!("[nexenv] Error inicializando SQLite: {}. Usando JSON.", e);
            }
        }
    }

    std::sync::Arc::new(JsonStore::new())
}
