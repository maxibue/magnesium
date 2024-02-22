use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub parent_directory: String,
    pub buckets: Vec<String>,
    pub serve_as_webp: bool,
    pub allow_resizing: bool,
    pub allow_admin: bool,
    pub db_name: String,
    pub collection_name: String,
}

fn read_config_from_toml<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    read_config_from_toml("config.toml")
        .expect("Failed to read configuration")
});