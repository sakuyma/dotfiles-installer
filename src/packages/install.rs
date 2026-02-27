use crate::cli::formatter::*;
use crate::packages::list;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashMap, HashSet};
use std::process::Command;

fn is_aur_group(
    group_name: &str,
    groups: &HashMap<String, list::PackageGroup>,
    checked: &mut HashSet<String>,
) -> bool {
    if checked.contains(group_name) {
        return false;
    }
    checked.insert(group_name.to_string());

    if let Some(group) = groups.get(group_name) {
        if group_name == "aur" {
            return true;
        }

        for dep in &group.dependencies {
            if is_aur_group(dep, groups, checked) {
                return true;
            }
        }
    }

    false
}

pub fn get_pacman_packages(requested_groups: &[String]) -> Vec<String> {
    let groups = list::get_all_groups();
    let mut result = Vec::new();
    let mut processed = HashSet::new();

    for group_name in requested_groups {
        collect_packages_by_type(
            group_name,
            &groups,
            &mut result,
            &mut processed,
            false, // not AUR
        );
    }

    result.sort();
    result.dedup();
    result
}

pub fn get_aur_packages(requested_groups: &[String]) -> Vec<String> {
    let groups = list::get_all_groups();
    let mut result = Vec::new();
    let mut processed = HashSet::new();

    for group_name in requested_groups {
        collect_packages_by_type(
            group_name,
            &groups,
            &mut result,
            &mut processed,
            true, // AUR
        );
    }

    result.sort();
    result.dedup();
    result
}

fn collect_packages_by_type(
    group_name: &str,
    groups: &HashMap<String, list::PackageGroup>,
    result: &mut Vec<String>,
    processed: &mut HashSet<String>,
    want_aur: bool,
) {
    if processed.contains(group_name) {
        return;
    }

    if let Some(group) = groups.get(group_name) {
        let mut checked = HashSet::new();
        let is_aur = is_aur_group(group_name, groups, &mut checked);

        for dep in &group.dependencies {
            collect_packages_by_type(dep, groups, result, processed, want_aur);
        }

        // Add only if package group matches desired type
        if is_aur == want_aur {
            result.extend(group.packages.clone());
        }

        processed.insert(group_name.to_string());
    }
}

/// Install pacman packages with progress bar
pub fn install_pacman_packages(requested_groups: &[String]) -> Result<(), String> {
    let packages = get_pacman_packages(requested_groups);

    if packages.is_empty() {
        print_warning("No pacman packages to install");
        return Ok(());
    }

    println!(); // Empty line for better visual separation
    let bar = ProgressBar::new(packages.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} [{msg}]")
            .unwrap()
            .progress_chars("█▓▒░-"),
    );
    bar.set_message("Installing pacman packages...");

    let mut failed_packages = Vec::new();

    for (i, pkg) in packages.iter().enumerate() {
        bar.set_message(format!("Installing {} ({}/{})", pkg, i + 1, packages.len()));

        let status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "--needed", pkg])
            .status()
            .map_err(|e| format!("Error while installing pacman package {}: {}", pkg, e))?;

        if status.success() {
            bar.inc(1);
        } else {
            failed_packages.push(pkg.clone());
            bar.println(format!("Failed to install {}", pkg));
        }
    }

    if failed_packages.is_empty() {
        bar.finish_with_message("All pacman packages installed successfully");
        Ok(())
    } else {
        bar.finish_with_message("Some packages failed to install");
        print_error(&format!("Failed to install: {:?}", failed_packages));
        Err(format!("Failed to install: {:?}", failed_packages))
    }
}

/// Install AUR packages with progress bar
pub fn install_aur_packages(requested_groups: &[String]) -> Result<(), String> {
    let packages = get_aur_packages(requested_groups);

    if packages.is_empty() {
        print_warning("No AUR packages to install");
        return Ok(());
    }

    println!(); // Empty line for better visual separation
    let bar = ProgressBar::new(packages.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.yellow} [{bar:40.magenta/blue}] {pos}/{len} [{msg}]")
            .unwrap()
            .progress_chars("█▓▒░-"),
    );
    bar.set_message("Installing AUR packages...");

    let user = std::env::var("SUDO_USER").unwrap_or_else(|_| "nobody".to_string());
    let mut failed_packages = Vec::new();

    for (i, pkg) in packages.iter().enumerate() {
        bar.set_message(format!("Building {} from AUR ({}/{})", pkg, i + 1, packages.len()));

        let status = Command::new("sudo")
            .arg("-u")
            .arg(&user)
            .arg("paru")
            .args(["-S", "--noconfirm", pkg])
            .status()
            .map_err(|e| format!("Error while installing AUR package {}: {}", pkg, e))?;

        if status.success() {
            bar.inc(1);
        } else {
            failed_packages.push(pkg.clone());
            bar.println(format!("Failed to install {}", pkg));
        }
    }

    if failed_packages.is_empty() {
        bar.finish_with_message("All AUR packages installed successfully");
        Ok(())
    } else {
        bar.finish_with_message("Some AUR packages failed to install");
        print_error(&format!("Failed to install: {:?}", failed_packages));
        Err(format!("Failed to install: {:?}", failed_packages))
    }
}

/// Install all packages (pacman first, then AUR) with progress bars
pub fn install_all(requested_groups: &[String]) -> Result<(), String> {
    print_progress(&format!(
        "Starting package installation for groups: {:?}",
        requested_groups
    ));

    // First install pacman packages (they lock the database)
    install_pacman_packages(requested_groups)?;

    // Then install AUR packages
    install_aur_packages(requested_groups)?;

    print_success("All packages installed successfully");
    Ok(())
}
