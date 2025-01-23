use std::{collections::HashMap, env};
use tauri::{command, PhysicalSize};

use crate::sensitive_constants;

#[command]
pub fn get_username() -> String {
    whoami::realname()
}

#[command]
pub async fn get_all_windows(application_window: tauri::Window) -> Result<Vec<String>, String> {
    let monitor_list = application_window
        .available_monitors()
        .map_err(|e| e.to_string())?;
    let monitors = monitor_list
        .iter()
        .map(|monitor| format!("{:?}: {:?}", monitor.name(), monitor.position()))
        .collect();
    Ok(monitors)
}

/**
 * {width: "", height: "", scale_factor: ""}
 */
#[command]
pub async fn get_current_monitor_info(
    application_window: tauri::Window,
) -> Result<HashMap<String, String>, String> {
    let current_monitor = application_window
        .current_monitor()
        .unwrap()
        .expect("No monitor found");
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(
        "width".to_string(),
        format!("{:?}", current_monitor.size().width),
    );
    map.insert(
        "height".to_string(),
        format!("{:?}", current_monitor.size().height),
    );
    map.insert(
        "scale_factor".to_string(),
        current_monitor.scale_factor().to_string(),
    );

    Ok(map)
}

#[command]
pub async fn set_base_size(application_window: tauri::Window) -> Result<u32, String> {
    let current_monitor = application_window
        .current_monitor()
        .unwrap()
        .expect("No monitor found");
    let current_width = current_monitor.size().width as f64;
    let current_height = current_monitor.size().height as f64;
    let scale_factor = current_monitor.scale_factor();
    let new_width = (current_width / 2.5 * scale_factor) as u32;
    let new_height = (current_height / 2.5 * scale_factor) as u32;
    application_window
        .set_size(PhysicalSize::new(new_width, new_height))
        .unwrap();
    Ok(new_width)
}
