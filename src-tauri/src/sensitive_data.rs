use tauri::command;

#[command]
pub fn get_api_key() -> String {
    // let api_keys:Vec<String> = Vec::new["62f59cf3c1fe46498ed297915d46dfac"];
    // return api_keys[0].to_string();
    return "62f59cf3c1fe46498ed297915d46dfac".to_string();
}
