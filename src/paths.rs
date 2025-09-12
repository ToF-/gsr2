use std::io::{ErrorKind, Result};
use std::path::PathBuf;

pub fn check_path(source: &str) -> Result<String> {
    let path = PathBuf::from(source);
    if path.exists() {
        Ok(source.to_string())
    } else {
        Err(ErrorKind::NotFound.into())
    }
}
