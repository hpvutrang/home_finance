use std::error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: Database,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub url: String,
    pub name: String,
    pub user: String,
    pub password: String,
    pub port: Option<String>, // Optional port for flexibility
}

pub fn load_config(file_name : String) -> Result<Config, Box<dyn error::Error>> {
    let config_content: String = std::fs::read_to_string(file_name)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}