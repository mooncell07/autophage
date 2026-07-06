mod commands;
mod core;
use std::sync::Mutex;

use crate::core::session::Session;
use std::sync::Arc;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Arc::new(Mutex::new(Session::preinit())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_session,
            commands::analyze_binary,
            commands::get_information,
            commands::get_functions,
            commands::close_application,
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
