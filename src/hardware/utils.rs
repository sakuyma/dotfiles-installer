use std::env;
use std::fs;

// Check if system is x86_64 architecture
// Returns true for x86_64, false for everything else
// (including your questionable life choices on ARM)
pub fn get_architecture() -> bool {
    let arch = env::consts::ARCH.to_string();
    if arch == "x86_64" {
        true
    } else {
        eprintln!("ARM detected - I have no idea what you're doing here");
        eprintln!("(seriously, ARM support is not a thing... yet)");
        false
    }
}

// Check if system is a laptop
// This is the most reliable way to detect laptops on Linux.
//
// If a battery is found, congratulations - you have a laptop!
// If not, you either have a desktop or you're in a VM pretending to be a laptop.
pub fn is_laptop() -> bool {
    // Check the Linux power supply directory
    // (this is where Linux keeps info about batteries, AC adapters, etc)
    if let Ok(entries) = fs::read_dir("/sys/class/power_supply/") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Linux usually names batteries as BAT0, BAT1, etc
            // (if they're named something else, good luck with this system)
            if name_str.starts_with("BAT") {
                let mut path = entry.path();
                path.push("type");

                // Double-check that this is actually a Battery and not
                // some other weird power supply device the system invented
                if let Ok(typ) = fs::read_to_string(path) {
                    if typ.trim() == "Battery" {
                        return true;
                    }
                }
            }
        }
    }
    // No battery found - either a desktop or a VM
    // (or the system is using some weird non-standard power setup)
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_laptop() {
        let _ = is_laptop();
    }
}
