extern crate image;

#[allow(dead_code)]
pub const SINGLE_DOT: &str = "testdata/single_dot.png";

use image::{Rgb, RgbImage};

#[allow(dead_code)]
pub fn gen_single_dot() {
    // a default (black) image containing Rgb values
    let mut image = RgbImage::new(10, 10);

    // set a central pixel to white
    image.put_pixel(5, 5, Rgb([255, 255, 255]));

    // write it out to a file
    image.save(SINGLE_DOT).unwrap();
}

