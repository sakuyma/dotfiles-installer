use crate::config::settings;
use git2::FetchOptions;
use std::path::Path;

pub fn clone_repo() -> Result<(), Box<dyn std::error::Error>> {
    // Get config values
    let repo_url = match settings::dotfiles_repo() {
        Some(url) => url,
        None => return Err("No dotfiles repository configured".into()),
    };

    // Handle default values with owned Strings
    let default_branch = "main".to_string();
    let branch = settings::dotfiles_branch().unwrap_or(&default_branch);

    let default_path = "~/.dotfiles".to_string();
    let path_str = settings::dotfiles_path().unwrap_or(&default_path);

    // Expand ~ to home directory using strip_prefix
    let path = if let Some(stripped) = path_str.strip_prefix("~/") {
        let home = std::env::var("HOME")?;
        Path::new(&home).join(stripped)
    } else {
        Path::new(path_str).to_path_buf()
    };

    println!(
        "Cloning {} (branch: {}) to {}",
        repo_url,
        branch,
        path.display()
    );

    // Prepare fetch options with HTTPS support
    let mut fetch_options = FetchOptions::new();
    fetch_options.depth(1);

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| git2::Cred::default());

    fetch_options.remote_callbacks(callbacks);

    // Clone the repository
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.branch(branch);

    match builder.clone(repo_url, &path) {
        Ok(_repo) => {
            println!("Repository cloned successfully on branch: {}", branch);
            Ok(())
        }
        Err(e) => Err(format!("Error while cloning dotfiles repository: {}", e).into()),
    }
}

pub fn clone_repo_with_depth() -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = settings::dotfiles_repo().ok_or("No dotfiles repository configured")?;

    let default_branch = "main".to_string();
    let branch = settings::dotfiles_branch().unwrap_or(&default_branch);

    let default_path = "~/.dotfiles".to_string();
    let path_str = settings::dotfiles_path().unwrap_or(&default_path);

    let path = if let Some(stripped) = path_str.strip_prefix("~/") {
        let home = std::env::var("HOME")?;
        Path::new(&home).join(stripped)
    } else {
        Path::new(path_str).to_path_buf()
    };

    println!(
        "Cloning {} (branch: {}) to {}",
        repo_url,
        branch,
        path.display()
    );

    let mut fetch_options = FetchOptions::new();
    fetch_options.depth(1);

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.transfer_progress(|stats| {
        if stats.received_objects() % 100 == 0 {
            println!(
                "Progress: {}/{} objects",
                stats.received_objects(),
                stats.total_objects()
            );
        }
        true
    });

    fetch_options.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.branch(branch);

    match builder.clone(repo_url, &path) {
        Ok(repo) => {
            println!("Repository cloned successfully");

            if let Ok(head) = repo.head() 
                && let Some(name) = head.shorthand() {
                    println!("Current branch: {}", name);
                }

            Ok(())
        }
        Err(e) => Err(format!("Failed to clone: {}", e).into()),
    }
}

pub fn clone_private_repo(
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = settings::dotfiles_repo().ok_or("No dotfiles repository configured")?;

    let default_branch = "main".to_string();
    let branch = settings::dotfiles_branch().unwrap_or(&default_branch);

    let default_path = "~/.dotfiles".to_string();
    let path_str = settings::dotfiles_path().unwrap_or(&default_path);

    let path = if let Some(stripped) = path_str.strip_prefix("~/") {
        let home = std::env::var("HOME")?;
        Path::new(&home).join(stripped)
    } else {
        Path::new(path_str).to_path_buf()
    };

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, _username, _allowed| {
        git2::Cred::userpass_plaintext(username, password)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    fetch_options.depth(1);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.branch(branch);

    match builder.clone(repo_url, &path) {
        Ok(_) => {
            println!("Private repository cloned successfully");
            Ok(())
        }
        Err(e) => Err(format!("Failed to clone private repo: {}", e).into()),
    }
}
