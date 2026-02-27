use crate::config::parser::{self, Parser};
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

// ==================== CONFIG STRUCTURES ====================

/// Git repository settings
#[derive(Debug, Clone, Default)]
pub struct GitConfig {
    /// URL of the dotfiles repository
    pub repo: Option<String>,
    /// Git branch to use (defaults to main/master if None)
    pub branch: Option<String>,
    /// Local path where dotfiles will be cloned
    pub dotfiles_path: Option<String>,
}

/// Laptop-specific settings
#[derive(Debug, Clone, Default)]
pub struct LaptopConfig {
    /// Whether to enable TLP power management
    pub enable_tlp: bool,
    /// Whether to enable auto-cpufreq for dynamic frequency scaling
    pub enable_auto_cpufreq: bool,
}

/// System-wide configuration
#[derive(Debug, Clone, Default)]
pub struct SystemConfig {
    /// System hostname
    pub hostname: Option<String>,
    /// System timezone (e.g., "Europe/Moscow")
    pub timezone: Option<String>,
    /// System locale (e.g., "en_US.UTF-8")
    pub locale: Option<String>,
}

/// Package groups collection
#[derive(Debug, Clone, Default)]
pub struct Packages {
    /// Map of group name -> list of packages
    pub groups: HashMap<String, Vec<String>>,
}

/// Dependencies between package groups
#[derive(Debug, Clone, Default)]
pub struct Dependencies {
    /// Map of group name -> list of dependencies (other group names)
    pub groups: HashMap<String, Vec<String>>,
}

/// Main configuration structure
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub git: GitConfig,
    pub laptop: LaptopConfig,
    pub system: SystemConfig,
    pub packages: Packages,
    pub dependencies: Dependencies,
}

// ==================== GLOBAL STORAGE ====================

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Initialize config with default path (~/.config/dotfiles-installer/config.conf)
pub fn init() {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Warning: HOME environment variable not found, using current directory");
            ".".to_string()
        }
    };

    let default_path = Path::new(&home).join(".config/dotfiles-installer/config.conf");

    if default_path.exists() {
        let _ = init_with_path(default_path.to_str().unwrap());
    } else {
        let _ = CONFIG.set(Config::default());
    }
}

/// Initialize config from specific path
pub fn init_with_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Expand tilde (~) to home directory if present
    let path = if path.starts_with("~/") {
        let home = std::env::var("HOME")?;
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    };

    let path = Path::new(&path);
    if !path.exists() {
        eprintln!("Config file not found: {}, using defaults", path.display());
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

/// Convert raw parsed data into structured Config
fn convert_to_config(mut raw: HashMap<String, parser::Value>) -> Result<Config, String> {
    let mut git = GitConfig::default();
    let mut laptop = LaptopConfig::default();
    let mut system = SystemConfig::default();
    let mut packages = HashMap::new();
    let mut dependencies = HashMap::new();

    for (key, value) in raw.drain() {
        if key.starts_with("git.") {
            match key.as_str() {
                "git.repo" => git.repo = Some(extract_string(value)?),
                "git.branch" => git.branch = Some(extract_string(value)?),
                "git.dotfiles_path" => git.dotfiles_path = Some(extract_string(value)?),
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
        dependencies: Dependencies {
            groups: dependencies,
        },
    })
}

/// Extract string value from parsed value
fn extract_string(value: parser::Value) -> Result<String, String> {
    match value {
        parser::Value::String(s) => Ok(s),
        _ => Err(format!("Expected string, got {:?}", value)),
    }
}

/// Extract boolean value from parsed value
fn extract_bool(value: parser::Value) -> Result<bool, String> {
    match value {
        parser::Value::String(s) if s == "true" => Ok(true),
        parser::Value::String(s) if s == "false" => Ok(false),
        _ => Err(format!("Expected boolean (true/false), got {:?}", value)),
    }
}

/// Extract list value from parsed value
fn extract_list(value: parser::Value) -> Result<Vec<String>, String> {
    match value {
        parser::Value::List(v) => Ok(v),
        _ => Err(format!("Expected list, got {:?}", value)),
    }
}

/// Get global config instance
pub fn get_config() -> &'static Config {
    CONFIG
        .get()
        .expect("Config not initialized. Call init() or init_with_path() first")
}
