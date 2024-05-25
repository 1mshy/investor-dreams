
use tauri::command;
use std::env;


#[command]
pub fn get_api_keys() -> String {
    return env::var("API_KEY").expect("API_KEY must be set")
}

#[command]
pub fn get_username() -> String {
    return whoami::realname()
}