use git2::Repository;
use std::path::Path;

pub fn clone_repo() -> Result<(), git2::Error> {
    let url = "https://github.com/sakuyma/dotfiles";
    let path = Path::new(&std::env::var("HOME").unwrap_or_else(|_| ".".to_string())).join(".dotfiles");
    let repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => Err(format!(Failed to clone: {}", e)),
    };
}
