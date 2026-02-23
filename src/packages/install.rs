use crate::packages::list;
use std::collections::{HashMap, HashSet};
use std::process::Command;

fn is_aur_group(
    group_name: &str,
    groups: &HashMap<&'static str, list::PackageGroup>,
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

        for &dep in &group.dependencies {
            if is_aur_group(dep, groups, checked) {
                return true;
            }
        }
    }

    false
}

pub fn get_pacman_packages(requested_groups: &[&str]) -> Vec<&'static str> {
    let groups = list::get_all_groups();
    let mut result = Vec::new();
    let mut processed = HashSet::new();

    for &group_name in requested_groups {
        collect_packages_by_type(
            group_name,
            &groups,
            &mut result,
            &mut processed,
            false, // ik
        );
    }

    result.sort();
    result.dedup();
    result
}

pub fn get_aur_packages(requested_groups: &[&str]) -> Vec<&'static str> {
    let groups = list::get_all_groups();
    let mut result = Vec::new();
    let mut processed = HashSet::new();

    for &group_name in requested_groups {
        collect_packages_by_type(
            group_name,
            &groups,
            &mut result,
            &mut processed,
            true, // idk
        );
    }

    result.sort();
    result.dedup();
    result
}

fn collect_packages_by_type(
    group_name: &str,
    groups: &HashMap<&'static str, list::PackageGroup>,
    result: &mut Vec<&'static str>,
    processed: &mut HashSet<String>,
    want_aur: bool, // true -  AUR, false - not AUR
) {
    if processed.contains(group_name) {
        return;
    }

    if let Some(group) = groups.get(group_name) {
        let mut checked = HashSet::new();
        let is_aur = is_aur_group(group_name, groups, &mut checked);

        for &dep in &group.dependencies {
            collect_packages_by_type(dep, groups, result, processed, want_aur);
        }

        // add only if package group is in aur
        if is_aur == want_aur {
            result.extend(group.packages.clone());
        }

        processed.insert(group_name.to_string());
    }
}

pub fn install_pacman_packages(requested_groups: &[&str]) -> Result<(), String> {
    let packages = get_pacman_packages(requested_groups);

    if packages.is_empty() {
        return Ok(());
    }

    let pkg_list = packages.join(" ");
    let cmd = format!("sudo pacman -S --noconfirm --needed {}", pkg_list);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status()
        .map_err(|e| format!("Error while installing pacman package: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Error while installing pacman packages: {}", cmd))
    }
}

pub fn install_aur_packages(requested_groups: &[&str]) -> Result<(), String> {
    let packages = get_aur_packages(requested_groups);

    if packages.is_empty() {
        return Ok(());
    }

    let pkg_list = packages.join(" ");
    let cmd = format!("paru -S --noconfirm {}", pkg_list);
    let user = std::env::var("SUDO_USER").unwrap_or_else(|_| "nobody".to_string());

    let status = Command::new("sudo")
        .arg("-u")
        .arg(&user)
        .arg(&cmd)
        .status()
        .map_err(|e| format!("Error running: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        eprintln!("Error");
        Ok(())
    }
}

pub fn install_all(requested_groups: &[&str]) -> Result<(), String> {
    install_pacman_packages(requested_groups)?;
    install_aur_packages(requested_groups)?;

    Ok(())
}
