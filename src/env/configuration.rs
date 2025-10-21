use serde::Deserialize;
use crate::file::paths::home_directory;
use crate::env::default_values::{CONFIG_FILE_VARIABLE, CONFIG_FILE_DEFAULT};
use std::env;
use std::io::Result;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub width: i32,
    pub height: i32,
    pub database_file: String,
}


pub fn config_file_location() -> String {
    if let Ok(file_name) = env::var(CONFIG_FILE_VARIABLE) {
        file_name
    } else {
        home_directory() + "/" + CONFIG_FILE_DEFAULT
    }
}

pub fn get_configuration() -> Result<Config> {
    match fs::read_to_string(config_file_location()) {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(err) => Err(std::io::Error::other(err)),
        },
        Err(err) => Err(err),
    }
}


