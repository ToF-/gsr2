use crate::default_values::THUMB_SUFFIX;
use crate::default_values::VALID_EXTENSIONS;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

pub fn check_path_exists(path: &PathBuf) -> Result<&PathBuf> {
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("not found: {}", path.display()),
        ))
    }
}

pub fn check_path_is_directory(path: &PathBuf) -> Result<&PathBuf> {
    if path.is_dir() {
        Ok(path)
    } else {
        Err(Error::new(
            ErrorKind::NotADirectory,
            format!("{} is not a directory", path.display()),
        ))
    }
}

pub fn check_path_is_a_file(path: &PathBuf) -> Result<&PathBuf> {
    if path.is_file() {
        Ok(path)
    } else {
        Err(Error::other(format!("{} is not a file", path.display())))
    }
}

pub fn check_path_is_a_jpg_or_png_file(path: &PathBuf) -> Result<&PathBuf> {
    if let Some(extension) = path.extension()
        && VALID_EXTENSIONS.contains(&extension.to_str().unwrap())
    {
        return Ok(path);
    }
    Err(Error::other(format!(
        "{} is not a jpg or png file",
        path.display()
    )))
}

pub fn check_picture_file(file_name: &str) -> Result<String> {
    match check_path_exists(&PathBuf::from(file_name))
        .and_then(check_path_is_a_file)
        .and_then(check_path_is_a_jpg_or_png_file)
    {
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
pub fn thumbnail_name_from(file_name: &str) -> String {
    let path: PathBuf = PathBuf::from(file_name);
    let result = path.parent().and_then(|parent| {
        path.extension().and_then(|extension| {
            path.file_stem().and_then(|file_stem| {
                let new_file_name = format!(
                    "{}{}.{}",
                    file_stem.to_str().unwrap(),
                    THUMB_SUFFIX,
                    extension.to_str().unwrap()
                );
                let new_path = parent.join(new_file_name);
                Some(new_path.to_str().unwrap().to_string())
            })
        })
    });
    result.expect(&format!(
        "can't convert {} to file_thumbnail_name",
        file_name
    ))
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn a_readme_file_is_not_a_picture_file() {
        let path: PathBuf = PathBuf::from("README.md");
        let _ = check_path_is_a_jpg_or_png_file(&path).unwrap();
    }

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

    #[test]
    fn thumbnail_name_from_normal_file_has_THUMB_suffix() {
        assert_eq!(
            "testdata/my_fileTHUMB.jpg",
            thumbnail_name_from("testdata/my_file.jpg")
        )
    }
}
