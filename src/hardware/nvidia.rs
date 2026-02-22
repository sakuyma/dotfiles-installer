use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

static CONFIG_PATH: &str = "/etc/mkinitcpio.conf";
static MODULES: &str = "i915 nvidia nvidia_modeset nvidia_uvm nvidia_drm";

fn install_drivers() -> Result<bool, Box<dyn std::error::Error>> {
    let status = Command::new("pacman")
        .args(&[
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
        println!("Driver installed");
        Ok(true)
    } else {
        Err(format!("pacman exit with error: {}", status).into())
    }
}

fn mkinitcpio() -> io::Result<bool> {
    let config_path = Path::new(CONFIG_PATH);
    let backup_path = config_path.with_extension("conf.bak");
    let new_modules = MODULES;
    let content = fs::read_to_string(config_path)?;
    let mut lines: Vec<String> = Vec::new();
    let mut modified = false;

    // Check if config file exists
    if !config_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File {} not found", CONFIG_PATH),
        ));
    }

    // Create backup if it doesn't exist
    if !backup_path.exists() {
        fs::copy(config_path, &backup_path)?;
        println!("Backup created: {:?}", backup_path);
    }

    // Parse config file and add modules
    for line in content.lines() {
        if line.trim_start().starts_with("MODULES=") && line.contains('(') {
            if let Some(start) = line.find('(') {
                if let Some(end) = line.rfind(')') {
                    let before = &line[..start + 1];
                    let existing = &line[start + 1..end];
                    let after = &line[end..];

                    let new_inside = if existing.trim().is_empty() {
                        new_modules.to_string()
                    } else {
                        format!("{} {}", existing, new_modules)
                    };

                    let new_line = format!("{}{}{}", before, new_inside, after);
                    lines.push(new_line);
                    modified = true;
                    continue;
                }
            }
        }
        lines.push(line.to_string());
    }

    // Write changes if modified
    if modified {
        fs::write(config_path, lines.join("\n"))?;

        // Verify that modules were added
        let new_content = fs::read_to_string(config_path)?;
        if !new_content.contains(MODULES.split_whitespace().next().unwrap()) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to add modules to config",
            ));
        }

        println!("Modules added to mkinitcpio.conf");

        // Rebuild initramfs
        let status = Command::new("mkinitcpio")
            .arg("-P")
            .status()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("mkinitcpio: {}", e)))?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to rebuild initramfs",
            ));
        }
    } else {
        println!("Modules already configured");
    }

    Ok(modified)
}

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    install_drivers()?;
    mkinitcpio()?;
    Ok(())
}
