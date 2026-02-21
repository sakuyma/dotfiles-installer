use gfxinfo::active_gpu;

fn what_vendor() -> String {
    match active_gpu() {
        Ok(gpu) => {
            let vendor = gpu.vendor();
            return vendor.to_string();
        }
        Err(_) => {
            return String::from("Unknown vendor");
        }
    }
}

fn nvidia_vendor() {}
fn amd_vendor() {}
fn intel_vendor() {}

pub fn setup_driver() {}
