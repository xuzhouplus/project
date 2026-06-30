mod api;
mod commands;
mod crypto;
mod db;
mod migration;
mod models;

pub use api::{ApiResponse, IdResult};
pub use crypto::CryptoState;
pub use db::DbState;
pub use models::{
    CreateEnvironment, CreateProject, Environment, Project, UpdateEnvironment, UpdateProject,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> Result<ApiResponse<String>, String> {
    Ok(ApiResponse::success(format!(
        "Hello, {name}! You've been greeted from Rust!"
    )))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:app.db", migration::migrations())
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            db::setup_db(app)?;
            crypto::setup(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::project::create_project,
            commands::project::get_project,
            commands::project::list_projects,
            commands::project::update_project,
            commands::project::delete_project,
            commands::environment::create_environment,
            commands::environment::get_environment,
            commands::environment::list_environments,
            commands::environment::update_environment,
            commands::environment::delete_environment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
