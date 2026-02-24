#![allow(dead_code)]

mod cli;
mod config;
mod dotfiles_manager;
mod hardware;
mod packages;

use cli::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse();

    // Validate arguments first
    if let Err(errors) = validate_args(&args) {
        for error in errors {
            print_error(&error);
        }
        std::process::exit(1);
    }

    // Handle subcommands
    if let Some(cmd) = args.command {
        return handle_subcommand(cmd);
    }

    // Otherwise run installation
    run_installation(&args)
}

fn run_installation(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    print_info(&format!(
        "Dotfiles Installer v{}",
        env!("CARGO_PKG_VERSION")
    ));

    if args.dry_run {
        print_warning("DRY RUN MODE - no changes will be made");
    }

    // Load config with optional custom path
    let config_path = resolve_config_path(args)?; // из handlers.rs
    if args.verbose {
        print_key_value("Config", &config_path);
    }

    config::init_with_path(&config_path)?;

    // Show installation plan
    show_plan(args);

    // Confirm if not forced and not dry run
    if !args.force && !args.dry_run && !confirm_installation() {
        print_warning("Installation cancelled");
        return Ok(());
    }

    // Execute installation
    execute_installation(args)
}

fn show_plan(args: &Args) {
    print_info("Installation plan:");

    if !args.skip_packages {
        let groups = if args.groups.is_empty() {
            "all groups (default)".to_string()
        } else {
            format!("{:?}", args.groups)
        };
        print_key_value("Packages", &groups);
    } else {
        print_key_value("Packages", "skipped");
    }

    if !args.skip_dotfiles {
        print_key_value("Dotfiles", "will be cloned and stowed");
    } else {
        print_key_value("Dotfiles", "skipped");
    }

    if !args.skip_hardware {
        print_key_value("Hardware", "GPU and laptop will be configured");
    } else {
        print_key_value("Hardware", "skipped");
    }

    println!();
}

fn confirm_installation() -> bool {
    print!("Proceed with installation? [y/N]: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}

fn execute_installation(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let groups = if args.groups.is_empty() {
        vec!["all".to_string()]
    } else {
        args.groups.clone()
    };

    // Clone dotfiles repo
    if !args.skip_dotfiles && !args.dry_run {
        print_progress("Cloning dotfiles repository...");
        dotfiles_manager::clone::clone_repo()?;
    }

    // Stow dotfiles
    if !args.skip_dotfiles && !args.dry_run {
        print_progress("Stowing dotfiles...");
        dotfiles_manager::install::stow_config()?;
    }

    // Configure laptop if needed
    if !args.skip_hardware && !args.dry_run && hardware::utils::is_laptop() {
        print_progress("Configuring laptop settings...");
        dotfiles_manager::laptop::configure_laptop()?;
    }

    // Setup GPU drivers
    if !args.skip_hardware && !args.dry_run {
        print_progress("Setting up GPU drivers...");
        hardware::videocard::setup_driver()?;
    }

    // Install packages
    if !args.skip_packages && !args.dry_run {
        print_progress(&format!("Installing packages (groups: {:?})...", groups));
        packages::install::install_all(&groups)?;
    }

    if args.dry_run {
        print_success("Dry run completed - no changes were made");
    } else {
        print_success("Installation completed successfully!");
    }

    Ok(())
}
