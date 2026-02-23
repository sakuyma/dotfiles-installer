mod structs;
mod parser;
pub mod settings;

// Re-export commonly used items
pub use parser::ConfigParser;

pub fn init() {
    let parser = ConfigParser::new();
    let _ = parser.load_from_default_and_apply();
}
