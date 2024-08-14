// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;

use crate::requesting::get_index_info;
use crate::sensitive_data::{
    get_all_windows, get_api_keys, get_current_monitor_info, get_username, set_base_size,
};
use tauri::Manager;
use window_vibrancy::{apply_acrylic, apply_blur, apply_vibrancy, NSVisualEffectMaterial};
mod requesting;
mod sensitive_constants;
mod sensitive_data;

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_acrylic(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
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
