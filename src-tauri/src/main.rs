// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::Manager;

use crate::file_reading::{get_company_name, get_company_exchange, TickerSymbolData};
use crate::sensitive_data::get_api_keys;

mod file_reading;
mod sensitive_data;
mod csv_data;

fn main() {
  dotenv::dotenv().ok();
  let api_key = env::var("API_KEY").expect("API_KEY must be set");
  
  tauri::Builder::default()
  .setup(|app| {
    let main_window = app.get_window("main").unwrap();
    main_window.emit("api-key", api_key).unwrap();
    Ok(())
})
      .manage(Arc::new(RwLock::new(HashMap::<String, TickerSymbolData>::new()))) // Correctly instantiate the shared state
      .invoke_handler(tauri::generate_handler![get_company_name, get_company_exchange, get_api_keys ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

