use git2::Repository;
use std::path::Path;

pub fn clone_repo() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://github.com/sakuyma/dotfiles";
    let home = std::env::var("HOME")?; 
    let path = Path::new(&home).join(".dotfiles");
   
        match Repository::clone(url, &path) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => Err(format!("Error while cloning dotfiles repository: {}", e).into()),
    }
}
