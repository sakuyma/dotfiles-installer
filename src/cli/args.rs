use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "Dotfiles installer for Arch Linux")]
pub struct Cli {
    #[arg(short, long)]
    pub config: Option<String>,

    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    List(ListArgs),
    Install(InstallArgs),
    Remove(RemoveArgs),
    Init(InitArgs),
}
