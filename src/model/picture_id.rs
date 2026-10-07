use crate::file::paths::home_directory;
use crate::env::default_values::HOME_DIRECTORY_SYMBOL;
use crate::file::paths::file_path_as_stored;
use crate::env::default_values::BASE_DIRECTORY_SYMBOL;
use crate::env::configuration::CONFIGURATION;
use std::path::PathBuf;
use std::ops::Deref;
// the type used to uniquely identify a picture

#[derive(Clone, Debug)]
pub struct PictureId {
    value: String,
}

impl Deref for PictureId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PictureId {
    pub fn from_str(s: &str) -> Self {
        Self {
            value: file_path_as_stored(s).to_string()
        }
    }
    pub fn file_name(&self) -> String {
        let path: PathBuf = PathBuf::from(&self.value);
        path.file_name()
            .expect("can't extract file_name from picture_id")
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_name_is_part_of_the_picture_id() {
        let picture_id = PictureId::from_str("/foo/bar/qux.jpg");
        assert_eq!("qux.jpg".to_string(), picture_id.file_name());
    }

    #[test]
    fn picture_id_is_the_file_path_as_stored_with_base_prefix() {
        let base_dir = &CONFIGURATION.get().unwrap().base_dir;
        let original_file_path = format!("{}/foo/bar/qux.jpg", &base_dir);
        let picture_id = PictureId::from_str(&original_file_path);
        let picture_file_path = format!("{}/foo/bar/qux.jpg", BASE_DIRECTORY_SYMBOL);
        assert_eq!(picture_file_path, picture_id.to_string());
    }
    #[test]
    fn picture_id_is_the_file_path_as_stored_with_home_prefix_if_not_base_prefix() {
        let original_file_path = format!("{}/tmp/4807.jpg", &home_directory());
        let picture_id = PictureId::from_str(&original_file_path);
        let picture_file_path = format!("{}/tmp/4807.jpg", HOME_DIRECTORY_SYMBOL);
        assert_eq!(picture_file_path, picture_id.to_string());
    }
    #[test]
    fn picture_id_is_the_file_path_as_stored_as_is_if_neither_base_or_home_prefix() {
        let original_file_path = "/Volumes/another_disk/tmp/4807.jpg";
        let picture_id = PictureId::from_str(&original_file_path);
        assert_eq!(original_file_path, picture_id.to_string());
    }
}
