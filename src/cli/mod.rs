mod args;
pub mod commands;
mod formatter;
mod handlers;
mod validator;

pub use args::*;
pub use formatter::*;
pub use handlers::{handle_subcommand, resolve_config_path};
pub use validator::validate_args;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Dotfiles installer for Arch Linux", long_about = None)]
pub struct Args {
    /// Config file path
    #[arg(short, long)]
    pub config: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Package groups to install (can be used multiple times)
    #[arg(short, long = "group")]
    pub groups: Vec<String>,

    /// Skip package installation
    #[arg(long)]
    pub skip_packages: bool,

    /// Skip dotfiles setup
    #[arg(long)]
    pub skip_dotfiles: bool,

    /// Skip hardware configuration
    #[arg(long)]
    pub skip_hardware: bool,

    /// Force reinstall even if already installed
    #[arg(short, long)]
    pub force: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available package groups
    List(ListArgs),

    /// Install specific packages without groups
    Install(InstallArgs),

    /// Remove packages
    Remove(RemoveArgs),

    /// Create example config file
    Init(InitArgs),
}

#[derive(clap::Args, Debug)]
pub struct ListArgs {
    /// Show only AUR packages
    #[arg(long)]
    pub aur: bool,

    /// Show only pacman packages
    #[arg(long)]
    pub pacman: bool,
}

#[derive(clap::Args, Debug)]
pub struct InstallArgs {
    /// Package names to install
    pub packages: Vec<String>,

    /// Install from AUR
    #[arg(long)]
    pub aur: bool,
}

#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    /// Package names to remove
    pub packages: Vec<String>,
}

#[derive(clap::Args, Debug)]
pub struct InitArgs {
    /// Path to create config file
    #[arg(default_value = "config.example.toml")]
    pub path: String,
}
