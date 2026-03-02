use super::utils;
use std::fs::File;
use std::path::Path;

pub fn hostname(hostname: String) -> Result<(), String> {
    let hostname_path = Path::new("/etc/hostname");
    let mut file = File::create(hostname_path);

    if !utils::check_root() {
        return Err("Run with sudo".into());
    }
    if hostname.is_empty() {
       return Err("Hostname cannot be emty".into()) 
    }
    
    fs::write("/etc/hostname", format!("{}\n", hostname))
        .map_err(|e| format!("Failed to write hostname: {}", e))?;
    Ok(())
}
