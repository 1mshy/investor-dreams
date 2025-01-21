// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
use std::env;
use tauri::{LogicalSize, Manager, Size, Url};

use crate::algorithms::{monte_carlo_rsi, rsi};
use crate::ollama::ollama_generate;
use crate::requesting::{
    fetch_reddit_access_token, fetch_reddit_subreddit_posts, get_all_static_ticker_info,
    get_request_api, reddit_request_api, req_nasdaq_info, request_deep,
};
use crate::sensitive_data::{
    get_all_windows, get_api_keys, get_current_monitor_info, get_username, set_base_size,
};
use crate::tools::{close_window, is_macos, save_json_file, save_json_to_folder};
use window_vibrancy::{apply_acrylic, apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};
mod algorithms;
mod ollama;
mod requesting;
mod sensitive_constants;
mod sensitive_data;
mod tools;

use ollama_rs::Ollama;
use once_cell::sync::Lazy; // Import Ollama from the `ollama-rs` crate
                           // #[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    println!("Starting Investor Dreams...");
    dotenv::dotenv().ok();
    let ollama_instance = Ollama::new("http://localhost".to_string(), 11434);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(ollama_instance) // Manage the Ollama instance
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            // setting screen size to 2/3 of the screen
            match window.current_monitor() {
                Ok(monitor) => {
                    let current_monitor = monitor.unwrap();
                    let scale_factor = current_monitor.scale_factor();
                    let two_thirds_screen = (2f64 / 3f64) / scale_factor;
                    let width = current_monitor.size().width as f64 * two_thirds_screen;
                    let height = current_monitor.size().height as f64 * two_thirds_screen;
                    match window.set_size(Size::Logical(LogicalSize { width, height })) {
                        Ok(_) => {
                            println!("Set window size to: {}x{}", width, height);
                        }
                        Err(e) => {
                            println!("Error setting window size: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error getting monitor: {:?}", e);
                }
            }
            // second window
            // let webview_url = tauri::WebviewUrl::External(Url::parse("https://google.com").unwrap());
            // tauri::WebviewWindowBuilder::new(app, "second", webview_url)
            // .title("Second")
            // .build()?;

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
            get_all_static_ticker_info,
            req_nasdaq_info,
            get_all_windows,
            get_current_monitor_info,
            set_base_size,
            request_deep,
            get_request_api,
            reddit_request_api,
            ollama_generate,
            save_json_file,
            save_json_to_folder,
            close_window,
            is_macos,
            monte_carlo_rsi,
            rsi,
            fetch_reddit_subreddit_posts,
            fetch_reddit_access_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
