use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Default)]
pub struct GitSettings {
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub dotfiles_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PackageGroup {
    pub packages: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct LaptopSettings {
    pub enable_tlp: bool,
    pub enable_auto_cpufreq: bool,
}

// Global configuration variables
pub static GIT_CONFIG: OnceLock<GitSettings> = OnceLock::new();
pub static PACKAGE_GROUPS: OnceLock<HashMap<String, PackageGroup>> = OnceLock::new();
pub static LAPTOP_CONFIG: OnceLock<LaptopSettings> = OnceLock::new();

// Accessor functions
pub fn git() -> &'static GitSettings {
    GIT_CONFIG.get().expect("Git config not initialized")
}

pub fn package_groups() -> &'static HashMap<String, PackageGroup> {
    PACKAGE_GROUPS.get().expect("Package groups not initialized")
}

pub fn package_group(name: &str) -> Option<&'static PackageGroup> {
    package_groups().get(name)
}

pub fn laptop() -> &'static LaptopSettings {
    LAPTOP_CONFIG.get().expect("Laptop config not initialized")
}

pub fn dotfiles_repo() -> Option<&'static String> {
    git().repo.as_ref()
}

pub fn dotfiles_branch() -> Option<&'static String> {
    git().branch.as_ref()
}

pub fn dotfiles_path() -> Option<&'static String> {
    git().dotfiles_path.as_ref()
}

pub fn is_tlp_enabled() -> bool {
    laptop().enable_tlp
}

pub fn is_auto_cpufreq_enabled() -> bool {
    laptop().enable_auto_cpufreq
}

pub fn get_packages_for_groups(group_names: &[String]) -> Vec<String> {
    let mut packages = Vec::new();
    let groups = package_groups();
    
    for name in group_names {
        if let Some(group) = groups.get(name) {
            packages.extend(group.packages.clone());
        }
    }
    
    packages.sort();
    packages.dedup();
    packages
}

// Internal initialization function
pub(crate) fn initialize(
    git: GitSettings,
    groups: HashMap<String, PackageGroup>,
    laptop: LaptopSettings,
) {
    GIT_CONFIG.set(git).unwrap();
    PACKAGE_GROUPS.set(groups).unwrap();
    LAPTOP_CONFIG.set(laptop).unwrap();
}
