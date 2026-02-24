use std::path::Path;

pub mod parser;
pub mod settings;
pub mod structs;

// Re-export commonly used items
pub use parser::ConfigParser;
pub use structs::Config;

pub fn init() {
    let parser = ConfigParser::new();
    let _ = parser.load_from_default_and_apply();
}

pub fn init_with_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parser = ConfigParser::new();

    // Expand ~ if needed
    let path = if path.starts_with("~/") {
        let home = std::env::var("HOME")?;
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    };

    if Path::new(&path).exists() {
        parser.load_and_apply(&path)?;
    } else {
        println!("Config file not found: {}, using defaults", path);
        // Create empty config
        let empty_config = Config::default();
        parser.apply_config(empty_config);
    }

    Ok(())
}
