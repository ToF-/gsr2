use std::io::Result;
use std::path::PathBuf;

pub fn check_path(source: &str) -> Result<String> {
    let path = PathBuf::from(source);

    Ok(source.to_string())
}
