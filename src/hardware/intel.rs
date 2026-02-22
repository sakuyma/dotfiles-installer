use std::process::Command;

fn install_drivers() {
    let cmd = format!("sudo pacman -S --noconfirm --needed");
    let status = Command::new("pacman")
        .args(&[
            "-S",
            "--noconfirm",
            "--needed",
            "mesa",
            "lib32-mesa",
            "xf86-video-intel",
            "lib32-vulcan-intel",
        ])
        .status()?;
    Ok(status.success())
}

pub fn setup() {
    install_drivers();
}
