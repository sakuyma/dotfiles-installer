#![allow(dead_code)]

mod dotfiles_manager;
mod hardware;
mod packages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting installing");

    dotfiles_manager::clone::clone_repo()?;

    dotfiles_manager::install::stow_config()?;

    if hardware::utils::is_laptop() {
        dotfiles::laptop::laptop_mode()?;
    }

    hardware::videocard::setup_driver()?;

    let groups = vec!["all"];
    packages::install::install_all(&groups)?;

    Ok(())
}
