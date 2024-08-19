// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;

use crate::requesting::{get_index_info, req_nasdaq_info, request_deep};
use crate::sensitive_data::{
    get_all_windows, get_api_keys, get_current_monitor_info, get_username, set_base_size,
};
use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};
mod requesting;
mod sensitive_constants;
mod sensitive_data;

// #[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn main() {
    println!("Starting Tauri App");
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(
                &window,
                NSVisualEffectMaterial::HudWindow,
                Some(NSVisualEffectState::Active),
                Some(2.0),
            )
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            println!("Applying blur");
            #[cfg(target_os = "windows")]
            apply_acrylic(&window, Some((1, 1, 1, 1)))
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
            req_nasdaq_info,
            request_deep
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
