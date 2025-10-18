use crate::file::picture_file::get_data_from_picture_file;
use crate::model::palette::Palette;
use image::Rgb;
use std::collections::HashSet;
use std::io::Result;
use std::time::SystemTime;

pub type Rgb8 = Rgb<u8>;
pub type FileSize = u64;
#[allow(dead_code)]
pub struct PictureFileData(pub FileSize, pub SystemTime);
pub type Tags = HashSet<String>;

#[derive(Debug, Clone)]
pub struct ImageData {
    pub label: String,
    pub size: FileSize,
    pub modified_time: SystemTime,
    pub palette: Palette,
    pub tags: Tags,
}

impl ImageData {
    pub fn new(label: &str) -> Self {
        ImageData {
            label: label.to_string(),
            size: 0,
            modified_time: SystemTime::now(),
            palette: Palette::new(vec![], 0),
            tags: HashSet::new(),
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        get_data_from_picture_file(file_path).and_then(|file_data| {
            Ok(ImageData {
                label: String::from(""),
                size: file_data.0,
                modified_time: file_data.1,
                palette: Palette::new(vec![], 0),
                tags: HashSet::new(),
            })
        })
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    #[allow(dead_code)]
    pub fn modified_time(&self) -> SystemTime {
        self.modified_time
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::picture_file::get_palette_from_picture_file;
    use crate::test_data::*;
    use image::DynamicImage;

    #[test]
    fn extract_a_palette_of_9_most_used_colors() {
        let image: DynamicImage = gen_nine_colors();
        let palette = Palette::from(&image);
        let sample = palette.sample();
        // assert_eq!(Rgb([4, 4, 4]), sample[0]);
        // assert_eq!(Rgb([4, 4, 252]), sample[1]);
        // assert_eq!(Rgb([4, 132, 132]), sample[2]);
        // assert_eq!(Rgb([136, 100, 76]), sample[3]);
        // assert_eq!(Rgb([156, 204, 52]), sample[4]);
        // assert_eq!(Rgb([236, 132, 236]), sample[5]);
        // assert_eq!(Rgb([252, 4, 4]), sample[6]);
        // assert_eq!(Rgb([252, 140, 4]), sample[7]);
        // assert_eq!(Rgb([252, 252, 4]), sample[8]);
    }

    #[test]
    fn extract_a_palette_from_a_picture_file() {
        let image: DynamicImage = gen_nine_colors();
        let palette = Palette::from(&image);
        let sample = palette.sample();
        // assert_eq!(Rgb([4, 4, 4]), sample[0]);
        // assert_eq!(Rgb([4, 4, 252]), sample[1]);
        // assert_eq!(Rgb([4, 132, 132]), sample[2]);
        // assert_eq!(Rgb([136, 100, 76]), sample[3]);
        // assert_eq!(Rgb([156, 204, 52]), sample[4]);
        // assert_eq!(Rgb([236, 132, 236]), sample[5]);
        // assert_eq!(Rgb([252, 4, 4]), sample[6]);
        // assert_eq!(Rgb([252, 140, 4]), sample[7]);
        // assert_eq!(Rgb([252, 252, 4]), sample[8]);
    }

    #[test]
    fn extract_size_from_a_picture_file() {
        let file_data = get_data_from_picture_file(NINE_COLORS).unwrap();
        assert_eq!(49746, file_data.0);
    }
}
