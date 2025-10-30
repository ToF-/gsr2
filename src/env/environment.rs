use crate::env::configuration::Configuration;
use std::io::Result;

pub fn database_connection(config: Configuration) -> Result<String> {
    Ok(config.database_file)
}
