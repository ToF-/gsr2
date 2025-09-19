use crate::image_data::ImageData;
use crate::paths::{file_name_from, thumbnail_name_from};

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Picture {
    file_path: String,
    thumbnail_file_path: String,
    image_data: Option<ImageData>,
}

impl Picture {
    pub fn new(file_path: &str) -> Self {
        Picture {
            file_path: file_path.to_string(),
            thumbnail_file_path: thumbnail_name_from(file_path),
            image_data: None,
        }
    }

    pub fn new_with_image_data(file_path: &str, label: &str) -> Self {
        let mut picture: Picture = Self::new(file_path);
        let image_data = ImageData::new(label);
        picture.set_image_data(image_data);
        picture
    }
    pub fn file_path(&self) -> String {
        self.file_path.clone()
    }

    pub fn file_name(&self) -> String {
        file_name_from(&self.file_path)
    }

    pub fn image_data(&self) -> Option<ImageData> {
        self.image_data.clone()
    }
    #[allow(dead_code)]
    pub fn thumbnail_file_path(&self) -> String {
        self.thumbnail_file_path.clone()
    }

    pub fn set_image_data(&mut self, image_data: ImageData) {
        self.image_data = Some(image_data)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_picture_has_file_path_which_is_the_full_path_and_file_path_on_the_file_system() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(
            String::from("testdata/nine_colors.png"),
            picture.file_path()
        )
    }

    #[test]
    fn a_thumbnail_picture_has_the_name_as_the_original_picture_with_thumb_suffix() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(
            String::from("testdata/nine_colorsTHUMB.png"),
            picture.thumbnail_file_path()
        )
    }

    #[test]
    fn a_picture_has_a_file_name() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(String::from("nine_colors.png"), picture.file_name())
    }
}
