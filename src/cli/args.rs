use super::commands;
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

    /// Dry run (show what would be done without actually doing it)
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// Assume yes to all prompts (non-interactive mode)
    #[arg(short = 'y', long)]
    pub assume_yes: bool,

    /// Interactive mode - ask before each step
    #[arg(short, long)]
    pub interactive: bool,

    // Enable logging in cosole with timestamps
    #[arg(long)]
    pub log: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List available package groups
    List(commands::list::ListArgs),

    /// Install specific packages without groups
    Install(commands::install::InstallArgs),

    /// Remove packages
    Remove(commands::remove::RemoveArgs),

    /// Create example config file
    Init(commands::init::InitArgs),
}
