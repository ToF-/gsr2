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

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn check_path_return_error_on_non_existent_path() {
        let dir: String = check_path("/this_dir_cant_exist").unwrap();
        assert_eq!(String::from("/this_dir_cant_exist"), dir);
    }
}
