#![allow(dead_code)]

mod configs;
mod hardware;
mod packages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting installing");

    configs::clone::clone_repo()?;

    configs::install::stow_config()?;

    if hardware::hardware::is_laptop() {
        configs::laptop::laptop_mode()?;
    }
    
    hardware::videocard::setup_driver()?;

    let groups = vec!["all"];
    packages::install::install_all(&groups)?;

    Ok(())
}
