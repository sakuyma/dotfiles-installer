use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub git: Option<GitConfig>,
    pub packages: Option<PackagesConfig>,
    pub laptop: Option<LaptopConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GitConfig {
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub dotfiles_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackagesConfig {
    #[serde(flatten)]
    pub groups: HashMap<String, PackageGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageGroup {
    pub packages: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LaptopConfig {
    pub enable_tlp: Option<bool>,
    pub enable_auto_cpufreq: Option<bool>,
}
