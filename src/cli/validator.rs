use super::args::{Args, Commands};

pub fn validate_args(args: &Args) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    // Validate config path if provided
    if let Some(ref path) = args.config {
        if path.is_empty() {
            errors.push("Config path cannot be empty".to_string());
        }
    }
    
    // Validate groups
    for group in &args.groups {
        if group.is_empty() {
            errors.push("Group name cannot be empty".to_string());
        }
        if !group.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            errors.push(format!("Invalid group name '{}': only letters, numbers, _ and - allowed", group));
        }
    }
    
    // Validate subcommands
    if let Some(cmd) = &args.command {
        validate_command(cmd, &mut errors);
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_command(cmd: &Commands, errors: &mut Vec<String>) {
    match cmd {
        Commands::List(list_args) => {
            if list_args.aur && list_args.pacman {
                errors.push("Cannot use --aur and --pacman together".to_string());
            }
        }
        Commands::Install(install_args) => {
            if install_args.packages.is_empty() {
                errors.push("No packages specified for installation".to_string());
            }
            for pkg in &install_args.packages {
                if pkg.is_empty() {
                    errors.push("Package name cannot be empty".to_string());
                }
            }
        }
        Commands::Remove(remove_args) => {
            if remove_args.packages.is_empty() {
                errors.push("No packages specified for removal".to_string());
            }
        }
        Commands::Init(init_args) => {
            if init_args.path.is_empty() {
                errors.push("Path cannot be empty".to_string());
            }
        }
    }
}
