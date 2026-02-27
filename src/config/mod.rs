use crate::config::parser::Parser;
use std::collections::HashMap;
use std::path::Path;

pub mod lexer;
pub mod parser;
pub mod settings;
pub mod structs;

pub fn init() {
    let git = settings::GitSettings::default();
    let laptop = settings::LaptopSettings::default();
    let groups = HashMap::new();
    settings::initialize(git, groups, laptop);
}

pub fn init_with_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = if path.starts_with("~/") {
        let home = std::env::var("HOME")?;
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    };

    let path = Path::new(&path);
    if !path.exists() {
        init();
        return Ok(());
    }

    let mut parser = Parser::new();
    let raw_data = parser.parse_file(path)?;
    build_and_apply_config(raw_data)?;

    Ok(())
}

fn build_and_apply_config(raw: HashMap<String, parser::Value>) -> Result<(), String> {
    let mut git = settings::GitSettings::default();
    let mut laptop = settings::LaptopSettings::default();
    let mut packages_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut dependencies_map: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in raw {
        if key.starts_with("git.") {
            match key.as_str() {
                "git.repo" => git.repo = extract_optional_string(value)?,
                "git.branch" => git.branch = extract_optional_string(value)?,
                "git.dotfiles_path" => git.dotfiles_path = extract_optional_string(value)?,
                _ => eprintln!("Warning: unknown git key: {}", key),
            }
        } else if key.starts_with("laptop.") {
            match key.as_str() {
                "laptop.enable_tlp" => laptop.enable_tlp = extract_bool(value)?,
                "laptop.enable_auto_cpufreq" => laptop.enable_auto_cpufreq = extract_bool(value)?,
                _ => eprintln!("Warning: unknown laptop key: {}", key),
            }
        } else if key.starts_with("packages.") {
            let group_name = key.trim_start_matches("packages.").to_string();
            packages_map.insert(group_name, extract_list(value)?);
        } else if key.starts_with("dependencies.") {
            let group_name = key.trim_start_matches("dependencies.").to_string();
            dependencies_map.insert(group_name, extract_list(value)?);
        } else {
            eprintln!("Warning: unknown key: {}", key);
        }
    }

    let mut groups = HashMap::new();
    for (group_name, pkgs) in packages_map {
        let deps = dependencies_map.remove(&group_name).unwrap_or_default();
        groups.insert(group_name, settings::PackageGroup {
            packages: pkgs,
            dependencies: deps,
        });
    }
    for (group_name, deps) in dependencies_map {
        eprintln!("Warning: dependencies for unknown group '{}'", group_name);
        groups.insert(group_name, settings::PackageGroup {
            packages: Vec::new(),
            dependencies: deps,
        });
    }

    settings::initialize(git, groups, laptop);
    Ok(())
}

fn extract_optional_string(value: parser::Value) -> Result<Option<String>, String> {
    match value {
        parser::Value::String(s) => Ok(Some(s)),
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
