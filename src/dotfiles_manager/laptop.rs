use crate::config::settings;
use std::env;
use std::fs::{self, OpenOptions, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn laptop_mode() -> Result<(), Box<dyn std::error::Error>> {
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

fn enable_tlp() -> Result<(), String> {
    println!("Enabling TLP service...");

    // Enable and start TLP service
    let status = Command::new("systemctl")
        .args(["enable", "--now", "tlp.service"])
        .status()
        .map_err(|e| format!("Failed to run systemctl: {}", e))?;

    if !status.success() {
        return Err("Failed to enable TLP service".to_string());
    }

    println!("TLP enabled successfully");

    // Enable tlp-sleep
    let _ = Command::new("systemctl")
        .args(["enable", "tlp-sleep.service"])
        .status();

    // Mask rfkill services
    let _ = Command::new("systemctl")
        .args(["mask", "systemd-rfkill.service", "systemd-rfkill.socket"])
        .status();

    Ok(())
}

fn enable_auto_cpufreq() -> Result<(), String> {
    println!("Enabling auto-cpufreq service...");

    // Check if auto-cpufreq is installed
    let check = Command::new("which")
        .arg("auto-cpufreq")
        .status()
        .map_err(|e| format!("Failed to check for auto-cpufreq: {}", e))?;
    // Йо
    if !check.success() {
        return Err("auto-cpufreq not found. Install with: paru -S auto-cpufreq".to_string());
    }

    // Install and start auto-cpufreq service
    let status = Command::new("auto-cpufreq")
        .args(["--install"])
        .status()
        .map_err(|e| format!("Failed to run auto-cpufreq: {}", e))?;

    if !status.success() {
        return Err("Failed to install auto-cpufreq service".to_string());
    }

    println!("auto-cpufreq enabled successfully");
    Ok(())
}

pub fn configure_laptop() -> Result<(), Box<dyn std::error::Error>> {
    if settings::is_tlp_enabled() {
        enable_tlp().map_err(std::io::Error::other)?;
    }

    if settings::is_auto_cpufreq_enabled() {
        enable_auto_cpufreq().map_err(std::io::Error::other)?;
    }

    laptop_mode()?;
    Ok(())
}

// Im stupid asf
