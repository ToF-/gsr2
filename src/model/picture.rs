use crate::file::paths::{file_name_from, thumbnail_name_from};
use crate::model::cover::Cover;
use crate::model::image_data::FileSize;
use crate::model::image_data::ImageData;
use crate::model::image_data::datetime_from_time_stamp;
use crate::model::rank::Rank;
use crate::model::selection::Selection;
use crate::model::tags::{Tags, empty};
use std::io::Result;

#[derive(Debug, Clone)]
pub struct Picture {
    file_path: String,
    image_data: Option<ImageData>,
}

impl Picture {
    pub fn new(file_path: &str) -> Self {
        Picture {
            file_path: file_path.to_string(),
            image_data: None,
        }
    }
    pub fn copy(original: &Self, file_path: &str) -> Self {
        let mut picture: Picture = Self::new(file_path);
        if let Some(image_data) = &original.image_data {
            picture.set_image_data(image_data.clone())
        };
        picture
    }

    pub fn new_with_label(file_path: &str, label: &str) -> Self {
        let mut picture: Picture = Self::new(file_path);
        picture.set_image_data(ImageData::new(label));
        picture
    }

    pub fn new_with_image_data(file_path: &str, image_data: &ImageData) -> Self {
        let mut picture: Picture = Self::new(file_path);
        picture.set_image_data(image_data.clone());
        picture
    }

    pub fn new_with_file_image_data(file_path: &str, label: &str) -> Result<Self> {
        ImageData::from_file(file_path).and_then(|image_data| {
            let new_image_data = ImageData {
                label: label.to_string(),
                ..image_data
            };
            let mut picture: Picture = Self::new(file_path);
            picture.set_image_data(new_image_data);
            Ok(picture)
        })
    }

    pub fn file_path(&self) -> String {
        self.file_path.clone()
    }

    pub fn modified_time_display(&self) -> String {
        if let Some(image_data) = &self.image_data {
            datetime_from_time_stamp(image_data.modified_time())
                .format("%Y-%m-%d %H:%M:%S%.f")
                .to_string()
        } else {
            String::from("…/…")
        }
    }

    pub fn view_file_path(&self, pictures_per_row: usize) -> String {
        if pictures_per_row > 1 {
            self.thumbnail_file_path_for_size(pictures_per_row)
        } else {
            self.file_path()
        }
    }

    pub fn file_name(&self) -> String {
        file_name_from(&self.file_path)
    }

    pub fn image_data(&self) -> Option<ImageData> {
        self.image_data.clone()
    }

    pub fn file_size(&self) -> Option<FileSize> {
        self.image_data.as_ref().map(|image_data| image_data.size)
    }

    pub fn label(&self) -> String {
        if let Some(image_data) = &self.image_data {
            image_data.label()
        } else {
            String::from("")
        }
    }

    pub fn label_sort_key(&self) -> String {
        if let Some(image_data) = &self.image_data {
            if !image_data.label().is_empty() {
                image_data.label()
            } else {
                String::from("~")
            }
        } else {
            String::from("~")
        }
    }

    pub fn selected(&self, selection: &Selection) -> bool {
        if let Some(image_data) = &self.image_data {
            selection.matches(image_data.tags.clone())
        } else {
            false
        }
    }

    pub fn rank(&self) -> Rank {
        if let Some(image_data) = &self.image_data {
            image_data.rank()
        } else {
            Rank::NoStar
        }
    }

    pub fn tags(&self) -> Tags {
        if let Some(image_data) = &self.image_data {
            image_data.tags.clone()
        } else {
            empty()
        }
    }
    pub fn add_tag(&mut self, label: &str) {
        let mut new_image_data: ImageData = match &self.image_data {
            Some(image_data) => image_data.clone(),
            None => ImageData::new(""),
        };
        let _ = new_image_data.tags.insert(label.to_string());
        self.image_data = Some(new_image_data.clone());
    }

