use dirs::{config_dir, document_dir};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub data_location: String,
}

fn create_default_config() -> Result<(), Error> {
    let mut config_path = config_dir()
        .ok_or("Could not find config directory")
        .unwrap();

    config_path.push("mddo");
    fs::create_dir_all(&config_path)?;

    config_path.push("config.toml");

    let default_config = Config {
        data_location: format!(
            "{}/mddo/",
            document_dir()
                .ok_or("Could not find document directory")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        ),
    };

    let _ = fs::write(config_path, toml::to_string(&default_config).unwrap());

    Ok(())
}

pub fn load_config() -> Result<Config, Error> {
    let mut config_path = config_dir()
        .ok_or("Could not find config directory")
        .unwrap();

    config_path.push("mddo");
    config_path.push("config.toml");

    if !config_path.try_exists().is_err() {
        let _ = create_default_config();
    }

    let config = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config).unwrap();

    Ok(config)
}
