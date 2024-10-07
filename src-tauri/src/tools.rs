use directories::UserDirs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use tauri::command;

#[command]
pub async fn save_json_file(filename: String, json_content: String) -> String {
    return match save_file_to_downloads(&filename, json_content).await {
        Ok(path) => format!("File saved to: {:?}", path),
        Err(e) => format!("Failed to save file {:?}", e),
    };
}

async fn save_file_to_downloads(filename: &str, content: String) -> io::Result<PathBuf> {
    // Get the user's directories
    let user_dirs = UserDirs::new().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find user directories")
    })?;

    // Get the Downloads directory
    let downloads_dir = user_dirs.download_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find Downloads directory",
        )
    })?;

    // Construct the full path to the file
    let file_path = downloads_dir.join(filename);

    // Create the file and wrap it in a buffered writer
    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);

    // Write the content to the file
    writer.write_all(content.as_bytes())?;
    writer.flush()?; // Ensure all data is written to the file

    // Return the path to the saved file
    Ok(file_path)
}
