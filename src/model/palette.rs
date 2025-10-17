use crate::env::default_values::MAX_PALETTE_COLORS;
use image::{DynamicImage, GenericImageView};
use palette_extract::{get_palette_with_options, Quality, MaxColors, PixelEncoding, PixelFilter};
use palette_extract::get_palette_rgb;
use palette_extract::Color;
use image::Rgba;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Palette {
    sample: Vec<Color>,
    count: usize,
}

impl Palette {

    pub fn new(sample: Vec<Color>, count: usize) -> Self {
        Palette{
            sample,
            count,
        }
    }

    pub fn from(image: &DynamicImage) -> Self {
        let sample = get_palette(image);
        let count = color_count(image);
        Palette { sample, count, }
    }

    pub fn sample(&self) -> Vec<Color> {
        self.sample.clone()
    }

    pub fn count(&self) -> usize {
        self.count
    }

}

pub fn get_palette(image: &DynamicImage) -> Vec<Color> {
    let pixels: &[u8] = image.as_bytes();
    let mut sample = get_palette_rgb(pixels);
    let mut sample = get_palette_with_options(&pixels,
        PixelEncoding::Rgb,
        Quality::new(5),
        MaxColors::new(MAX_PALETTE_COLORS),
        PixelFilter::None);
    sample.sort_by_key(color_to_u32);
    sample
}

fn rgba_key(rgba: Rgba<u8>) -> u32 {
        (rgba[0] as u32) << 24
        | (rgba[1] as u32) << 16
        | (rgba[2] as u32) << 8
        | (rgba[3] as u32)
}



pub fn color_count(image: &DynamicImage) -> usize {
    let mut colors: HashSet<u32> = HashSet::new();
    let pixels: Vec<(u32,u32,Rgba<u8>)> = image.pixels().collect();
    for pixel in pixels {
        let rgba = pixel.2;
        colors.insert(rgba_key(rgba));
    };
    colors.len()
}

fn color_to_u32(color: &Color) -> u32 {
    ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
}

