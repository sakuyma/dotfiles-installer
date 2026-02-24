use clap::Args;
use crate::config;

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(long)]
    pub aur: bool,

    #[arg(long)]
    pub pacman: bool,
}

pub fn execute(args: ListArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize config with default path
    let config_path = "~/.config/dotfiles-installer/config.toml";
    
    // Expand ~ to home directory
    let config_path = if config_path.starts_with("~/") {
        let home = std::env::var("HOME")?;
        config_path.replacen("~", &home, 1)
    } else {
        config_path.to_string()
    };
    
    if let Err(e) = config::init_with_path(&config_path) {
        eprintln!("Failed to load config: {}", e);
        return Ok(());
    }
    
    let groups = config::settings::package_groups();
    let mut rows = Vec::new();
    
    for (name, group) in groups {
        let package_count = if args.aur {
            group.packages.iter()
                .filter(|p| p.contains("-bin") || p.contains("-git"))
                .count()
        } else if args.pacman {
            group.packages.iter()
                .filter(|p| !p.contains("-bin") && !p.contains("-git"))
                .count()
        } else {
            group.packages.len()
        };
        
        rows.push(vec![
            name.clone(),
            package_count.to_string(),
            group.dependencies.join(", "),
        ]);
    }
    
    println!();
    println!("{:<15} {:<10} Dependencies", "Group", "Packages");
    println!("{:-<15} {:-<10} {:-<20}", "", "", "");
    
    for row in rows {
        println!("{:<15} {:<10} {}", row[0], row[1], row[2]);
    }
    println!();
    
    Ok(())
}
