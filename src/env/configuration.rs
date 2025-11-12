use crate::env::default_values::{CONFIG_FILE_DEFAULT, CONFIG_FILE_VARIABLE};
use crate::file::paths::home_directory;
use crate::file_exists;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub width: i32,
    pub height: i32,
    pub database_file: String,
    pub temp_dir: String,
}

pub fn config_file_location() -> String {
    if let Ok(file_name) = env::var(CONFIG_FILE_VARIABLE) {
        file_name
    } else {
        home_directory() + "/" + CONFIG_FILE_DEFAULT
    }
}

pub fn get_configuration() -> Result<Configuration> {
    if !file_exists(&config_file_location()) {
        return Err(std::io::Error::other(format!(
            "configuration file {} does not exist",
            config_file_location()
        )));
    };
    println!("configuration: {}", config_file_location());
    match fs::read_to_string(config_file_location()) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(err) => Err(std::io::Error::other(err)),
        },
        Err(err) => Err(err),
    }
}

#[cfg(test)]

pub mod tests {
    use super::*;
    use crate::test_data::TEST_DATA_DIR;
    use crate::file::paths::test::current_directory;

    pub fn my_cfg() -> Configuration {
        Configuration {
            width: 1000,
            height: 1000,
            database_file: format!("{}/{}/gsr2.db", current_directory(), TEST_DATA_DIR),
        }
    }
}
