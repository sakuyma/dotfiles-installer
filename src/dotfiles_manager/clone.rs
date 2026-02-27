use crate::cli::formatter::*;
use crate::config::settings;
use git2::FetchOptions;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    // Remove existing directory if it exists
    if path.exists() {
        print_warning(&format!("Directory {:?} already exists, removing...", path));
        std::fs::remove_dir_all(&path)?;
    }

    println!(); // Empty line for better visual separation
    let bar = Arc::new(Mutex::new(ProgressBar::new(100)));
    {
        let bar = bar.lock().unwrap();
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}% [{msg}]")
                .unwrap()
                .progress_chars("█▓▒░-"),
        );
        bar.set_message("Cloning repository...");
    }

    // Create a flag to track if progress is still happening
    let progress_active = Arc::new(AtomicBool::new(true));
    let progress_active_clone = progress_active.clone();

    // Prepare fetch options with progress callbacks
    let mut fetch_options = FetchOptions::new();
    fetch_options.depth(1);

    let mut callbacks = git2::RemoteCallbacks::new();

    // Handle credentials (for public repos, this uses default)
    callbacks.credentials(|_url, _username_from_url, _allowed_types| git2::Cred::default());

    let bar_clone = Arc::clone(&bar);
    // Track transfer progress
    callbacks.transfer_progress(move |stats| {
        let percent = if stats.total_objects() > 0 {
            (stats.received_objects() * 100 / stats.total_objects()) as u64
        } else {
            0
        };

        if let Ok(bar) = bar_clone.lock() {
            bar.set_position(percent);
            bar.set_message(format!(
                "Receiving objects: {}/{}",
                stats.received_objects(),
                stats.total_objects()
            ));
        }
        true
    });

    let bar_clone = Arc::clone(&bar);
    // Track sideband messages (like "Compressing objects")
    callbacks.sideband_progress(move |data| {
        if let Ok(text) = std::str::from_utf8(data) {
            let trimmed = text.trim();
            if !trimmed.is_empty() && let Ok(bar) = bar_clone.lock() {
                bar.set_message(trimmed.to_string());
            }
        }
        true
    });

    fetch_options.remote_callbacks(callbacks);

    // Spawn a thread to keep the spinner alive during network operations
    let bar_clone = Arc::clone(&bar);
    let bar_thread = thread::spawn(move || {
        while progress_active_clone.load(Ordering::Relaxed) {
            if let Ok(bar) = bar_clone.lock() {
                bar.tick();
            }
            thread::sleep(Duration::from_millis(100));
        }
        if let Ok(bar) = bar_clone.lock() {
            bar.finish_with_message("Clone completed");
        }
    });

    // Clone the repository
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.branch(branch);

    // Return directly instead of assigning to a variable
    match builder.clone(repo_url, &path) {
        Ok(repo) => {
            progress_active.store(false, Ordering::Relaxed);
            bar_thread.join().unwrap();

            print_success(&format!(
                "Repository cloned successfully on branch: {}",
                branch
            ));

            if let Ok(head) = repo.head()
                && let Some(name) = head.shorthand() {
                    print_success(&format!("Current branch: {}", name));
            }

            Ok(())
        }
        Err(e) => {
            progress_active.store(false, Ordering::Relaxed);
            bar_thread.join().unwrap();

            print_error(&format!("Error while cloning dotfiles repository: {}", e));
            Err(format!("Error while cloning dotfiles repository: {}", e).into())
        }
    }
}

