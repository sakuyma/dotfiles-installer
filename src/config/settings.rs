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

#[derive(Debug, Clone, Default)]
pub struct SystemSettings {
    pub hostname: Option<String>,
    pub locale: Option<String>,
    pub localtime: Option<String>,
    pub sudoers: Option<String>,
    pub hosts: Option<String>,
}

// Global configuration variables
pub static GIT_CONFIG: OnceLock<GitSettings> = OnceLock::new();
pub static PACKAGE_GROUPS: OnceLock<HashMap<String, PackageGroup>> = OnceLock::new();
pub static LAPTOP_CONFIG: OnceLock<LaptopSettings> = OnceLock::new();
pub static SYSTEM_CONFIG: OnceLock<SystemSettings> = OnceLock::new();

// Accessor functions
pub fn git() -> &'static GitSettings {
    GIT_CONFIG.get().expect("Git config not initialized")
}

pub fn package_groups() -> &'static HashMap<String, PackageGroup> {
    PACKAGE_GROUPS
        .get()
        .expect("Package groups not initialized")
}

pub fn package_group(name: &str) -> Option<&'static PackageGroup> {
    package_groups().get(name)
}

pub fn laptop() -> &'static LaptopSettings {
    LAPTOP_CONFIG.get().expect("Laptop config not initialized")
}

pub fn system() -> &'static SystemSettings {
    SYSTEM_CONFIG.get().expect("System config not initialized")
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

pub fn system_hostname() -> Option<&'static String> {
    system().hostname.as_ref()
}

pub fn system_locale() -> Option<&'static String> {
    system().locale.as_ref()
}

pub fn system_localtime() -> Option<&'static String> {
    system().localtime.as_ref()
}

pub fn system_sudoers() -> Option<&'static String> {
    system().sudoers.as_ref()
}

pub fn system_hosts() -> Option<&'static String> {
    system().hosts.as_ref()
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
    system: SystemSettings,
) {
    GIT_CONFIG.set(git).unwrap();
    PACKAGE_GROUPS.set(groups).unwrap();
    LAPTOP_CONFIG.set(laptop).unwrap();
    SYSTEM_CONFIG.set(system).unwrap();
}
