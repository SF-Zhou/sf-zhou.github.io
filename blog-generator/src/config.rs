use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub site_name: String,
    pub site_url: String,
    pub site_description: String,
    #[serde(default = "default_language")]
    pub site_language: String,
    pub posts_path: String,
    pub output_path: String,
    #[serde(default)]
    pub profile_path: Option<String>,
    #[serde(default = "default_format")]
    pub article_format: String,
    pub default_author: String,
    pub web_master: String,
    #[serde(default)]
    pub github_repo: Option<String>,
    #[serde(default)]
    pub github_repo_id: Option<String>,
    #[serde(default)]
    pub google_analytics_id: Option<String>,
}

fn default_language() -> String {
    "zh-CN".to_string()
}

fn default_format() -> String {
    "md".to_string()
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
