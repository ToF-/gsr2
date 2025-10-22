use std::io::Result;
use crate::get_configuration;

pub fn database_connection() -> Result<String> {
    match get_configuration() {
        Ok(config) => Ok(config.database_file),
        Err(err) => Err(err)
    }
}
