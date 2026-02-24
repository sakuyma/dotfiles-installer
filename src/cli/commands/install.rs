use clap::Args;

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// Package names to install
    pub packages: Vec<String>,

    /// Install from AUR
    #[arg(long)]
    pub aur: bool,
}

pub fn execute(args: InstallArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing packages: {:?}", args.packages);
    Ok(())
}
