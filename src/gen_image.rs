extern crate image;
use image::DynamicImage;

#[allow(dead_code)]
pub const SINGLE_DOT: &str = "testdata/single_dot.png";
pub const NINE_COLORS: &str = "testdata/nine_colors.png";
pub const WHITE_SQUARE: &str = "testdata/white_square.png";

use image::{Rgb, RgbImage};
use gtk::prelude::*;
use gtk::{gdk, Picture};
use gtk::glib;

pub fn no_thumbnail_picture() -> gtk::Picture {
       let width = 256;
    let height = 256;
    let stride = width * 4;
    let mut pixels = vec![0u8; stride * height];

    for y in 0..height {
        for x in 0..width {
            if x >= 32 && x < (width-32) {
                if x == y || x == (width - 1 - y) {
                    let offset = y * stride + x * 4;
                    pixels[offset] = 127;  
                    pixels[offset + 1] = 127;
                    pixels[offset + 2] = 127;
                    pixels[offset + 3] = 255;
                }
            }
        }
    }

    let bytes = glib::Bytes::from_owned(pixels);
    let texture = gdk::MemoryTexture::new(
        width as i32,
        height as i32,
        gdk::MemoryFormat::R8g8b8a8,
        &bytes,
        stride,
    );
    gtk::Picture::for_paintable(&texture)
}

#[allow(dead_code)]
pub fn gen_single_dot() {
    let mut image = RgbImage::new(10, 10);
    image.put_pixel(5, 5, Rgb([255, 255, 255]));
    image.save(SINGLE_DOT).unwrap();
}

#[allow(dead_code)]
pub fn gen_nine_colors() -> DynamicImage {
    let mut image = RgbImage::new(900, 900);

    for cx in 0..90 {
        for cy in 0..90 {
            let color = match (cx * cy) % 10 {
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
pub fn gen_white_square() {
    let range = 100..900;
    let mut image = RgbImage::new(1000, 1000);

    for x in 0..900 {
        for y in 0..900 {
            let color = if range.contains(&x) && range.contains(&y) {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            };
            image.put_pixel(x, y, color)
        }
    }
    image.save(WHITE_SQUARE).unwrap();
}
#[allow(dead_code)]
pub fn save_nine_colors() {
    let image = gen_nine_colors();
    image.save(NINE_COLORS).unwrap();
}
