mod args;
pub mod commands;
mod formatter;
mod handlers;
mod validator;

// Re-export everything needed
pub use args::Args;
pub use formatter::*;
pub use handlers::{handle_subcommand, resolve_config_path};
pub use validator::validate_args;

use clap::Parser;

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}
