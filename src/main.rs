#![allow(dead_code)]

mod config;
mod dotfiles_manager;
mod hardware;
mod packages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting installing");
    config::init();

    dotfiles_manager::clone::clone_repo()?;

    dotfiles_manager::install::stow_config()?;

    if hardware::utils::is_laptop() {
        dotfiles_manager::laptop::laptop_mode()?;
    }

    hardware::videocard::setup_driver()?;

    let groups = vec!["all".to_string()];
    packages::install::install_all(&groups)?;

    Ok(())
}
