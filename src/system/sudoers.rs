use super::utils;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn sudoers(sudoers: String) -> Result<(), String> {
    // Is we root? ye, seriously?
    if !utils::check_root() {
        return Err("Run with sudo".into());
    }

    if sudoers.is_empty() {
        // If we get here, user is dumb
        return Err("sudoers cannot be emty".into());
    }

    // Temp file for secutity
    let temp_path = "/tmp/sudoers.test";
    fs::write(temp_path, format!("{}\n", rule))
        .map_err(|e| format!("Failed to create temp sudoers: {}", e))?;

    // Check if user is not dumb with visudo
    let status = Command::new("visudo")
        .args(["-c", "-f", temp_path])
        .status()
        .map_err(|e| format!("Failed to run visudo: {}", e))?;

    // REMOVE FREAKING TEMP FILE AFTER YOU
    let _ = fs::remove_file(temp_path);

    if !status.success() {
        return Err(format!("Invalid sudoers syntax: '{}'", rule));
    }

    let sudoers_d = Path::new("/etc/sudoers.d");
    if !sudoers_d.exists() {
        fs::create_dir_all(sudoers_d).map_err(|e| format!("Failed to create sudoers.d: {}", e))?;
    }
    let file = sudoers_d.join("99-dotfiles-installer");
    fs::write(&file, format!("{}\n", sudoers)).map_err(|e| format!("Failed to write: {}", e))?;

    Command::new("chmod")
        .args(["440", file.to_str().unwrap()])
        .status()
        .map_err(|e| format!("Failed to chmod: {}", e))?;

    Ok(())
}
