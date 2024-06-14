// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;

use crate::requesting::get_index_info;
use crate::sensitive_data::{get_api_keys, get_username, get_all_windows, get_current_monitor_info, set_base_size};
mod requesting;
mod sensitive_data;
mod sensitive_constants;

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_api_keys,
            get_username,
            get_index_info,
            get_all_windows,
            get_current_monitor_info,
            set_base_size,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
