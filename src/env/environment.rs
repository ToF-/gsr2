use crate::env::default_values::{DATABASE_CONNECTION_VAR, TEST_DATABASE_FILE};
use std::env;
use std::io::{Error, Result};
use crate::get_configuration;

pub fn database_connection() -> Result<String> {
    match get_configuration() {
        Ok(config) => Ok(config.database_file),
        Err(err) => Err(err)
    }
}
