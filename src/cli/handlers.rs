use super::args::{Args, Commands};
use super::commands::{list, install, remove, init};
use std::env;

pub fn handle_subcommand(cmd: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Commands::List(c) => list::execute(c),
        Commands::Install(c) => install::execute(c),
        Commands::Remove(c) => remove::execute(c),
        Commands::Init(c) => init::execute(c),
    }
}

pub fn resolve_config_path(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    let path = args.config
        .as_deref()
        .unwrap_or("~/.config/dotfiles-installer/config.toml");
    
    if path.starts_with("~/") {
        let home = env::var("HOME")?;
        Ok(path.replacen("~", &home, 1))
    } else {
        Ok(path.to_string())
    }
}
