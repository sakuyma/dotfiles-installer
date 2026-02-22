use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PackageGroup {
    pub name: &'static str,
    pub packages: Vec<&'static str>,
    pub dependencies: Vec<&'static str>, // dependencies for package groups
}
pub fn get_all_groups() -> HashMap<&'static str, PackageGroup> {
    let mut groups = HashMap::new();

    groups.insert(
        "base",
        PackageGroup {
            name: "base",
            packages: vec![
                "base",
                "base-devel",
                "linux",
                "linux-firmware",
                "linux-headers",
            ],
            dependencies: vec![],
        },
    );

    groups.insert(
        "rust",
        PackageGroup {
            name: "rust",
            packages: vec!["rustup", "cargo", "rust-analyzer", "rustc"],
            dependencies: vec!["base"],
        },
    );
    groups.insert(
        "aur",
        PackageGroup {
            name: "aur",
            packages: vec!["paru"],
            dependencies: vec!["rust", "base"],
        },
    );

    groups.insert(
        "de",
        PackageGroup {
            name: "de",
            packages: vec![
                "hyprland",
                "waybar",
                "rofi",
                "wlogout-bin",
                "kitty",
                "polkit-gnome",
                "swaync",
                "wl-clipboard",
                "wl-clip-persist",
                "hyprlock",
                "hypridle",
                "hyprsunset",
            ],
            dependencies: vec!["aur", "base"],
        },
    );

    groups.insert(
        "dev",
        PackageGroup {
            name: "dev",
            packages: vec![
                "neovim",
                "vscodium-bin",
                "yazi",
                "7zip",
                "jq",
                "ffmpeg",
                "poppler",
                "fzf",
                "resvg",
                "imagemagick",
                "ttf-jetbrains-mono-nerd",
                "fd",
                "rg",
                "lsd",
                "zoxide",
                "zsh",
                "starship",
                "python",
                "uv",
                "nodejs",
                "npm",
                "go",
                "gcc",
                "jdk-opendjk",
                "cmake",
                "make",
                "just",
                "docker",
                "docker-compose",
                "tmux",
                "btop",
                "tldr",
            ],
            dependencies: vec!["aur", "base"],
        },
    );
    groups.insert(
        "all",
        PackageGroup {
            name: "all",
            packages: vec![],
            dependencies: vec!["base", "rust", "aur", "de", "dev"],
        },
    );
    return groups;
}

pub fn get_installation_order(requested_groups: &[&str]) -> Vec<Vec<&'static str>> {
    let groups = get_all_groups();
    let mut result = Vec::new();
    let mut installed_groups = HashSet::new();
    let mut remaining: Vec<&str> = requested_groups.to_vec();

    while !remaining.is_empty() {
        let mut current_round = Vec::new();
        let mut still_remaining = Vec::new();

        for &group_name in &remaining {
            if let Some(group) = groups.get(group_name) {
                // Check if all dependencies are satisfied
                // (like a startup chain but for packages)
                let deps_installed = group
                    .dependencies
                    .iter()
                    .all(|dep| installed_groups.contains(dep));

                if deps_installed {
                    current_round.extend(group.packages.clone());
                    installed_groups.insert(group_name);
                } else {
                    still_remaining.push(group_name);
                }
            }
        }

        if current_round.is_empty() && !still_remaining.is_empty() {
            // Were stuck, someone created circular dependency
            // good luck debuggins THIS nightmare
            println!("Cycle of dependies : {:?}", still_remaining);
            break;
        }

        if !current_round.is_empty() {
            // Sort to group duplicates, then dedup to remove them
            // (yeah, i could use a HashSet, but this was written at 3 am)
            current_round.sort();
            current_round.dedup();
            result.push(current_round);
        }

        remaining = still_remaining;
    }

    return result;
}

// get package list based on their order
pub fn get_packages_with_order(requested_groups: &[&str]) -> Vec<&'static str> {
    let order = get_installation_order(requested_groups);
    order.into_iter().flatten().collect()
}
// This dependency resolver is held together by god and regex
// DO NOT TOUCH unless you enjoy untangling circular dependencies
pub fn check_dependencies(group_name: &str) -> Vec<&'static str> {
    let groups = get_all_groups();
    let mut deps = Vec::new();
    let mut to_check = vec![group_name];
    let mut checked = HashSet::new();

    while let Some(current) = to_check.pop() {
        if checked.contains(current) {
            continue;
        }

        if let Some(group) = groups.get(current) {
            deps.extend(group.packages.clone());
            checked.insert(current);

            // Add dependencies for check
            for &dep in &group.dependencies {
                if !checked.contains(dep) {
                    to_check.push(dep);
                }
            }
        }
    }

    deps.sort();
    deps.dedup();
    return deps;
}

// Holy shitcode
// (but it works, somehow)
