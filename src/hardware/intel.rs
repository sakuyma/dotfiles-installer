use std::process::Command;
use crate::cli::formatter::*;

// Check if we're root
// because Intel drivers need admin privileges (they're fancy like that)
fn is_root() -> bool {
    std::env::var("USER").map(|u| u == "root").unwrap_or(false)
}

// Ask pacman if a package exists
fn check_package_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(vec!["-Q", pkg])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// Install Intel drivers - the blue team special
// (these actually work out of the box, unlike some others)
fn install_drivers() -> Result<bool, Box<dyn std::error::Error>> {
    print_progress("Checking Intel drivers... (the reliable ones)");

    // The sacred Intel packages
    // (they've been around since the dawn of time)
    let packages = vec![
        "mesa",
        "lib32-mesa",
        "xf86-video-intel",
        "vulkan-intel",
        "lib32-vulkan-intel",
    ];

    // See what's already there (probably everything, Intel users are lucky)
    let mut all_installed = true;
    let mut missing_packages = Vec::new();

    for &pkg in &packages {
        if !check_package_installed(pkg) {
            all_installed = false;
            missing_packages.push(pkg);
        }
    }

    if all_installed {
        print_success("Intel drivers already installed (typical Intel - already there when you need them)");
        return Ok(true);
    }

    print_progress(&format!(
        "Installing Intel drivers: {:?} (boring but functional)",
        missing_packages
    ));

    // Call pacman (the friendly neighborhood package manager)
    let status = Command::new("pacman")
        .args(vec![
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
        print_success("Intel drivers installed! They work so well you'll forget they're there");
        Ok(true)
    } else {
        Err(format!("pacman had a moment: {}", status).into())
    }
}

// The main event - Intel driver installation (the boring, reliable kind)
pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    print_progress("Setting up Intel drivers... (it's gonna be uneventful, promise)");

    // Root check - because even Intel needs permissions sometimes
    if !is_root() {
        return Err("Root privileges required. Intel doesn't work for free (but almost)".into());
    }

    // Install the drivers (they'll probably just work)
    match install_drivers() {
        Ok(true) => {
            print_success("Intel drivers setup complete! Your graphics will now be... adequate");
            println!("You can reboot, or not, Intel doesn't care either way");
            Ok(())
        }
        Ok(false) => {
            print_warning("Nothing changed (Intel was already perfect)");
            Ok(())
        }
        Err(e) => {
            print_error(&format!("Intel drivers failed to install (this never happens): {}", e));
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_package_installed() {
        // Testing with something that definitely doesn't exist
        // (unlike Intel's market presence)
        assert!(!check_package_installed("intel-owns-the-world-12345"));
    }

    #[test]
    fn test_is_root() {
        // Probably false unless you're running tests as root (you madman)
        let _ = is_root();
    }
}
