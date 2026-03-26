#[cfg(feature = "tauri-app")]
mod commands;
pub mod core;

#[cfg(feature = "tauri-app")]
use commands::{config, detection, environments, portable, projects, runtimes, shell, templates, vscode};

#[cfg(feature = "tauri-app")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            projects::list_projects,
            projects::get_project,
            projects::create_project,
            projects::open_project,
            projects::update_project,
            projects::delete_project,
            environments::get_env_vars,
            environments::set_env_vars,
            runtimes::detect_runtimes,
            shell::open_terminal,
            shell::open_in_editor,
            config::get_config,
            config::set_config,
            detection::detect_project_stack,
            templates::create_from_template,
            portable::export_project,
            portable::import_project,
            vscode::generate_vscode_workspace,
        ])
        .run(tauri::generate_context!())
        .expect("Error al iniciar Delixon");
}
