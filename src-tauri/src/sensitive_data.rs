
use tauri::command;
use std::env;


#[command]
pub fn get_api_key() -> String {
    return env::var("API_KEY").expect("API_KEY must be set")
}