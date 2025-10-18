use crate::env::default_values::GARBAGE;
use crate::env::default_values::THUMB_SUFFIX;
use crate::env::default_values::VALID_EXTENSIONS;
use crate::model::thumbnail::{thumbnail_size_display, thumbnail_size_for};
use std::ffi::OsStr;
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
    check_path_does_not_contain_garbage(path).and_then(|path| {
        if let Some(extension) = path.extension()
            && VALID_EXTENSIONS.contains(&extension.to_str().unwrap())
        {
            return Ok(path);
        }
        Err(Error::other(format!(
            "{} is not a jpg or png file",
            path.display()
        )))
    })
}

pub fn check_path_does_not_contain_garbage(path: &PathBuf) -> Result<&PathBuf> {
    for ch in path.display().to_string().chars() {
        if GARBAGE.contains(ch) {
            return Err(Error::other(format!(
                "{} is not a valid jpg or png file",
                path.display()
            )));
        }
    }
    Ok(path)
}

pub fn check_picture_file(file_name: &str) -> Result<String> {
    match check_path_exists(&PathBuf::from(file_name))
        .and_then(check_path_is_a_file)
        .and_then(check_path_is_a_jpg_or_png_file)
        .and_then(check_path_does_not_contain_garbage)
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

#[allow(dead_code)]
pub fn file_name_from(file_path: &str) -> String {
    let path: PathBuf = PathBuf::from(file_path);
    path.file_name()
        .expect("can't extract file_name")
        .to_str()
        .unwrap()
        .to_string()
}

fn thumbnail_name_for_path(
    path: PathBuf,
    file_stem: &OsStr,
    extension: &OsStr,
    pictures_per_row: usize,
) -> String {
    let thumb_file_name = PathBuf::from(
        file_stem.to_str().unwrap().to_owned()
            + THUMB_SUFFIX
            + &thumbnail_size_display(thumbnail_size_for(pictures_per_row)),
    )
    .with_extension(extension);
    path.with_file_name(thumb_file_name)
        .to_str()
        .unwrap()
        .to_string()
}
pub fn thumbnail_name_from(file_name: &str, pictures_per_row: usize) -> String {
    let path: PathBuf = PathBuf::from(file_name);
    let extension = path.extension().expect("can't compute path extension");
    let file_stem = path.file_stem().expect("can't compute path file stem");
    thumbnail_name_for_path(path.clone(), file_stem, extension, pictures_per_row)
}

pub fn thumbnail_names_from(file_name: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    result.push(thumbnail_name_from(file_name, 10));
    result.push(thumbnail_name_from(file_name, 7));
    result.push(thumbnail_name_from(file_name, 4));
    result.push(thumbnail_name_from(file_name, 2));
    result
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
    #[should_panic]
    fn check_path_return_error_on_several_extensions() {
        let _ = check_picture_file("testdata/nine_colors.png!Large.png").unwrap();
    }
    #[test]
    fn thumbnail_name_from_normal_file_has_thumb_suffix() {
        const PICTURES_PER_ROW: usize = 10;
        assert_eq!(
            "testdata/my_fileTHUMBSmall.jpg",
            thumbnail_name_from("testdata/my_file.jpg", PICTURES_PER_ROW)
        );
        assert_eq!(
            "testdata/my_other_fileTHUMBSmall.PNG",
            thumbnail_name_from("testdata/my_other_file.PNG", PICTURES_PER_ROW)
        )
    }
    #[test]
    fn thumbnail_names_for_all_grid_sizes() {
        assert_eq!(
            vec![
                String::from("testdata/my_fileTHUMBSmall.jpg"),
                String::from("testdata/my_fileTHUMBMedium.jpg"),
                String::from("testdata/my_fileTHUMBLarge.jpg"),
                String::from("testdata/my_fileTHUMBLarger.jpg")
            ],
            thumbnail_names_from("testdata/my_file.jpg")
        )
    }
}
