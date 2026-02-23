use std::process::Command;

fn install_drivers() -> Result<bool, Box<dyn std::error::Error>> {
    let status = Command::new("pacman")
        .args(&[
            "-S",
            "--noconfirm",
            "--needed",
            "mesa",
            "lib32-mesa",
            "xf86-video-intel",
            "vulkan-intel",
            "lib32-vulcan-intel",
        ])
        .status()?;
    Ok(status.success())
}

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let _ = install_drivers();
    Ok(())
}
