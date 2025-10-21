use crate::file::paths::home_directory;
use crate::env::default_values::{CONFIG_FILE_VARIABLE, CONFIG_FILE_DEFAULT};
use std::env;

pub fn config_file_location() -> String {
    if let Ok(file_name) = env::var(CONFIG_FILE_VARIABLE) {
        file_name
    } else {
        home_directory() + "/" + CONFIG_FILE_DEFAULT
    }
}

