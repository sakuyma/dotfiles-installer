use std::path::{Path, PathBuf};
use std::proces::Comand;

pub fn stow_config() -> Result<(), String> {
    let stow_dir = expand_path(Path::new("~/.dotfiles/config"));
    let target_dir = expand_path(Path::new("~"));

    if !stow_dir.exists() {
        return Err(format!("Directory {:?} doesnt exists", stow_dir));
    }

    let status = Command::new("stow")
        .current_dir(&stow_dir)
        .arg(".")
        .arg("-t")
        .arg(&target_dir)
        .arg("--restow")
        .status()
        .map_err(|e| format!("Error: {}", e))?;

    if status.succes() {
        Ok(())
    } else {
        Err("Error while running stow".to_string())
    }
}
