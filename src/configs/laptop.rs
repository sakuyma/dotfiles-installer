use std::fs::{self, OpenOptions, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::env;

pub fn laptop_mode() -> Result<(), Box<dyn std::error::Error>> {
    let home = env::var("HOME").expect("HOME не найден");
    let hyprland_conf = PathBuf::from(&home).join(".config/hypr/hyprland.conf");

    let source_line = "source = ./modules/laptop.conf";

    // make sure that file exists and there is laptop mode
    if hyprland_conf.exists() {
        let content = read_to_string(&hyprland_conf)?;
        if content.lines().any(|line| line.trim() == source_line) {
            return Ok(());
        }
    }

    // Create directory (for sure)
    if let Some(parent) = hyprland_conf.parent() {
        fs::create_dir_all(parent)?;
    }

    // Add laptop mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(hyprland_conf)?;

    // Idk 
    if file.metadata()?.len() > 0 {
        writeln!(file)?;
    }

    writeln!(file, "{}", source_line)?;

    Ok(())
}

// Im stupid asf
