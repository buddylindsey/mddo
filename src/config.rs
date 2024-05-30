use dirs::{config_dir, document_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub data_location: String,
}

fn create_default_config(config_path: &std::path::Path) -> Result<(), Error> {
    let data_dir = document_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find document directory"))?;
    let data_location = format!("{}/mddo/", data_dir.to_str().unwrap());

    let default_config = Config { data_location };

    let _ = fs::write(config_path, toml::to_string(&default_config).unwrap());

    Ok(())
}

pub fn load_config() -> Result<Config, Error> {
    let mut config_path = config_dir()
        .ok_or("Could not find config directory")
        .unwrap();
    config_path.push("mddo");

    if !config_path.try_exists().is_err() {
        fs::create_dir(&config_path)?;
    }

    config_path.push("config.toml");

    if !config_path.try_exists().is_err() {
        create_default_config(&config_path)?;
    }

    let config = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config).unwrap();

    Ok(config)
}
