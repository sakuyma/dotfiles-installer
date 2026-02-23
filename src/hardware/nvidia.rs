use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

static CONFIG_PATH: &str = "/etc/mkinitcpio.conf";
static MODULES: &str = "i915 nvidia nvidia_modeset nvidia_uvm nvidia_drm";

// Check if we're root
// because touching system files as a mortal is a bad idea
fn is_root() -> bool {
    std::env::var("USER").map(|u| u == "root").unwrap_or(false)
}

// Ask pacman nicely if a package exists
// (pacman is usually grumpy but answers sometimes)
fn check_package_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(vec!["-Q", pkg])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// The main event - installing NVIDIA NOT proprietary blob
// May cause: eye candy, heating issues, or existential dread
fn install_drivers() -> Result<bool, Box<dyn std::error::Error>> {
    println!("Checking NVIDIA drivers... (please don't explode)");
    
    // The sacred list of NVIDIA packages
    // (may or may not actually work with your GPU)
    let packages = vec![
        "nvidia-open",
        "nvidia-utils",
        "lib32-nvidia-utils",
        "egl-wayland",
    ];
    
    // Check what's already there (probably nothing if you're on Intel)
    let mut all_installed = true;
    let mut missing_packages = Vec::new();
    
    for &pkg in &packages {
        if !check_package_installed(pkg) {
            all_installed = false;
            missing_packages.push(pkg);
        }
    }
    
    if all_installed {
        println!("NVIDIA drivers already installed (someone beat us to it)");
        return Ok(true);
    }
    
    println!(
        "Installing NVIDIA drivers: {:?} (fingers crossed)",
        missing_packages
    );
    
    // Summon the pacman demon
    let status = Command::new("pacman")
        .args(vec![
            "-S",
            "--noconfirm",
            "--needed",
            "nvidia-open",
            "nvidia-utils",
            "lib32-nvidia-utils",
            "egl-wayland",
        ])
        .status()?;
    
    if status.success() {
        println!("NVIDIA drivers installed! Your GPU may now demand more fans");
        Ok(true)
    } else {
        Err(format!("pacman cried with error code: {}", status).into())
    }
}

// Create a backup because we're not COMPLETE psychopaths
fn backup_config(path: &Path) -> io::Result<()> {
    let backup_path = path.with_extension("conf.bak");
    if !backup_path.exists() {
        fs::copy(path, &backup_path)?;
        println!("Backup created at {:?} (you're welcome)", backup_path);
    } else {
        println!("Backup already exists (we're not THAT paranoid)");
    }
    Ok(())
}

// Modify mkinitcpio.conf to include NVIDIA modules
// Warning: contains regex-like string manipulation that definitely won't break
fn mkinitcpio() -> io::Result<bool> {
    let config_path = Path::new(CONFIG_PATH);
    let new_modules = MODULES;
    
    // Check if the file exists (spoiler: it should)
    if !config_path.exists() {
        return Err(io::Error::other(
            format!("Config file {} is missing (how did you even boot?)", CONFIG_PATH)
        ));
    }
    
    // Backup first, regret later
    backup_config(config_path)?;
    
    // Read the ancient text
    let content = fs::read_to_string(config_path)?;
    let mut lines: Vec<String> = Vec::new();
    let mut modified = false;
    
    // Parse each line like it's 1970 and we're reading punch cards
    for line in content.lines() {
        if line.trim_start().starts_with("MODULES=")
            && line.contains('(')
            && let Some(start) = line.find('(')
            && let Some(end) = line.rfind(')')
        {
            let before = &line[..start + 1];
            let existing = &line[start + 1..end];
            let after = &line[end..];
            
            // Check if NVIDIA modules are already haunting this config
            let new_inside = if existing.trim().is_empty() {
                new_modules.to_string()
            } else if existing.contains("nvidia") {
                println!("NVIDIA modules already lurking in config");
                existing.to_string() // Don't touch what's not broken
            } else {
                format!("{} {}", existing, new_modules)
            };
            
            // Did we actually change something?
            if new_inside != existing {
                let new_line = format!("{}{}{}", before, new_inside, after);
                lines.push(new_line);
                modified = true;
                println!("Injecting NVIDIA modules into MODULES line (science!)");
            } else {
                lines.push(line.to_string());
            }
        } else {
            lines.push(line.to_string());
        }
    }
    
    // Write changes if any (brave step)
    if modified {
        fs::write(config_path, lines.join("\n"))?;
        println!("mkinitcpio.conf has been... modified");
        
        // Verify our work (or lack thereof)
        let new_content = fs::read_to_string(config_path)?;
        if !new_content.contains("nvidia") {
            return Err(io::Error::other(
                "Failed to add NVIDIA modules (the config fought back)",
            ));
        }
        println!("Verified modules are now in the matrix");
        
        // Rebuild initramfs - the final boss
        println!("Rebuilding initramfs... (this may take a while, go make tea)");
        let status = Command::new("mkinitcpio")
            .arg("-P")
            .status()
            .map_err(|e| io::Error::other(format!("mkinitcpio refused to run: {}", e)))?;
        
        if status.success() {
            println!("Initramfs rebuilt! The kernel now knows about NVIDIA");
            Ok(true)
        } else {
            Err(io::Error::other(
                "mkinitcpio failed (it's not you, it's them)",
            ))
        }
    } else {
        println!("No changes to mkinitcpio.conf (it was perfect already)");
        Ok(false)
    }
}

// The grand finale - install NVIDIA drivers or die trying
pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to tame the NVIDIA dragon...");
    
    // Root check - because system files are like "no touchy" for regular users
    if !is_root() {
        return Err("You need root powers. Try again with sudo (like a boss)".into());
    }
    
    // Install the drivers (prayer circle recommended)
    install_drivers()?;
    
    // Configure the system (black magic happens here)
    mkinitcpio()?;
    
    println!("\nNVIDIA setup complete! Your GPU is now (probably) working");
    println!("Reboot when you're ready to see if we broke anything");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_check_package_installed() {
        // This should return false because nobody installs this
        assert!(!check_package_installed("definitely-not-a-real-package-12345"));
    }
    
    #[test]
    fn test_is_root() {
        // Test runs as user, so should be false (unless you're running tests as root, you madman)
        let _ = is_root();
    }
}
