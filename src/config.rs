use std::path::PathBuf;
use dirs::{config_dir, document_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub data_location: PathBuf,
}

fn create_default_config(config_path: &std::path::Path) -> Result<(), Error> {
    let mut data_location = document_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find document directory"))?;
    data_location.push("mddo");

    let default_config = Config { data_location };

    let _ = fs::write(config_path, toml::to_string(&default_config).unwrap());

    Ok(())
}

pub fn load_config() -> Result<Config, Error> {
    let mut config_path = config_dir()
        .ok_or("Could not find config directory")
        .unwrap();
    config_path.push("mddo");

    match config_path.try_exists() {
        Ok(exists) => {
            if !exists {
                fs::create_dir(&config_path)?;
            }
        }
        Err(_) => {
            fs::create_dir(&config_path)?;
        }
    }

    config_path.push("config.toml");

    match config_path.try_exists() {
        Ok(exists) => {
            if !exists {
                create_default_config(&config_path)?;
            }
        }
        Err(_) => {
            create_default_config(&config_path)?;
        }
    }

    let config = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config).unwrap();

    Ok(config)
}
