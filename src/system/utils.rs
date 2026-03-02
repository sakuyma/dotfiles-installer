use std::env;

pub fn check_root() -> Result<(), String> {
    match env::var("USER") {
        Ok(user) if user == "root" => Ok(()),
        Ok(_) => Err("This program must be run as root. Use sudo.".into()),
        Err(_) => Err("Could not determine current user".into()),
    }
}
