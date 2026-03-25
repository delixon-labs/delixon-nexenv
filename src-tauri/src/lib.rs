mod commands;
mod models;
mod storage;
mod utils;

use commands::{environments, projects, runtimes, shell};

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
        ])
        .run(tauri::generate_context!())
        .expect("Error al iniciar Delixon");
}
