use crate::config::settings;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn stow_config() -> Result<(), String> {
    let home = env::var("HOME").map_err(|e| format!("HOME not found: {}", e))?;
    
    // Get dotfiles path from settings and convert to PathBuf
    let stow_dir_str = settings::dotfiles_path()
        .ok_or("Dotfiles path not configured")?;
    
    // Expand ~ if needed using strip_prefix
    let stow_dir = if let Some(stripped) = stow_dir_str.strip_prefix("~/") {
        PathBuf::from(&home).join(stripped)
    } else {
        PathBuf::from(stow_dir_str)
    };
    
    let target_dir = Path::new(&home);

    if !stow_dir.exists() {
        return Err(format!("Directory {:?} doesn't exist", stow_dir));
    }

    if !stow_dir.is_dir() {
        return Err(format!("{:?} is not a directory", stow_dir));
    }

    println!("Stowing dotfiles from {} to {}", stow_dir.display(), target_dir.display());

    let status = Command::new("stow")
        .current_dir(&stow_dir)
        .arg(".")
        .arg("-t")
        .arg(target_dir)
        .arg("--restow")
        .status()
        .map_err(|e| format!("Failed to execute stow: {}", e))?;

    if status.success() {
        println!("Dotfiles stowed successfully");
        Ok(())
    } else {
        Err("Error while running stow".to_string())
    }
}
