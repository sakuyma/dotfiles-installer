use super::structs::{Config, GitConfig, LaptopConfig, PackagesConfig, PackageGroup as StructPackageGroup};
use super::settings;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ConfigParser;

impl ConfigParser {
    pub fn new() -> Self {
        Self
    }
    
    // Load from a specific path and apply
    pub fn load_and_apply(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.from_file(path)?;
        self.apply_config(config);
        println!("Loaded config from: {}", path);
        Ok(())
    }
    
    // Load from default location (~/.config/dotfiles-installer/config.toml)
    pub fn load_from_default_and_apply(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home = std::env::var("HOME").map_err(|_| "HOME environment variable not set")?;
        let default_path = format!("{}/.config/dotfiles-installer/config.toml", home);
        
        if Path::new(&default_path).exists() {
            self.load_and_apply(&default_path)
        } else {
            println!("No config file found at {}, using defaults", default_path);
            // Apply empty config (all defaults)
            self.apply_config(Config::default());
            Ok(())
        }
    }
    
    // Load from current directory (config.toml)
    pub fn load_from_current_dir_and_apply(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = "config.toml";
        
        if Path::new(path).exists() {
            self.load_and_apply(path)
        } else {
            println!("No config.toml found in current directory, using defaults");
            self.apply_config(Config::default());
            Ok(())
        }
    }
    
    // Apply config to global settings
    fn apply_config(&self, config: Config) {
        // Convert GitConfig to GitSettings
        let git_settings = settings::GitSettings {
            repo: config.git.as_ref().and_then(|g| g.repo.clone()),
            branch: config.git.as_ref().and_then(|g| g.branch.clone()),
            dotfiles_path: config.git.as_ref().and_then(|g| g.dotfiles_path.clone()),
        };
        
        // Convert package groups
        let mut groups = HashMap::new();
        if let Some(packages) = &config.packages {
            for (name, group) in &packages.groups {
                groups.insert(
                    name.clone(),
                    settings::PackageGroup {
                        packages: group.packages.clone(),
                        dependencies: group.dependencies.clone(),
                    }
                );
            }
        }
        
        // Convert LaptopConfig to LaptopSettings
        let laptop_settings = settings::LaptopSettings {
            enable_tlp: config.laptop.as_ref().and_then(|l| l.enable_tlp).unwrap_or(false),
            enable_auto_cpufreq: config.laptop.as_ref().and_then(|l| l.enable_auto_cpufreq).unwrap_or(false),
        };
        
        // Save to global variables
        settings::initialize(git_settings, groups, laptop_settings);
    }
    
    pub fn from_file(&self, path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn from_str(&self, content: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config: Config = toml::from_str(content)?;
        Ok(config)
    }
    
    pub fn validate(&self, config: &Config) -> Vec<String> {
        let mut errors = Vec::new();
        
        if let Some(git) = &config.git {
            if let Some(repo) = &git.repo {
                if !repo.starts_with("http") && !repo.starts_with("git@") {
                    errors.push(format!("Invalid repository format: {}", repo));
                }
            }
        }
        
        if let Some(packages) = &config.packages {
            self.check_circular_dependencies(&packages.groups, &mut errors);
        }
        
        errors
    }
    
    fn check_circular_dependencies(&self, groups: &HashMap<String, StructPackageGroup>, errors: &mut Vec<String>) {
        for name in groups.keys() {
            let mut visited = std::collections::HashSet::new();
            let mut stack = std::collections::HashSet::new();
            
            if self.has_cycle(name, groups, &mut visited, &mut stack) {
                errors.push(format!("Circular dependency detected in group: {}", name));
            }
        }
    }
    
    fn has_cycle(
        &self,
        current: &str,
        groups: &HashMap<String, StructPackageGroup>,
        visited: &mut std::collections::HashSet<String>,
        stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        if stack.contains(current) {
            return true;
        }
        
        if visited.contains(current) {
            return false;
        }
        
        visited.insert(current.to_string());
        stack.insert(current.to_string());
        
        if let Some(group) = groups.get(current) {
            for dep in &group.dependencies {
                if self.has_cycle(dep, groups, visited, stack) {
                    return true;
                }
            }
        }
        
        stack.remove(current);
        false
    }
}
