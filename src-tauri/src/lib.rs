// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
use std::env;
use std::sync::Arc;
use tauri::{LogicalSize, Manager, Size, Url};
use tokio::sync::Mutex;

mod yahoo_finance;
use yahoo_finance::YahooFinanceClient;

use crate::algorithms::{monte_carlo_rsi, rsi};
use crate::ollama::ollama_generate;
use crate::requesting::{
    fetch_reddit_access_token, fetch_reddit_subreddit_posts, fetch_yahoo_private,
    get_all_static_ticker_info, get_request_api, req_nasdaq_info, request_deep,
    code_request_api,
};
use crate::sensitive_data::{
    get_all_windows, get_current_monitor_info, get_username, set_base_size,
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

#[derive(Clone)]
pub struct YahooFinanceState(pub Arc<Mutex<YahooFinanceClient>>);

impl YahooFinanceState {
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut inner = self.0.lock().await;
        *inner = YahooFinanceClient::new().await?;
        inner.refresh_crumb().await?;
        Ok(())
    }
}

pub async fn run() {
    println!("Starting Investor Dreams...");
    let ollama_instance = Ollama::new("http://localhost".to_string(), 11434);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(ollama_instance)
        .manage(YahooFinanceState(Arc::new(Mutex::new(
            YahooFinanceClient::new().await.unwrap_or_else(|e| {
                println!("Failed to create YahooFinanceClient: {}", e);
                panic!("Could not initialize YahooFinanceClient");
            }),
        ))))
        .setup(|app| {
            // Initialize Yahoo Finance client
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let yahoo_state = handle.state::<YahooFinanceState>();
                if let Err(e) = yahoo_state.initialize().await {
                    println!("Failed to initialize YahooFinanceState: {}", e);
                }
            });
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

            // tauri::async_runtime::spawn(async move {
            //     let (client, crumb) = match get_yahoo_session().await {
            //         Ok((client, crumb)) => (client,crumb),
            //         Err(e) => panic!("fsdfdsfdsfsf")
            //     };

            // })
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
            get_username,
            get_all_static_ticker_info,
            req_nasdaq_info,
            get_all_windows,
            get_current_monitor_info,
            set_base_size,
            request_deep,
            get_request_api,
            ollama_generate,
            save_json_file,
            save_json_to_folder,
            close_window,
            is_macos,
            monte_carlo_rsi,
            rsi,
            fetch_reddit_subreddit_posts,
            fetch_reddit_access_token,
            fetch_yahoo_private,
            code_request_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
