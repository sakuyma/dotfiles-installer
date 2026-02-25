mod args;
pub mod commands;
pub mod formatter;
mod handlers;
mod interactive;
mod prompt_manager;
mod validator;

// Re-export only what's actually used
pub use args::Args;
pub use handlers::{handle_subcommand, resolve_config_path};
pub use prompt_manager::PromptManager;
pub use validator::validate_args;

use clap::Parser;

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}
