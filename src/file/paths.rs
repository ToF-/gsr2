use crate::env::default_values::GARBAGE;
use crate::env::default_values::THUMB_SUFFIX;
use crate::env::default_values::VALID_EXTENSIONS;
use crate::model::thumbnail::{thumbnail_size_display, thumbnail_size_for};
use chrono::{Local, SecondsFormat};
use std::env::home_dir;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

pub fn home_directory() -> String {
    home_dir()
        .map(|path| path.display().to_string())
        .expect("can't access to home_dir")
}

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

pub fn parent_directory(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    path.parent()
        .map(|parent| parent.to_str().unwrap().to_string())
}

pub fn grand_parent_directory(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    match path.parent() {
        Some(parent) => parent
            .parent()
            .map(|grand_parent| grand_parent.to_str().unwrap().to_string()),
        None => None,
    }
}

pub fn file_exists(file_path: &str) -> bool {
    let path = PathBuf::from(file_path);
    check_path_exists(&path).is_ok()
}

pub fn check_collectable(path: &PathBuf) -> Result<&PathBuf> {
    check_path_is_directory(path).and_then(|path| {
        let home_dir = home_dir().unwrap_or("@".into());
        if path.starts_with(home_dir) || path.is_absolute() {
            Ok(path)
        } else {
            Err(Error::new(
                ErrorKind::NotADirectory,
                format!(
                    "{} is not absolute and not starting with home directory",
                    path.display()
                ),
            ))
        }
    })
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

pub fn check_picture_path_extension(path: &PathBuf) -> Result<&PathBuf> {
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
        .and_then(check_picture_path_extension)
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
    vec![
        thumbnail_name_from(file_name, 10),
        thumbnail_name_from(file_name, 7),
        thumbnail_name_from(file_name, 4),
        thumbnail_name_from(file_name, 2),
    ]
}

pub fn file_path_as_stored(source: &str) -> String {
    let home = home_directory();
    if !source.starts_with(&home) {
        return source.to_string();
    }
    let home_iter = home.chars();
    let mut source_iter = source.chars();
    for _ in home_iter {
        source_iter.next();
    }
    format!("~{}", source_iter.as_str())
}

pub fn file_path_as_retrieved(source: &str) -> String {
    if !source.starts_with("~") {
        return source.to_string();
    }
    let mut source_iter = source.chars();
    source_iter.next();
    format!("{}{}", home_directory(), source_iter.as_str())
}

pub fn renamed_file_path(file_path: &str, name: &str) -> String {
    let mut path: PathBuf = PathBuf::from(file_path);
    let binding = path.clone();
    let ext_opt = binding.extension();
    path.set_file_name(name);
    if let Some(ext) = ext_opt {
        path.set_extension(ext);
    }
    path.display().to_string()
}
pub fn timestamp_filename(prefix: &str, ext: &str) -> String {
    let now = Local::now();
    let stamp = now
        .to_rfc3339_opts(SecondsFormat::Millis, true)
        .replace(':', "-");
    format!(
        "{}{}{}.{}",
        prefix,
        if prefix.is_empty() { "" } else { "-" },
        stamp,
        ext
    )
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::env;

    #[test]
    #[should_panic]
    fn a_readme_file_is_not_a_picture_file() {
        let path: PathBuf = PathBuf::from("README.md");
        let _ = check_picture_path_extension(&path).unwrap();
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

    #[test]
    fn changing_the_file_name_of_a_file_path() {
        let file_path = "my/long/path/foo.ext";
        let new_name = "bar";
        assert_eq!(
            "my/long/path/bar.ext",
            renamed_file_path(&file_path, &new_name)
        );
    }
    #[test]
    fn file_path_starting_with_home_dir_are_tilded_as_stored() {
        if let Some(home_dir) = env::home_dir() {
            let home = home_dir.display().to_string();
            let file_path = format!("{home}/test_dir/{home}/file.jpg");
            let expected = format!("~/test_dir/{home}/file.jpg");
            assert_eq!(expected, file_path_as_stored(&file_path))
        }
    }
    #[test]
    fn file_path_not_starting_with_home_dir_are_not_tilded_as_stored() {
        if let Some(home_dir) = env::home_dir() {
            let home = home_dir.display().to_string();
            let file_path = format!("/other/{home}/test_file.jpg");
            assert_eq!(file_path, file_path_as_stored(&file_path))
        }
    }

    #[test]
    fn file_path_starting_with_tilde_are_developped_as_retrieved() {
        if let Some(home_dir) = env::home_dir() {
            let home = home_dir.display().to_string();
            let file_path = "~/test_file/~/.jpg";
            let expected = format!("{home}/test_file/~/.jpg");
            assert_eq!(expected, file_path_as_retrieved(&file_path));
        }
    }

    #[test]
    fn file_path_not_starting_with_tilde_are_not_developped_as_retrieved() {
        if let Some(home) = env::home_dir() {
            let file_path = "/other/~/test_file.jpg";
            assert_eq!(file_path, file_path_as_retrieved(&file_path));
        }
    }

    #[test]
    fn having_parent_directory() {
        assert_eq!(
            Some("testdata/subdir".to_string()),
            parent_directory("testdata/subdir/my_file.jpg")
        );
        assert_eq!(Some("".to_string()), parent_directory(&format!("foo.jpg")))
    }
    #[test]
    fn having_grand_parent_directory() {
        assert_eq!(
            Some("testdata".to_string()),
            grand_parent_directory("testdata/subdir/my_file.jpg")
        );
        assert_eq!(None, grand_parent_directory(&format!("foo.jpg")))
    }
}

#[cfg(test)]

pub mod test {
    use super::*;
    use std::env::current_dir;

    pub fn current_directory() -> String {
        current_dir().unwrap().display().to_string()
    }
}
