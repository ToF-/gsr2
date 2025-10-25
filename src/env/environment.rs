use crate::get_configuration;
use std::io::Result;

pub fn database_connection() -> Result<String> {
    match get_configuration() {
        Ok(config) => Ok(config.database_file),
        Err(err) => Err(err),
    }
}
