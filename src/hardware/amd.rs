use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

static CONFIG_PATH: &str = "/etc/mkinitcpio.conf";
static MODULES: &str = "amdgpu radeon";

/// Check if program is running as root
fn is_root() -> bool {
    std::env::var("USER").map(|u| u == "root").unwrap_or(false)
}

/// Check if a package is already installed
fn check_package_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(&["-Q", pkg])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Install AMD drivers using pacman
fn install_drivers() -> Result<bool, Box<dyn std::error::Error>> {
    println!("Checking AMD drivers...");

    // Check if drivers are already installed
    let packages = ["mesa", "lib32-mesa", "vulkan-radeon", "lib32-vulkan-radeon"];
    let mut all_installed = true;

    for pkg in &packages {
        if !check_package_installed(pkg) {
            all_installed = false;
            break;
        }
    }

    if all_installed {
        println!("AMD drivers already installed");
        return Ok(true);
    }

    println!("Installing AMD drivers...");
    let status = Command::new("pacman")
        .args(&[
            "-S",
            "--noconfirm",
            "--needed",
            "mesa",
            "lib32-mesa",
            "vulkan-radeon",
            "lib32-vulkan-radeon",
        ])
        .status()?;

    if status.success() {
        println!("AMD drivers installed successfully");
        Ok(true)
    } else {
        Err(format!("pacman failed with error code: {}", status).into())
    }
}

/// Create backup of mkinitcpio.conf before modifying
fn backup_config(path: &Path) -> io::Result<()> {
    let backup_path = path.with_extension("conf.bak");
    if !backup_path.exists() {
        fs::copy(path, &backup_path)?;
        println!("Backup created: {:?}", backup_path);
    } else {
        println!("Backup already exists: {:?}", backup_path);
    }
    Ok(())
}

/// Add AMD modules to mkinitcpio.conf and rebuild initramfs
fn mkinitcpio() -> io::Result<bool> {
    let config_path = Path::new(CONFIG_PATH);
    let new_modules = MODULES;

    // Check if config file exists
    if !config_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Config file {} not found", CONFIG_PATH),
        ));
    }

    // Create backup before modifying
    backup_config(config_path)?;

    // Read current config
    let content = fs::read_to_string(config_path)?;
    let mut lines: Vec<String> = Vec::new();
    let mut modified = false;

    // Parse each line and add modules if needed
    for line in content.lines() {
        if line.trim_start().starts_with("MODULES=") && line.contains('(') {
            if let Some(start) = line.find('(') {
                if let Some(end) = line.rfind(')') {
                    let before = &line[..start + 1];
                    let existing = &line[start + 1..end];
                    let after = &line[end..];

                    // Check if modules are already present
                    let new_inside = if existing.trim().is_empty() {
                        new_modules.to_string()
                    } else if existing.contains("amdgpu") && existing.contains("radeon") {
                        println!("AMD modules already present in config");
                        existing.to_string() // Already there, don't change
                    } else if existing.contains("amdgpu") {
                        format!("{} radeon", existing)
                    } else if existing.contains("radeon") {
                        format!("amdgpu {}", existing)
                    } else {
                        format!("{} {}", existing, new_modules)
                    };

                    // Only mark as modified if something actually changed
                    if new_inside != existing {
                        let new_line = format!("{}{}{}", before, new_inside, after);
                        lines.push(new_line);
                        modified = true;
                        println!("Adding AMD modules to MODULES line");
                    } else {
                        lines.push(line.to_string());
                    }
                    continue;
                }
            }
        }
        lines.push(line.to_string());
    }

    // Write changes if modified
    if modified {
        fs::write(config_path, lines.join("\n"))?;
        println!("mkinitcpio.conf updated");

        // Verify that modules were added
        let new_content = fs::read_to_string(config_path)?;
        if !new_content.contains("amdgpu") || !new_content.contains("radeon") {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to verify modules were added to config",
            ));
        }
        println!("Verified modules were added correctly");

        // Rebuild initramfs
        println!("Rebuilding initramfs...");
        let status = Command::new("mkinitcpio").arg("-P").status().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to run mkinitcpio: {}", e),
            )
        })?;

        if status.success() {
            println!("Initramfs rebuilt successfully");
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to rebuild initramfs",
            ));
        }
    } else {
        println!("No changes needed for mkinitcpio.conf");
    }

    Ok(modified)
}

/// Main setup function for AMD drivers
pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up AMD drivers...");

    // Check if running as root
    if !is_root() {
        return Err("This command requires root privileges. Run with sudo".into());
    }

    // Install drivers
    install_drivers()?;

    // Configure mkinitcpio
    mkinitcpio()?;

    println!("\nAMD drivers setup completed successfully!");
    println!(" Reboot your system to apply changes");

    Ok(())
}