    pub fn remove_tag(&mut self, label: &str) {
        let mut new_image_data: ImageData = match &self.image_data {
            Some(image_data) => image_data.clone(),
            None => ImageData::new(""),
        };
        let _ = new_image_data.tags.remove(label);
        self.image_data = Some(new_image_data.clone());
    }

    pub fn set_label(&mut self, label: &str) {
        let new_image_data = if let Some(image_data) = &self.image_data {
            ImageData {
                label: label.to_string(),
                ..image_data.clone()
            }
        } else {
            ImageData::new(label)
        };
        self.image_data = Some(new_image_data)
    }

    pub fn set_rank(&mut self, rank: Rank) {
        let new_image_data = if let Some(image_data) = &self.image_data {
            ImageData {
                rank,
                ..image_data.clone()
            }
        } else {
            ImageData::new("")
        };
        self.image_data = Some(new_image_data)
    }

    pub fn toggle_cover(&mut self, dir_count: usize) {
        let mut new_image_data = if let Some(image_data) = &self.image_data {
            image_data.clone()
        } else {
            ImageData::new("")
        };
        if self.cover().is_some() {
            new_image_data.cover = None;
        } else {
            new_image_data.cover = Some(dir_count)
        }
        self.image_data = Some(new_image_data);
    }

    pub fn cover(&self) -> Cover {
        if let Some(image_data) = &self.image_data {
            image_data.cover
        } else {
            None
        }
    }

    pub fn thumbnail_file_path_for_size(&self, pictures_per_row: usize) -> String {
        thumbnail_name_from(&self.file_path, pictures_per_row)
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
    fn mofiying_a_picture() {
        let mut picture = Picture::new("testdata/nine_colors.png");
        picture.set_label("foo");
        picture.set_rank(Rank::ThreeStars);
        assert_eq!(String::from("foo"), picture.label());
        assert_eq!(Rank::ThreeStars, picture.rank());
        assert!(picture.tags().is_empty());
        picture.add_tag("foo");
        assert!(picture.tags().contains("foo"));
    }
    #[test]
    fn copying_a_picture_with_a_different_file_path() {
        let mut original = Picture::new("testdata/nine_colors.png");
        original.set_label("foo");
        original.set_rank(Rank::ThreeStars);
        original.add_tag("bar");
        let picture = Picture::copy(&original, "testdata/other.png");
        assert_eq!(String::from("foo"), picture.label());
        assert_eq!(Rank::ThreeStars, picture.rank());
        assert!(picture.tags().contains("bar"));
    }

    #[test]
    fn a_thumbnail_picture_has_the_name_of_the_original_picture_with_thumb_and_size_suffix() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(
            String::from("testdata/nine_colorsTHUMBSmall.png"),
            picture.thumbnail_file_path_for_size(10)
        );
        assert_eq!(
            String::from("testdata/nine_colorsTHUMBMedium.png"),
            picture.thumbnail_file_path_for_size(5)
        );
        assert_eq!(
            String::from("testdata/nine_colorsTHUMBLarge.png"),
            picture.thumbnail_file_path_for_size(4)
        );
        assert_eq!(
            String::from("testdata/nine_colorsTHUMBLarger.png"),
            picture.thumbnail_file_path_for_size(2)
        );
    }

    #[test]
    fn a_picture_has_a_file_name() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(String::from("nine_colors.png"), picture.file_name())
    }

    #[test]
    fn a_picture_view_file_name_depends_on_pictures_per_row() {
        let picture = Picture::new("testdata/nine_colors.png");
        assert_eq!(
            String::from("testdata/nine_colors.png"),
            picture.view_file_path(0)
        );
        assert_eq!(
            String::from("testdata/nine_colorsTHUMBSmall.png"),
            picture.view_file_path(10)
        );
        assert_eq!(
            String::from("testdata/nine_colorsTHUMBLarger.png"),
            picture.view_file_path(2)
        );
    }
    #[test]
    fn set_label_changes_image_data() {
        let mut picture = Picture::new("testdata/nine_colors.png");
        picture.set_label("foo-bar-qux");
        assert_eq!(String::from("foo-bar-qux"), picture.label());
    }
}
