use crate::file::paths::{file_name_from, thumbnail_name_from};
use crate::model::cover::Cover;
use crate::model::image_data::FileSize;
use crate::model::image_data::ImageData;
use crate::model::image_data::datetime_from_time_stamp;
use crate::model::rank::Rank;
use crate::model::selection_criteria::SelectionCriteria;
use crate::model::tags::Tags;
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
        picture.set_image_data(ImageData::new_with_label(label));
        picture
    }

    pub fn new_with_image_data(file_path: &str, image_data: &ImageData) -> Self {
        Picture {
            file_path: file_path.to_string(),
            image_data: Some(image_data.clone()),
        }
    }

    pub fn new_with_file_image_data(file_path: &str, label: &str) -> Result<Self> {
        ImageData::from_file(file_path).map(|image_data| {
            let new_image_data = ImageData {
                label: label.to_string(),
                ..image_data
            };
            Self::new_with_image_data(file_path, &new_image_data)
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
        self.image_data
            .as_ref()
            .map(|d| d.label())
            .unwrap_or_default()
    }

    pub fn selected(&self, selection_criteria: &SelectionCriteria) -> bool {
        self.image_data
            .as_ref()
            .map(|d| selection_criteria.matches(d.tags.clone()))
            .unwrap_or_default()
    }

    pub fn rank(&self) -> Rank {
        self.image_data
            .as_ref()
            .map(|d| d.rank())
            .unwrap_or_default()
    }

    pub fn tags(&self) -> Tags {
        self.image_data
            .as_ref()
            .map(|d| d.tags.clone())
            .unwrap_or_default()
    }

    pub fn increment_score(&mut self, score: u32) {
        let mut new_image_data = self.image_data().unwrap_or_default();
        new_image_data.increment_score(score);
        self.set_image_data(new_image_data);
    }
    pub fn cover(&self) -> Cover {
        if let Some(image_data) = &self.image_data {
            image_data.cover
        } else {
            None
        }
    }

    pub fn set_image_data(&mut self, image_data: ImageData) {
        self.image_data = Some(image_data)
    }
    pub fn set_label(&mut self, label: &str) {
        let mut new_image_data = self.image_data().unwrap_or_default();
        new_image_data.set_label(label);
        self.set_image_data(new_image_data);
    }

    pub fn set_rank(&mut self, rank: Rank) {
        let mut new_image_data = self.image_data().unwrap_or_default();
        new_image_data.set_rank(rank);
        self.set_image_data(new_image_data);
    }

    pub fn toggle_cover(&mut self, dir_count: usize) {
        let mut new_image_data = self.image_data().unwrap_or_default();
        new_image_data.toggle_cover(dir_count);
        self.set_image_data(new_image_data);
    }

    pub fn add_tag(&mut self, label: &str) {
        let mut new_image_data = self.image_data().unwrap_or_default();
        let _ = new_image_data.add_tag(label);
        self.set_image_data(new_image_data);
    }

    pub fn remove_tag(&mut self, label: &str) {
        let mut new_image_data = self.image_data().unwrap_or_default();
        let _ = new_image_data.remove_tag(label);
        self.set_image_data(new_image_data);
    }

    pub fn thumbnail_file_path_for_size(&self, pictures_per_row: usize) -> String {
        thumbnail_name_from(&self.file_path, pictures_per_row)
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
    fn modifiying_a_picture() {
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
