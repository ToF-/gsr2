extern crate image;
use image::DynamicImage;
use rand::prelude::*;

#[allow(dead_code)]
pub const SINGLE_DOT: &str = "testdata/single_dot.png";
pub const NINE_COLORS: &str = "testdata/nine_colors.png";

use image::{Rgb, RgbImage};

#[allow(dead_code)]
pub fn gen_single_dot() {
    let mut image = RgbImage::new(10, 10);
    image.put_pixel(5, 5, Rgb([255, 255, 255]));
    image.save(SINGLE_DOT).unwrap();
}

#[allow(dead_code)]
pub fn gen_nine_colors() -> DynamicImage {
    let mut image = RgbImage::new(900, 900);
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (1..100).collect();

    for cx in 0..90 {
        for cy in 0..90 {
            let color = match ((cx * cy) % 10) {
                0 => Rgb([0, 0, 0]),
                1 => Rgb([238, 130, 238]),
                2 => Rgb([154, 205, 50]),
                3 => Rgb([0, 0, 255]),
                4 => Rgb([255, 0, 0]),
                5 => Rgb([255, 140, 0]),
                6 => Rgb([0, 128, 128]),
                7 => Rgb([139, 69, 19]),
                8 => Rgb([255, 255, 0]),
                _ => Rgb([128, 128, 128]),
            };
            for x in 0..10 {
                for y in 0..10 {
                    image.put_pixel(cx * 10 + x, cy * 10 + y, color)
                }
            }
        }
    }
    DynamicImage::ImageRgb8(image)
}
#[allow(dead_code)]
pub fn save_nine_colors() {
    let image = gen_nine_colors();
    image.save(NINE_COLORS).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_some_test_images() {
        gen_single_dot();
        save_nine_colors();
        assert!(true);
    }
}
