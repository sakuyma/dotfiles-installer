use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

static CONFIG_PATH: &str = "/etc/mkinitcpio.conf";
static MODULES: &str = "amdgpu radeon";

// Check if we're root
// because touching system files without permission is rude
fn is_root() -> bool {
    std::env::var("USER").map(|u| u == "root").unwrap_or(false)
}

// Ask pacman politely if a package exists
// (pacman has mood swings, but it's usually honest)
fn check_package_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(vec!["-Q", pkg])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// The AMD experience - installing open source drivers that actually work
// (unlike some other GPU vendors *cough* NVIDIA *cough*)
fn install_drivers() -> Result<bool, Box<dyn std::error::Error>> {
    println!("Checking AMD drivers... (the open source ones, because AMD is cool like that)");

    // The holy list of AMD packages
    // (all open source, all free, all glorious)
    let packages = vec!["mesa", "lib32-mesa", "vulkan-radeon", "lib32-vulkan-radeon"];

    // Check what's already installed (hopefully everything)
    let mut all_installed = true;
    let mut missing_packages = Vec::new();

    for &pkg in &packages {
        if !check_package_installed(pkg) {
            all_installed = false;
            missing_packages.push(pkg);
        }
    }

    if all_installed {
        println!("AMD drivers already installed (someone has good taste)");
        return Ok(true);
    }

    println!(
        "Installing AMD drivers: {:?} (the way Linus intended)",
        missing_packages
    );

    // Summon pacman (the friendly version, not the video game character)
    let status = Command::new("pacman")
        .args(vec![
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
        println!("AMD drivers installed! Your GPU is now free as in freedom");
        Ok(true)
    } else {
        Err(format!("pacman had an existential crisis: {}", status).into())
    }
}

// Create a backup because we're responsible adults (sometimes)
fn backup_config(path: &Path) -> io::Result<()> {
    let backup_path = path.with_extension("conf.bak");
    if !backup_path.exists() {
        fs::copy(path, &backup_path)?;
        println!("Backup saved at {:?} (safety first!)", backup_path);
    } else {
        println!("Backup already exists (we're consistent, if nothing else)");
    }
    Ok(())
}

// Add AMD modules to mkinitcpio.conf
// Warning: contains string manipulation that would make a CS professor cry
fn mkinitcpio() -> io::Result<bool> {
    let config_path = Path::new(CONFIG_PATH);
    let new_modules = MODULES;

    // Does the file exist? It should. It really should.
    if !config_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Config file {} is playing hide and seek", CONFIG_PATH),
        ));
    }

    // Backup in case we mess up (we probably won't, but still)
    backup_config(config_path)?;

    // Read the ancient scrolls
    let content = fs::read_to_string(config_path)?;
    let mut lines: Vec<String> = Vec::new();
    let mut modified = false;

    // Parse each line like it's 1995 and we're writing HTML by hand
    for line in content.lines() {
        if line.trim_start().starts_with("MODULES=")
            && line.contains('(')
            && let (Some(start), Some(end)) = (line.find('('), line.rfind(')'))
        {
            let before = &line[..start + 1];
            let existing = &line[start + 1..end];
            let after = &line[end..];

            // Check if AMD modules are already partying in there
            let new_inside = if existing.trim().is_empty() {
                new_modules.to_string()
            } else if existing.contains("amdgpu") && existing.contains("radeon") {
                println!("AMD modules already living rent-free in config");
                existing.to_string() // Don't evict them
            } else if existing.contains("amdgpu") {
                format!("{} radeon", existing)
            } else if existing.contains("radeon") {
                format!("amdgpu {}", existing)
            } else {
                format!("{} {}", existing, new_modules)
            };

            // Did we actually change something or just move furniture around?
            if new_inside != existing {
                let new_line = format!("{}{}{}", before, new_inside, after);
                lines.push(new_line);
                modified = true;
                println!("Adding AMD modules to MODULES (they brought friends)");
            } else {
                lines.push(line.to_string());
            }
        } else {
            lines.push(line.to_string());
        }
    }

    // Commit changes to the system (scary stuff)
    if modified {
        fs::write(config_path, lines.join("\n"))?;
        println!("mkinitcpio.conf has been updated (the system will never know)");

        // Double-check our work like a paranoid sysadmin
        let new_content = fs::read_to_string(config_path)?;
        if !new_content.contains("amdgpu") || !new_content.contains("radeon") {
            return Err(io::Error::other(
                "Failed to add AMD modules (the config is being stubborn)",
            ));
        }
        println!("Verified AMD modules are now in the building");

        // Rebuild initramfs - the moment of truth
        println!("Rebuilding initramfs... (time to question your life choices)");
        let status = Command::new("mkinitcpio")
            .arg("-P")
            .status()
            .map_err(|e| io::Error::other(format!("mkinitcpio said no: {}", e)))?;

        if status.success() {
            println!("Initramfs rebuilt! Your kernel now knows about AMD");
            Ok(true)
        } else {
            Err(io::Error::other(
                "mkinitcpio failed (blame the kernel, not us)",
            ))
        }
    } else {
        println!("No changes to mkinitcpio.conf (it was already perfect)");
        Ok(false)
    }
}

// The main event - AMD driver installation extravaganza
pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting AMD driver installation (the red team awaits)");

    // Root check - because we're not animals
    if !is_root() {
        return Err("Root privileges required. Use sudo like a responsible adult".into());
    }

    // Install the good stuff
    install_drivers()?;

    // Configure the system (magic happens here)
    mkinitcpio()?;

    Ok(())
}
