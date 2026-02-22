use crate::hardware::{amd, intel, nvidia};
use gfxinfo::active_gpu;

// try to find out what gpu the user has
// this is basically russian roullet
fn what_vendor() -> String {
    match active_gpu() {
        Ok(gpu) => {
            // We got answer
            gpu.vendor().to_string()
        }
        Err(_) => {
            // could be integrated graphics,
            // could be the user running this in vm
            // could be system just being wierd
            return String::from("Unknown vendor");
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
            nvidia::setup();
            Ok(())
        }
        "Amd" => {
            amd::setup();
            Ok(())
        }
        "Intel" => {
            intel::setup();
            Ok(())
        }
        // If we get here, either:
        // a) User has some exotic GPU
        // b) gfxinfo doesn't recognize it
        // c) They're running in a VM and this whole thing was doomed anyway
        _unknown => {
            eprintln!("Unknown Gpu: {}", vendor);
            Ok(())
        }
    }
}
