// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use tauri::Manager;

use crate::requesting::get_index_info;
use crate::sensitive_data::{get_api_keys, get_username};
mod requesting;
mod sensitive_data;

fn main() {
    dotenv::dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.emit("api-key", api_key).unwrap();
            Ok(())
        }) // Correctly instantiate the shared state
        .invoke_handler(tauri::generate_handler![
            get_api_keys,
            get_username,
            get_index_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
