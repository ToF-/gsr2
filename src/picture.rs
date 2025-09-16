use crate::default_values::THUMB_SUFFIX;
use std::path::PathBuf;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Picture {
    file_name: String,
}

impl Picture {
    pub fn new(file_name: &str) -> Self {
        Picture {
            file_name: file_name.to_string(),
        }
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn file_thumbnail_name(&self) -> String {
        let path: PathBuf = PathBuf::from(&self.file_name);
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
        result.expect("can't convert file_name to file_thumbnail_name")
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
            picture.file_thumbnail_name()
        )
    }
}
