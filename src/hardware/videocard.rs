use crate::hardware::{amd, intel, nvidia};

// try to find out what gpu the user has
// this is basically russian roullet
fn what_vendor() -> String {
    // Try lspci first (more reliable)
    if let Ok(output) = std::process::Command::new("lspci").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.to_lowercase().contains("nvidia") {
            return "Nvidia".to_string();
        } else if output_str.to_lowercase().contains("amd")
            || output_str.to_lowercase().contains("radeon")
        {
            return "Amd".to_string();
        } else if output_str.to_lowercase().contains("intel") {
            return "Intel".to_string();
        }
    }

    // Fallback to gfxinfo
    match gfxinfo::active_gpu() {
        Ok(gpu) => {
            // We got answer
            gpu.vendor().to_string()
        }
        Err(_) => {
            // could be integrated graphics,
            // could be the user running this in vm
            // could be system just being wierd
            String::from("Unknown vendor")
        }
    }
}

pub fn setup_driver() -> Result<(), Box<dyn std::error::Error>> {
    // Actually get the GPU vendor this time (revolutionary idea, I know)
    let vendor = what_vendor();

    // Now install drivers based on what we found
    // (GPU vendors are inconsistent with capitalization, so good luck)
    match vendor.as_str() {
        "Nvidia" => {
            nvidia::setup()?;
            Ok(())
        }
        "Amd" => {
            amd::setup()?;
            Ok(())
        }
        "Intel" => {
            intel::setup()?;
            Ok(())
        }
        // If we get here, either:
        // a) User has some exotic GPU
        // b) gfxinfo doesn't recognize it
        // c) They're running in a VM and this whole thing was doomed anyway
        _ => {
            eprintln!("Unknown Gpu: {}", vendor);
            Ok(())
        }
    }
}