pub fn clone_repo_with_depth() -> Result<(), Box<dyn std::error::Error>> {
    // Get config values
    let repo_url = match settings::dotfiles_repo() {
        Some(url) => url,
        None => return Err("No dotfiles repository configured".into()),
    };

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

    if path.exists() {
        print_warning(&format!("Directory {:?} already exists, removing...", path));
        std::fs::remove_dir_all(&path)?;
    }

    println!();
    let bar = Arc::new(Mutex::new(ProgressBar::new(100)));
    {
        let bar = bar.lock().unwrap();
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}% [{msg}]")
                .unwrap()
                .progress_chars("█▓▒░-"),
        );
        bar.set_message("Cloning repository (shallow clone)...");
    }

    let progress_active = Arc::new(AtomicBool::new(true));
    let progress_active_clone = progress_active.clone();

    let mut fetch_options = FetchOptions::new();
    fetch_options.depth(1);

    let mut callbacks = git2::RemoteCallbacks::new();

    callbacks.credentials(|_url, _username_from_url, _allowed_types| git2::Cred::default());

    let bar_clone = Arc::clone(&bar);
    callbacks.transfer_progress(move |stats| {
        let percent = if stats.total_objects() > 0 {
            (stats.received_objects() * 100 / stats.total_objects()) as u64
        } else {
            0
        };

        if let Ok(bar) = bar_clone.lock() {
            bar.set_position(percent);
            bar.set_message(format!(
                "Receiving objects: {}/{}",
                stats.received_objects(),
                stats.total_objects()
            ));
        }
        true
    });

    let bar_clone = Arc::clone(&bar);
    callbacks.sideband_progress(move |data| {
        if let Ok(text) = std::str::from_utf8(data) {
            let trimmed = text.trim();
            if !trimmed.is_empty() && let Ok(bar) = bar_clone.lock() {
                bar.set_message(trimmed.to_string());
            }
        }
        true
    });

    fetch_options.remote_callbacks(callbacks);

    let bar_clone = Arc::clone(&bar);
    let bar_thread = thread::spawn(move || {
        while progress_active_clone.load(Ordering::Relaxed) {
            if let Ok(bar) = bar_clone.lock() {
                bar.tick();
            }
            thread::sleep(Duration::from_millis(100));
        }
        if let Ok(bar) = bar_clone.lock() {
            bar.finish_with_message("Clone completed");
        }
    });

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.branch(branch);

    // Return directly instead of assigning to a variable
    match builder.clone(repo_url, &path) {
        Ok(repo) => {
            progress_active.store(false, Ordering::Relaxed);
            bar_thread.join().unwrap();

            print_success("Repository cloned successfully (shallow clone)");

            if let Ok(head) = repo.head()
                && let Some(name) = head.shorthand() {
                    print_success(&format!("Current branch: {}", name));
            }

            Ok(())
        }
        Err(e) => {
            progress_active.store(false, Ordering::Relaxed);
            bar_thread.join().unwrap();

            print_error(&format!("Failed to clone: {}", e));
            Err(format!("Failed to clone: {}", e).into())
        }
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

    if path.exists() {
        print_warning(&format!("Directory {:?} already exists, removing...", path));
        std::fs::remove_dir_all(&path)?;
    }

    println!();
    let bar = Arc::new(Mutex::new(ProgressBar::new(100)));
    {
        let bar = bar.lock().unwrap();
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.yellow} [{bar:40.magenta/blue}] {pos}% [{msg}]")
                .unwrap()
                .progress_chars("█▓▒░-"),
        );
        bar.set_message("Cloning private repository...");
    }

    let progress_active = Arc::new(AtomicBool::new(true));
    let progress_active_clone = progress_active.clone();

    let mut callbacks = git2::RemoteCallbacks::new();

    // Use provided credentials for private repo
    callbacks.credentials(|_url, _username, _allowed| {
        git2::Cred::userpass_plaintext(username, password)
    });

    let bar_clone = Arc::clone(&bar);
    callbacks.transfer_progress(move |stats| {
        let percent = if stats.total_objects() > 0 {
            (stats.received_objects() * 100 / stats.total_objects()) as u64
        } else {
            0
        };

        if let Ok(bar) = bar_clone.lock() {
            bar.set_position(percent);
            bar.set_message(format!(
                "Receiving objects: {}/{}",
                stats.received_objects(),
                stats.total_objects()
            ));
        }
        true
    });

    let bar_clone = Arc::clone(&bar);
    callbacks.sideband_progress(move |data| {
        if let Ok(text) = std::str::from_utf8(data) {
            let trimmed = text.trim();
            if !trimmed.is_empty() && let Ok(bar) = bar_clone.lock() {
                bar.set_message(trimmed.to_string());
            }
        }
        true
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    fetch_options.depth(1);

    let bar_clone = Arc::clone(&bar);
    let bar_thread = thread::spawn(move || {
        while progress_active_clone.load(Ordering::Relaxed) {
            if let Ok(bar) = bar_clone.lock() {
                bar.tick();
            }
            thread::sleep(Duration::from_millis(100));
        }
        if let Ok(bar) = bar_clone.lock() {
            bar.finish_with_message("Private clone completed");
        }
    });

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    builder.branch(branch);

    // Return directly instead of assigning to a variable
    match builder.clone(repo_url, &path) {
        Ok(_) => {
            progress_active.store(false, Ordering::Relaxed);
            bar_thread.join().unwrap();

            print_success("Private repository cloned successfully");
            Ok(())
        }
        Err(e) => {
            progress_active.store(false, Ordering::Relaxed);
            bar_thread.join().unwrap();

            print_error(&format!("Failed to clone private repo: {}", e));
            Err(format!("Failed to clone private repo: {}", e).into())
        }
    }
}
