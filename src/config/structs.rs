use crate::config::parser::Parser;
use crate::config::structs::{Config, GitConfig, LaptopConfig, SystemConfig, Packages, Dependencies};
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init() {
    let default_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".config/dotfiles-installer/config.conf");
    if default_path.exists() {
        let _ = init_with_path(default_path.to_str().unwrap());
    } else {
        let _ = CONFIG.set(Config::default());
    }
}

pub fn init_with_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Expand ~ if needed
    let path = if path.starts_with("~/") {
        let home = std::env::var("HOME")?;
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    };

    let path = Path::new(&path);
    if !path.exists() {
        let _ = CONFIG.set(Config::default());
        return Ok(());
    }

    let mut parser = Parser::new();
    let raw_data = parser.parse_file(path)?;
    let config = convert_to_config(raw_data)?;

    CONFIG
        .set(config)
        .map_err(|_| "Config already initialized".to_string())?;

    Ok(())
}

fn convert_to_config(mut raw: HashMap<String, parser::Value>) -> Result<Config, String> {
    let mut git = GitConfig::default();
    let mut laptop = LaptopConfig::default();
    let mut system = SystemConfig::default();
    let mut packages = HashMap::new();
    let mut dependencies = HashMap::new();

    for (key, value) in raw.drain() {
        if key.starts_with("git.") {
            match key.as_str() {
                "git.repo" => git.repo = extract_string(value)?,
                "git.branch" => git.branch = extract_string(value)?,
                "git.dotfiles_path" => git.dotfiles_path = extract_string(value)?,
                _ => eprintln!("Warning: unknown git key: {}", key),
            }
        } else if key.starts_with("laptop.") {
            match key.as_str() {
                "laptop.enable_tlp" => laptop.enable_tlp = extract_bool(value)?,
                "laptop.enable_auto_cpufreq" => laptop.enable_auto_cpufreq = extract_bool(value)?,
                _ => eprintln!("Warning: unknown laptop key: {}", key),
            }
        } else if key.starts_with("system.") {
            match key.as_str() {
                "system.hostname" => system.hostname = Some(extract_string(value)?),
                "system.timezone" => system.timezone = Some(extract_string(value)?),
                "system.locale" => system.locale = Some(extract_string(value)?),
                _ => eprintln!("Warning: unknown system key: {}", key),
            }
        } else if key.starts_with("packages.") {
            let group_name = key.trim_start_matches("packages.").to_string();
            let list = extract_list(value)?;
            packages.insert(group_name, list);
        } else if key.starts_with("dependencies.") {
            let group_name = key.trim_start_matches("dependencies.").to_string();
            let list = extract_list(value)?;
            dependencies.insert(group_name, list);
        } else {
            eprintln!("Warning: unknown key: {}", key);
        }
    }

    Ok(Config {
        git,
        laptop,
        system,
        packages: Packages { groups: packages },
        dependencies: Dependencies { groups: dependencies },
    })
}

fn extract_string(value: parser::Value) -> Result<String, String> {
    match value {
        parser::Value::String(s) => Ok(s),
        _ => Err(format!("Expected string, got {:?}", value)),
    }
}

fn extract_bool(value: parser::Value) -> Result<bool, String> {
    match value {
        parser::Value::String(s) if s == "true" => Ok(true),
        parser::Value::String(s) if s == "false" => Ok(false),
        _ => Err(format!("Expected boolean, got {:?}", value)),
    }
}

fn extract_list(value: parser::Value) -> Result<Vec<String>, String> {
    match value {
        parser::Value::List(v) => Ok(v),
        _ => Err(format!("Expected list, got {:?}", value)),
    }
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}
