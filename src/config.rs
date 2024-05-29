use std::io::Error;
use dirs::config_dir;
use std::fs;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub data_location: String,
}

pub fn load_config() -> Result<Config, Error> {
    let mut config_path = config_dir().ok_or("Could not find config directory").unwrap();

    config_path.push("mddo");
    config_path.push("config.toml");

    let config = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config).unwrap();

    Ok(config)
}
