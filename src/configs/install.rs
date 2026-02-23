use std::env;
use std::path::Path;
use std::process::Command;

pub fn stow_config() -> Result<(), String> {
    let home = env::var("HOME").map_err(|e| format!("HOME not found: {}", e))?;
    let stow_dir = Path::new(&home).join(".dotfiles/config");
    let target_dir = Path::new(&home);

    if !stow_dir.exists() {
        return Err(format!("Directory {:?} doesnt exists", stow_dir));
    }

    let status = Command::new("stow")
        .current_dir(&stow_dir)
        .arg(".")
        .arg("-t")
        .arg(target_dir)
        .arg("--restow")
        .status()
        .map_err(|e| format!("Error: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Error while running stow".to_string())
    }
}
