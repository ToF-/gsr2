extern crate image;
use image::DynamicImage;

#[allow(dead_code)]
pub const SINGLE_DOT: &str = "testdata/single_dot.png";
pub const NINE_COLORS: &str = "testdata/nine_colors.png";
pub const WHITE_SQUARE: &str = "testdata/white_square.png";

use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use image::{Rgb, RgbImage};

pub fn no_thumbnail_picture() -> gtk::Picture {
    let width = 256;
    let height = 256;
    let stride = width * 4;
    let mut pixels = vec![0u8; stride * height];

    for y in 0..height {
        for x in 0..width {
            if x >= 32 && x < (width - 32) {
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

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::io::Result as IOResult;
use std::path::Path;
use thumbnailer::ThumbnailSize;
use thumbnailer::create_thumbnails;
use thumbnailer::error::ThumbResult;

fn write_thumbnail<R: std::io::Seek + std::io::Read>(
    reader: BufReader<R>,
    extension: &str,
    mut output_file: File,
) -> ThumbResult<()> {
    let mime = match extension {
        "jpg" | "jpeg" | "JPG" | "JPEG" => mime::IMAGE_JPEG,
        "png" | "PNG" => mime::IMAGE_PNG,
        _ => panic!("wrong extension"),
    };
    let mut thumbnails = match create_thumbnails(reader, mime, [ThumbnailSize::Small]) {
        Ok(tns) => tns,
        Err(err) => {
            eprintln!("error while creating thumbnails:{:?}", err);
            return Err(err);
        }
    };
    let thumbnail = thumbnails.pop().unwrap();
    let write_result = match extension {
        "jpg" | "jpeg" | "JPG" | "JPEG" => thumbnail.write_jpeg(&mut output_file, 255),
        "png" | "PNG" => thumbnail.write_png(&mut output_file),
        _ => panic!("wrong extension"),
    };
    match write_result {
        Err(err) => {
            eprintln!("error while writing thunbnail:{}", err);
            Err(err)
        }
        ok => ok,
    }
}
pub fn create_thumbnail_file(thumbnail_file_path: &str, picture_file_path: &str) -> IOResult<()> {
    match File::open(picture_file_path) {
        Err(err) => Err(err),
        Ok(picture_file) => {
            let path = Path::new(&picture_file_path);
            let extension = match path.extension().and_then(OsStr::to_str) {
                None => return Err(std::io::Error::other("picture file has no extension")),
                Some(ext) => ext,
            };
            let reader = BufReader::new(picture_file);
            let output_file = match File::create(thumbnail_file_path) {
                Err(err) => return Err(std::io::Error::other("cannot create output file")),
                Ok(file) => file,
            };
            match write_thumbnail(reader, extension, output_file) {
                Err(err) => return Err(std::io::Error::other(err)),
                Ok(_) => Ok(()),
            }
        }
    }
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
