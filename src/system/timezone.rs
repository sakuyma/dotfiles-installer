use super::utils;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn set_timezone(timezone: &str) -> Result<(), String> {
    if !utils::check_root() {
        return Err("Root privileges required. Run with sudo.".into());
    }

    if timezone.is_empty() {
        return Err("Timezone cannot be empty".into());
    }

    let zoneinfo_path = format!("/usr/share/zoneinfo/{}", timezone);
    let zoneinfo_file = Path::new(&zoneinfo_path);
    let localtime_path = Path::new("/etc/localtime");

    if !zoneinfo_file.exists() {
        return Err(format!("Timezone '{}' not found in /usr/share/zoneinfo/", timezone));
    }

    // if /etc/localtime alr exists, delete it
    if localtime_path.exists() {
        fs::remove_file(localtime_path)
            .map_err(|e| format!("Failed to remove existing /etc/localtime: {}", e))?;
    }

    // Create new symlink
    std::os::unix::fs::symlink(zoneinfo_file, localtime_path)
        .map_err(|e| format!("Failed to create symlink: {}", e))?;

    let status = Command::new("hwclock")
        .arg("--systohc")
        .status()
        .map_err(|e| format!("Failed to execute hwclock: {}", e))?;

    if !status.success() {
        return Err("hwclock failed to sync hardware clock".into());
    }

    println!("Timezone set to: {}", timezone);
    Ok(())
}
