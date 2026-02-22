use std::process::Command;

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

/// Install Intel drivers using pacman
fn install_drivers() -> Result<bool, Box<dyn std::error::Error>> {
    println!("Checking Intel drivers...");
    
    // List of Intel driver packages
    let packages = [
        "mesa",
        "lib32-mesa", 
        "xf86-video-intel",
        "vulkan-intel",
        "lib32-vulkan-intel",
    ];
    
    // Check if all packages are already installed
    let mut all_installed = true;
    let mut missing_packages = Vec::new();
    
    for &pkg in &packages {
        if !check_package_installed(pkg) {
            all_installed = false;
            missing_packages.push(pkg);
        }
    }
    
    if all_installed {
        println!("All Intel drivers are already installed");
        return Ok(true);
    }
    
    println!("Installing Intel drivers: {:?}", missing_packages);
    
    let status = Command::new("pacman")
        .args(&[
            "-S",
            "--noconfirm",
            "--needed",
            "mesa",
            "lib32-mesa",
            "xf86-video-intel",
            "vulkan-intel",
            "lib32-vulkan-intel",
        ])
        .status()?;
    
    if status.success() {
        println!("Intel drivers installed successfully");
        Ok(true)
    } else {
        Err(format!("pacman failed with error code: {}", status).into())
    }
}

/// Main setup function for Intel drivers
pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up Intel drivers...");
    
    // Check if running as root
    if !is_root() {
        return Err("This command requires root privileges. Run with sudo".into());
    }
    
    // Install drivers
    match install_drivers() {
        Ok(true) => {
            println!("Intel drivers setup completed successfully!");
            println!("You may need to reboot for changes to take effect");
            Ok(())
        }
        Ok(false) => {
            println!("No changes were made");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error installing Intel drivers: {}", e);
            Err(e)
        }
    }
}
