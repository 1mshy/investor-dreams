// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::file_reading::{read_csv, get_company_name, get_company_exchange, TickerSymbolData};

mod file_reading;
mod sensitive_data;

pub type TickerInfoMap = Arc<RwLock<HashMap<String, TickerSymbolData>>>;

fn main() {
  tauri::Builder::default()
      .manage(Arc::new(RwLock::new(HashMap::<String, TickerSymbolData>::new()))) // Correctly instantiate the shared state
      .invoke_handler(tauri::generate_handler![read_csv, get_company_name, get_company_exchange, sensitive_data::get_api_key, ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

