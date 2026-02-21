use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

static CONFIG_PATH: &str = "/etc/mkinitcpio.conf";
static MODULES: &str = "amdgpu radeon";

fn install_drivers() {
    let cmd = format!("sudo pacman -S --noconfirm --needed");
    let status = Command::new("pacman")
        .args(&[
            "-S",
            "--noconfirm",
            "--needed",
            "mesa",
            "lib32-mesa",
            "vulcan-radeon",
            "lib32-vulcan-radeon",
        ])
        .status()?;
    Ok(status.success())
}

fn mkinitcpio() -> io::Result<bool> {
    let config_path = Path::new(CONFIG_PATH);
    let new_modules = MODULES;

    let content = fs::read_to_string(config_path)?;
    let mut lines: Vec<String> = Vec::new();
    let mut modified = false;

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

    if modified {
        fs::write(config_path, lines.join("\n"))?;
    }

    Ok(modified)
}

pub fn install() {
    install_drivers();
    mkinitcpio();
}
