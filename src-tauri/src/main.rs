// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

fn get_seed_path() -> PathBuf {
    // Get the path to the seed template directory
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    
    // Try different possible locations for the seed folder
    let possible_paths = vec![
        // Development: relative to project root
        exe_dir.join("../../../seed"),
        // Production macOS: in Resources/_up_ folder (Tauri bundles relative resources here)
        exe_dir.join("../Resources/_up_/seed"),
        // Production macOS: alternative Resources location
        exe_dir.join("../Resources/seed"),
        // Production: next to executable
        exe_dir.join("seed"),
        // Fallback: current directory
        PathBuf::from("seed"),
    ];
    
    for path in possible_paths {
        if path.exists() {
            return path;
        }
    }
    
    // Return the Resources/_up_ path as default (most likely for production)
    exe_dir.join("../Resources/_up_/seed")
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        // Skip node_modules and other build artifacts
        if let Some(name) = entry.file_name().to_str() {
            if name == "node_modules" || name == "dist" || name == ".git" || name == "target" {
                continue;
            }
        }

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

#[tauri::command]
fn create_shadcn_project(project_name: String, project_path: String) -> Result<String, String> {
    // Validate project name
    if project_name.trim().is_empty() {
        return Err("Project name cannot be empty".to_string());
    }

    // Validate project path
    let base_path = Path::new(&project_path);
    if !base_path.exists() {
        return Err(format!("Directory does not exist: {}", project_path));
    }

    let full_project_path = base_path.join(&project_name);
    
    // Check if project already exists
    if full_project_path.exists() {
        return Err(format!("Project '{}' already exists in this directory", project_name));
    }

    // Get seed template path
    let seed_path = get_seed_path();
    
    if !seed_path.exists() {
        return Err(format!("Seed template not found at: {}. Please ensure seed folder exists.", seed_path.display()));
    }

    // Step 1: Copy seed template
    copy_dir_recursive(&seed_path, &full_project_path)
        .map_err(|e| format!("Failed to copy seed template: {}", e))?;

    // Step 2: Update package.json with new project name (lowercase for npm)
    let package_json_path = full_project_path.join("package.json");
    if package_json_path.exists() {
        let content = fs::read_to_string(&package_json_path)
            .map_err(|e| format!("Failed to read package.json: {}", e))?;
        
        // Convert project name to lowercase and replace spaces/special chars with hyphens
        let npm_safe_name = project_name
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-");
        
        // Replace the name in package.json
        let updated_content = content.replace(
            r#""name": "seed""#,
            &format!(r#""name": "{}""#, npm_safe_name)
        );
        
        fs::write(&package_json_path, updated_content)
            .map_err(|e| format!("Failed to update package.json: {}", e))?;
    }

    // Step 3: Install dependencies
    // Try to find npm in common locations
    let mut npm_paths = vec![
        "/usr/local/bin/npm".to_string(),
        "/opt/homebrew/bin/npm".to_string(),
        "/usr/bin/npm".to_string(),
    ];
    
    // Add nvm path if HOME is set
    if let Ok(home) = std::env::var("HOME") {
        npm_paths.push(format!("{}/.nvm/versions/node/v22.20.0/bin/npm", home));
    }
    
    let npm_cmd = npm_paths.iter()
        .find(|p| Path::new(p).exists())
        .map(|s| s.as_str())
        .unwrap_or("npm");

    let npm_install = Command::new(npm_cmd)
        .arg("install")
        .current_dir(&full_project_path)
        .env("PATH", format!("/usr/local/bin:/opt/homebrew/bin:/usr/bin:{}", std::env::var("PATH").unwrap_or_default()))
        .status();

    match npm_install {
        Ok(status) if status.success() => {},
        Ok(_) => return Err("Failed to install dependencies. Make sure npm is installed.".to_string()),
        Err(e) => return Err(format!("Error installing dependencies: {}. Make sure npm is installed.", e)),
    }

    Ok(full_project_path.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_shadcn_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
