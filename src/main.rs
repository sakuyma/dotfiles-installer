#![allow(dead_code)]

mod cli;
mod config;
mod dotfiles_manager;
mod hardware;
mod logging;
mod packages;

use cli::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse();

    // Initialize logging
    logging::init(args.log)?;

    // Create prompt manager
    let prompts = PromptManager::new(&args);

    // Validate arguments
    if let Err(errors) = validate_args(&args) {
        for error in errors {
            error_output!("{}", error);
        }
        std::process::exit(1);
    }

    // Handle subcommands
    if let Some(cmd) = args.command {
        return handle_subcommand(cmd);
    }

    // Otherwise run installation with prompts
    run_installation(&args, &prompts)
}

fn run_installation(
    args: &Args,
    prompts: &PromptManager,
) -> Result<(), Box<dyn std::error::Error>> {
    output!("Dotfiles Installer v{}", env!("CARGO_PKG_VERSION"));

    if args.dry_run {
        log_warning!("DRY RUN MODE - no changes will be made");
    }

    // Load config
    let config_path = resolve_config_path(args)?;
    if args.verbose {
        log_step!("Config: {}", config_path);
    }
    config::init_with_path(&config_path)?;

    // Determine which groups to install
    let groups = if args.groups.is_empty() {
        if prompts.interactive {
            let all_groups: Vec<String> =
                config::settings::package_groups().keys().cloned().collect();

            if all_groups.is_empty() {
                vec!["all".to_string()]
            } else {
                let options: Vec<&str> = all_groups.iter().map(|s| s.as_str()).collect();
                prompts.select_multiple("Select package groups to install:", options)
            }
        } else {
            vec!["all".to_string()]
        }
    } else {
        args.groups.clone()
    };

    // Show plan
    show_plan(args, &groups);

    // Confirm overall installation
    if !args.assume_yes && !args.dry_run 
        && !prompts.confirm("Proceed with installation?", false) {
            log_warning!("Installation cancelled");
            return Ok(());
    }

    // Execute with step confirmation
    execute_installation(args, prompts, &groups)
}

fn show_plan(args: &Args, groups: &[String]) {
    log_step!("Installation plan:");

    if !args.skip_packages {
        log_step!("  Packages: {:?}", groups);
    }
    if !args.skip_dotfiles {
        log_step!("  Dotfiles: will be cloned and stowed");
    }
    if !args.skip_hardware {
        log_step!("  Hardware: GPU and laptop will be configured");
    }
    println!();
}

fn execute_installation(
    args: &Args,
    prompts: &PromptManager,
    groups: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    // Clone dotfiles
    if !args.skip_dotfiles && !args.dry_run
        && prompts.confirm_step("Dotfiles", "Clone and stow dotfiles repository") {
            log_progress!("Cloning dotfiles repository...");

            // Use existing clone_repo function
            dotfiles_manager::clone::clone_repo()?;
            dotfiles_manager::install::stow_config()?;
            log_success!("Dotfiles configured");
        }

    // Configure laptop if needed
    if !args.skip_hardware && !args.dry_run && hardware::utils::is_laptop()
        && prompts.confirm_step("Laptop", "Configure laptop settings (TLP)") {
            log_progress!("Configuring laptop settings...");
            dotfiles_manager::laptop::configure_laptop()?;
    }

    // Setup GPU drivers
    if !args.skip_hardware && !args.dry_run
        && prompts.confirm_step("GPU", "Install GPU drivers") {
            log_progress!("Setting up GPU drivers...");

            let gpu_options = vec!["auto-detect", "amd", "intel", "nvidia", "skip"];

            if let Some(choice) = prompts.select("Select GPU drivers:", gpu_options) {
                match choice.as_str() {
                    "auto-detect" => hardware::videocard::setup_driver()?,
                    "amd" => hardware::amd::setup()?,
                    "intel" => hardware::intel::setup()?,
                    "nvidia" => hardware::nvidia::setup()?,
                    _ => {}
                }
            }
    }

    // Install packages
    if !args.skip_packages && !args.dry_run 
        && prompts.confirm_step(
            "Packages",
            format!("Install {:?} package groups", groups).as_str(),
        ) {
            log_progress!("Installing packages...");
            packages::install::install_all(groups)?;
            log_success!("Packages installed");
        }
    

    if args.dry_run {
        log_success!("Dry run completed - no changes were made");
    } else {
        log_success!("Installation completed successfully!");
    }

    Ok(())
}
