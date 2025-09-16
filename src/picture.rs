use crate::default_values::THUMB_SUFFIX;
use crate::paths::thumbnail_name_from;
use std::path::PathBuf;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Picture {
    file_name: String,
    thumbnail_file_name: String,
}

impl Picture {
    pub fn new(file_name: &str) -> Self {
        Picture {
            file_name: file_name.to_string(),
            thumbnail_file_name: thumbnail_name_from(file_name),
        }
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn thumbnail_file_name(&self) -> String {
        self.thumbnail_file_name.clone()
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_picture_as_file_name_which_is_the_full_path_and_file_name_on_the_file_system() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(
            String::from("testdata/nine_colors.png"),
            picture.file_name()
        )
    }

    #[test]
    fn a_thumbnail_picture_has_the_name_as_the_original_picture_with_suffix_THUMB() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(
            String::from("testdata/nine_colorsTHUMB.png"),
            picture.thumbnail_file_name()
        )
    }
}
