use directories::UserDirs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tauri::command;

#[command]
pub async fn save_json_file(filename: String, json_content: String) -> String {
    return match save_file_to_downloads(&filename, &json_content) {
        Ok(path) => format!("File saved to: {:?}", path),
        Err(e) => format!("Failed to save file {:?}", e),
    }
}

fn save_file_to_downloads(filename: &str, content: &str) -> std::io::Result<PathBuf> {
    // Get the user's directories
    let user_dirs = UserDirs::new().ok_or_else(|| std::io::Error::new(
        std::io::ErrorKind::NotFound, "Could not find user directories"))?;

    // Get the Downloads directory
    let downloads_dir = user_dirs.download_dir().ok_or_else(|| std::io::Error::new(
        std::io::ErrorKind::NotFound, "Could not find Downloads directory"))?;

    // Construct the full path to the file
    let file_path = downloads_dir.join(filename);

    // Create and write to the file
    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    // Return the path to the saved file
    Ok(file_path)
}