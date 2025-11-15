use crate::env::default_values::MAX_PALETTE_COLORS;
use image::Rgba;
use image::{DynamicImage, GenericImageView};
use palette_extract::Color;
use palette_extract::{MaxColors, PixelEncoding, PixelFilter, Quality, get_palette_with_options};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Palette {
    sample: Vec<Color>,
    count: usize,
}

impl Palette {
    pub fn new(sample: Vec<Color>, count: usize) -> Self {
        Palette { sample, count }
    }

    pub fn from(image: &DynamicImage) -> Self {
        let sample = get_palette(image);
        let count = color_count(image);
        Palette { sample, count }
    }

    pub fn sample(&self) -> Vec<Color> {
        self.sample.clone()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn sample_as_array(&self) -> [u8; 31] {
        let mut result: [u8; 31] = [0; 31];
        result[0] = self.sample.len() as u8;
        for i in 0..self.sample.len() {
            let color = self.sample[i];
            result[i * 3 + 1] = color.r;
            result[i * 3 + 2] = color.g;
            result[i * 3 + 3] = color.b;
        }
        result
    }

    pub fn set_sample_from_array(&mut self, array: [u8; 31]) {
        let len: usize = array[0].into();
        self.sample = vec![];
        for i in 0..len {
            let color = Color {
                r: array[i * 3 + 1],
                g: array[i * 3 + 2],
                b: array[i * 3 + 3],
            };
            self.sample.push(color);
        }
    }
}

pub fn get_palette(image: &DynamicImage) -> Vec<Color> {
    let pixels: &[u8] = image.as_bytes();
    let mut sample = get_palette_with_options(
        pixels,
        PixelEncoding::Rgb,
        Quality::new(5),
        MaxColors::new(MAX_PALETTE_COLORS),
        PixelFilter::None,
    );
    sample.sort_by_key(color_to_u32);
    sample
}

fn rgba_key(rgba: Rgba<u8>) -> u32 {
    (rgba[0] as u32) << 24 | (rgba[1] as u32) << 16 | (rgba[2] as u32) << 8 | (rgba[3] as u32)
}

pub fn color_count(image: &DynamicImage) -> usize {
    let mut colors: HashSet<u32> = HashSet::new();
    let pixels: Vec<(u32, u32, Rgba<u8>)> = image.pixels().collect();
    for pixel in pixels {
        let rgba = pixel.2;
        colors.insert(rgba_key(rgba));
    }
    colors.len()
}

fn color_to_u32(color: &Color) -> u32 {
    ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::default_values::MAX_PALETTE_COLORS;
    use crate::test_data::*;
    use palette_extract::Color;

    #[test]
    fn counting_the_numbers_of_distinct_colors_in_an_image() {
        let image = image::open(single_dot_file_path())
            .expect(&format!("can't load {}", single_dot_file_path()));
        let palette = Palette::from(&image);
        assert_eq!(2, palette.count());
        let image = image::open(nine_colors_file_path())
            .expect(&format!("can't load {}", single_dot_file_path()));
        let palette = Palette::from(&image);
        assert_eq!(10, palette.count())
    }

    #[test]
    fn extracting_a_palette_from_an_image() {
        let image = image::open(single_dot_file_path())
            .expect(&format!("can't load {}", single_dot_file_path()));
        let palette = Palette::from(&image);
        assert_eq!(8, palette.sample().len());
    }
    #[test]
    fn converting_palette_sample_to_blob() {
        let image = image::open(single_dot_file_path())
            .expect(&format!("can't load {}", single_dot_file_path()));
        let palette = Palette::from(&image);
        let expected: Vec<u8> = vec![
            8, 4, 4, 4, 8, 4, 4, 8, 4, 4, 8, 4, 4, 64, 4, 132, 64, 132, 128, 68, 4, 4, 252, 252,
            252, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(expected, palette.sample_as_array());
    }
    #[test]
    fn converting_blob_to_palette_sample() {
        let image = image::open(single_dot_file_path())
            .expect(&format!("can't load {}", single_dot_file_path()));
        let palette = Palette::from(&image);
        let mut other = Palette::from(&image);
        other.set_sample_from_array([
            8, 4, 4, 4, 8, 4, 4, 8, 4, 4, 8, 4, 4, 64, 4, 132, 64, 132, 128, 68, 4, 4, 252, 252,
            252, 0, 0, 0, 0, 0, 0,
        ]);
        assert_eq!(palette.sample(), other.sample());
    }
}
