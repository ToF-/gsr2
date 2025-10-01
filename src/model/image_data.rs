use std::collections::HashSet;
use crate::env::default_values::MAX_PALETTE_COLORS;
use image::{DynamicImage, Rgb};
use palette_extract::{MaxColors, PixelEncoding, PixelFilter, Quality, get_palette_with_options};
use std::cmp::Ordering;
use std::fs;
use std::io::{Error, Result};
use std::path::PathBuf;
use std::time::SystemTime;

pub type Rgb8 = Rgb<u8>;
pub type Palette = [Rgb8; 9];
pub type FileSize = u64;
pub struct PictureFileData(FileSize, SystemTime);
pub type Tags = HashSet<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    label: String,
    size: FileSize,
    modified_time: SystemTime,
    palette: Palette,
    tags: Tags,
}

impl ImageData {
    pub fn new(label: &str) -> Self {
        ImageData {
            label: label.to_string(),
            size: 0,
            modified_time: SystemTime::now(),
            palette: [Rgb::from([0,0,0]); 9],
            tags: HashSet::new(),
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        get_data_from_picture_file(file_path)
            .and_then(|file_data| {
                get_palette_from_picture_file(file_path)
                    .and_then(|palette| {
                        Ok(ImageData {
                            label: String::from(""),
                            size: file_data.0,
                            modified_time: file_data.1,
                            palette,
                            tags: HashSet::new(),
                        })
                    })
            })
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }
}

impl Ord for ImageData {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering { todo!() }
}

impl PartialOrd for ImageData {
    fn partial_cmp(&self, _: &ImageData) -> std::option::Option<std::cmp::Ordering> { todo!() }
}

fn compare_rgb(color: &Rgb8, other: &Rgb8) -> Ordering {
    match color[0].cmp(&other[0]) {
        Ordering::Equal => match color[1].cmp(&other[1]) {
            Ordering::Equal => color[2].cmp(&other[2]),
            res => res,
        },
        res => res,
    }
}

fn get_palette(image: &DynamicImage) -> Palette {
    let mut palette: Palette = [Rgb([0, 0, 0]); 9];
    let pixels: &[u8] = image.as_bytes();
    let colors = get_palette_with_options(
        pixels,
        PixelEncoding::Rgb,
        Quality::new(5),
        MaxColors::new(MAX_PALETTE_COLORS + 1),
        PixelFilter::None,
    );
    colors.iter().enumerate().for_each(|(i, c)| {
        palette[i] = Rgb([c.r, c.g, c.b]);
    });
    palette.sort_by(compare_rgb);
    palette
}

pub fn get_palette_from_picture_file(file_path: &str) -> Result<Palette> {
    match image::open(file_path) {
        Ok(image) => {
            let palette = get_palette(&image);
            Ok(palette)
        }
        Err(_) => Err(Error::other(format!(
            "can't open image file {} for palette extraction",
            file_path
        ))),
    }
}

pub fn get_data_from_picture_file(file_path: &str) -> Result<PictureFileData> {
    let path = PathBuf::from(file_path);
    match fs::metadata(path.clone()) {
        Ok(metadata) => {
            let file_size = metadata.len();
            let modified_time = metadata.modified().unwrap();
            Ok(PictureFileData(file_size, modified_time))
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::gen_image::{NINE_COLORS, gen_nine_colors};

    #[test]
    fn extract_a_palette_of_9_most_used_colors() {
        let image: DynamicImage = gen_nine_colors();
        let palette = get_palette(&image);
        assert_eq!(Rgb([4, 4, 4]), palette[0]);
        assert_eq!(Rgb([4, 4, 252]), palette[1]);
        assert_eq!(Rgb([4, 132, 132]), palette[2]);
        assert_eq!(Rgb([136, 100, 76]), palette[3]);
        assert_eq!(Rgb([156, 204, 52]), palette[4]);
        assert_eq!(Rgb([236, 132, 236]), palette[5]);
        assert_eq!(Rgb([252, 4, 4]), palette[6]);
        assert_eq!(Rgb([252, 140, 4]), palette[7]);
        assert_eq!(Rgb([252, 252, 4]), palette[8]);
    }

    #[test]
    fn extract_a_palette_from_a_picture_file() {
        let palette = get_palette_from_picture_file(NINE_COLORS).unwrap();
        assert_eq!(Rgb([4, 4, 4]), palette[0]);
        assert_eq!(Rgb([4, 4, 252]), palette[1]);
        assert_eq!(Rgb([4, 132, 132]), palette[2]);
        assert_eq!(Rgb([136, 100, 76]), palette[3]);
        assert_eq!(Rgb([156, 204, 52]), palette[4]);
        assert_eq!(Rgb([236, 132, 236]), palette[5]);
        assert_eq!(Rgb([252, 4, 4]), palette[6]);
        assert_eq!(Rgb([252, 140, 4]), palette[7]);
        assert_eq!(Rgb([252, 252, 4]), palette[8]);
    }

    #[test]
    fn extract_size_from_a_picture_file() {
        let file_data = get_data_from_picture_file(NINE_COLORS).unwrap();
        assert_eq!(49746, file_data.0);
    }
}
