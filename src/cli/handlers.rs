use super::args::Commands;
use super::commands::{init, install, list, remove};

pub fn handle_subcommand(cmd: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Commands::List(c) => list::execute(c),
        Commands::Install(c) => install::execute(c),
        Commands::Remove(c) => remove::execute(c),
        Commands::Init(c) => init::execute(c),
    }
}
