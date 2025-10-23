use crate::model::label::{Label, from};
use crate::model::rank::Rank;
use chrono::{DateTime, Local};
use std::time::UNIX_EPOCH;
use crate::file::picture_file::get_data_from_picture_file;
use crate::model::palette::Palette;
use image::Rgb;
use std::collections::HashSet;
use std::io::Result;
use std::time::{Duration, SystemTime};

pub type Rgb8 = Rgb<u8>;
pub type FileSize = u64;
pub type TimeStamp = u64;
pub struct PictureFileData(pub FileSize, pub TimeStamp);
pub type Tags = HashSet<String>;

pub fn timestamp(system_time: SystemTime) -> TimeStamp {
    let duration = system_time.duration_since(UNIX_EPOCH).expect("time went backwards");
    duration.as_secs() * 1_000_000 + (duration.subsec_nanos() / 1_000) as u64
}

pub fn datetime_from_time_stamp(timestamp: u64) -> DateTime<Local> {
    let secs = timestamp / 1_000_000;
    let nanos = (timestamp % 1_000_000) * 1_000;
    let system_time = UNIX_EPOCH + Duration::new(secs, nanos as u32);
    DateTime::<Local>::from(system_time)
}


#[derive(Debug, Clone)]
pub struct ImageData {
    pub label: Label,
    pub size: FileSize,
    pub modified_time: TimeStamp,
    pub rank: Rank,
    pub palette: Palette,
    pub cover: bool,
    pub tags: Tags,
}

impl ImageData {
    pub fn new(label: &str) -> Self {
        ImageData {
            label: from(label),
            size: 0,
            rank: Rank::NoStar,
            modified_time: timestamp(SystemTime::UNIX_EPOCH),
            palette: Palette::new(vec![], 0),
            cover: false,
            tags: HashSet::new(),
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        get_data_from_picture_file(file_path).and_then(|file_data| {
            Ok(ImageData {
                label: String::from(""),
                size: file_data.0,
                modified_time: file_data.1,
                rank: Rank::NoStar,
                palette: Palette::new(vec![], 0),
                cover: false,
                tags: HashSet::new(),
            })
        })
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn modified_time(&self) -> TimeStamp {
        self.modified_time
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn palette(&self) -> Palette {
        self.palette.clone()
    }

    pub fn rank(&self) -> Rank {
        self.rank
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
