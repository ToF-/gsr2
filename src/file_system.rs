use crate::default_values::THUMB_SUFFIX;
use crate::paths::{check_path, check_path_is_a_jpg_or_png_file, check_picture_file};
use std::io::Result;
use walkdir::WalkDir;

pub fn get_all_picture_file_paths(path: &str) -> Result<Vec<String>> {
    check_path(path).map(|directory| {
        let mut file_paths: Vec<String> = Vec::new();
        for _entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.into_path())
            .filter(|path| {
                path.is_file()
                    && check_path_is_a_jpg_or_png_file(path).is_ok()
                    && !path.display().to_string().contains(THUMB_SUFFIX)
            })
        {
            file_paths.push(path.to_string())
        }
        file_paths
    })
}

pub fn get_picture_file_path(file_path: &str) -> Result<String> {
    check_picture_file(file_path)
}
