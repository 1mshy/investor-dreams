use std::fs;
use std::path::Path;

fn main() {
    // Load .env file if it exists
    if Path::new("../.env").exists() {
        let env_content = fs::read_to_string("../.env").expect("Failed to read .env file");

        for line in env_content.lines() {
            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                // Set environment variables that will be available at compile time
                println!("cargo:rustc-env={}={}", key, value);
            }
        }
    } else {
        println!("cargo:warning=.env file not found at ../.env");
    }

    tauri_build::build()
}
