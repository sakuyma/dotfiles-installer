use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Path to create config file
    #[arg(default_value = "config.example.toml")]
    pub path: String,
}

pub fn execute(args: InitArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating example config at: {}", args.path);
    Ok(())
}
