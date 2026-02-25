mod args;
mod validator;
pub mod formatter;
pub mod commands;
mod handlers;
mod interactive;
mod prompt_manager;

// Re-export only what's actually used
pub use args::Args;
pub use handlers::{handle_subcommand, resolve_config_path};
pub use validator::validate_args;
pub use prompt_manager::PromptManager;

use clap::Parser;

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}
