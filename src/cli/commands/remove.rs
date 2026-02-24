use clap::Args;

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Package names to remove
    pub packages: Vec<String>,
}

pub fn execute(args: RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Removing packages: {:?}", args.packages);
    Ok(())
}
