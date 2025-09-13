use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

fn check_path_exists(path: &PathBuf) -> Result<&PathBuf> {
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("not found: {}", path.display()),
        ))
    }
}

fn check_path_is_directory(path: &PathBuf) -> Result<&PathBuf> {
    if path.is_dir() {
        Ok(path)
    } else {
        Err(Error::new(
            ErrorKind::NotADirectory,
            format!("{} is not a directory", path.display()),
        ))
    }
}

pub fn check_picture_file(file_name: &str) -> Result<String> {
    match check_path_exists(&PathBuf::from(file_name)) {
        Ok(path) => Ok(path.display().to_string()),
        Err(e) => Err(e),
    }
}

pub fn check_path(source: &str) -> Result<String> {
    match check_path_exists(&PathBuf::from(source)).and_then(check_path_is_directory) {
        Ok(path) => Ok(path.display().to_string()),
        Err(e) => Err(e),
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
