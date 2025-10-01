use crate::env::default_values::{DATABASE_CONNECTION_VAR, TEST_DATABASE_FILE};
use std::env;
use std::io::{Error, Result};

pub fn database_connection() -> Result<String> {
    match env::var(DATABASE_CONNECTION_VAR) {
        Ok(result) => Ok(result),
        Err(err) => {
            if cfg!(test) {
                println!(
                    "test environment. {} is {}",
                    DATABASE_CONNECTION_VAR, TEST_DATABASE_FILE
                );
                Ok(TEST_DATABASE_FILE.to_string())
            } else {
                eprintln!("cannot find environment variable {DATABASE_CONNECTION_VAR}");
                Err(Error::other(err))
            }
        }
    }
}
