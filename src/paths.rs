use std::io::{ErrorKind, Result, Error};
use std::path::PathBuf;

fn check_path_exists(path: &PathBuf) -> Result<String> {
    Ok(path.display().to_string())
}
pub fn check_path(source: &str) -> Result<String> {
    let path = PathBuf::from(source);
    if path.exists() {
        if path.is_dir() {
            Ok(source.to_string())
        } else {
            Err(Error::new(
                    ErrorKind::NotADirectory,
                    format!("{} is not a directory", source))
            )
        }
    } else {
        Err(Error::new(
                ErrorKind::NotFound,
                format!("directory {} doesn't exist", source))
            )
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
    #[test]
    #[should_panic]
    fn check_path_return_error_on_path_that_is_not_a_directory() {
        let dir: String = check_path("./src/paths.rs").unwrap();
        assert_eq!(String::from("./src/paths.rs"), dir);
    }
}
