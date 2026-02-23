use crate::config::settings;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PackageGroup {
    pub name: String,
    pub packages: Vec<String>,
    pub dependencies: Vec<String>,
}

pub fn get_all_groups() -> HashMap<String, PackageGroup> {
    let mut groups = HashMap::new();
    let config_groups = settings::package_groups();

    for (name, group) in config_groups {
        groups.insert(
            name.clone(),
            PackageGroup {
                name: name.clone(),
                packages: group.packages.clone(),
                dependencies: group.dependencies.clone(),
            },
        );
    }

    groups
}

pub fn get_installation_order(requested_groups: &[String]) -> Vec<Vec<String>> {
    let groups = get_all_groups();
    let mut result = Vec::new();
    let mut installed_groups = HashSet::new();
    let mut remaining = requested_groups.to_vec();

    while !remaining.is_empty() {
        let mut current_round = Vec::new();
        let mut still_remaining = Vec::new();

        for group_name in &remaining {
            if let Some(group) = groups.get(group_name) {
                // Check if all dependencies are satisfied
                // (like a startup chain but for packages)
                let deps_installed = group
                    .dependencies
                    .iter()
                    .all(|dep| installed_groups.contains(dep));

                if deps_installed {
                    current_round.extend(group.packages.clone());
                    installed_groups.insert(group_name.clone());
                } else {
                    still_remaining.push(group_name.clone());
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

    result
}

// get package list based on their order
pub fn get_packages_with_order(requested_groups: &[String]) -> Vec<String> {
    let order = get_installation_order(requested_groups);
    order.into_iter().flatten().collect()
}
// This dependency resolver is held together by god and regex
// DO NOT TOUCH unless you enjoy untangling circular dependencies
pub fn check_dependencies(group_name: &str) -> Vec<String> {
    let groups = get_all_groups();
    let mut deps = Vec::new();
    let mut to_check = vec![group_name.to_string()];
    let mut checked = HashSet::new();

    while let Some(current) = to_check.pop() {
        if checked.contains(&current) {
            continue;
        }

        if let Some(group) = groups.get(&current) {
            deps.extend(group.packages.clone());
            checked.insert(current.clone());

            // Add dependencies for check
            for dep in &group.dependencies {
                if !checked.contains(dep) {
                    to_check.push(dep.clone());
                }
            }
        }
    }

    deps.sort();
    deps.dedup();
    deps
}

// Holy shitcode
// (but it works, somehow)
