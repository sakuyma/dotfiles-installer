mod structs;
mod parser;
pub mod settings;

// Re-export commonly used items
pub use structs::*;
pub use parser::ConfigParser;

pub fn init() {
    let parser = ConfigParser::new();
    parser.load_from_default_locations_and_apply();
}
