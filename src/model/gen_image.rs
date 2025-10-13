extern crate image;
use image::DynamicImage;

#[allow(dead_code)]
pub const SINGLE_DOT: &str = "testdata/single_dot.png";
pub const NINE_COLORS: &str = "testdata/nine_colors.png";
pub const WHITE_SQUARE: &str = "testdata/white_square.png";
pub const LARGE_PICTURE: &str = "testdata/large_picture.png";

use gtk::gdk;
use gtk::glib;
use image::{Rgb, RgbImage};

pub fn no_thumbnail_picture() -> gtk::Picture {
    let width = 256;
    let height = 256;
    let stride = width * 4;
    let mut pixels = vec![0u8; stride * height];

    for y in 0..height {
        for x in 0..width {
            if x >= 32 && x < (width - 32) && x == y || x == (width - 1 - y) {
                let offset = y * stride + x * 4;
                pixels[offset] = 127;
                pixels[offset + 1] = 127;
                pixels[offset + 2] = 127;
                pixels[offset + 3] = 255;
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

pub fn thumbnail_size_display(size: ThumbnailSize) -> String {
    match size {
        ThumbnailSize::Icon => String::from("Icon"),
        ThumbnailSize::Small => String::from("Small"),
        ThumbnailSize::Medium => String::from("Medium"),
        ThumbnailSize::Large => String::from("Large"),
        ThumbnailSize::Larger => String::from("Larger"),
        ThumbnailSize::Custom((w, h)) => format!("Custom({},{})", w, h),
    }
}

pub fn thumbnail_size_for(pictures_per_row: usize) -> ThumbnailSize {
    match pictures_per_row {
        10 => ThumbnailSize::Small,
        9 => ThumbnailSize::Small,
        8 => ThumbnailSize::Small,
        7 => ThumbnailSize::Medium,
        6 => ThumbnailSize::Medium,
        5 => ThumbnailSize::Medium,
        4 => ThumbnailSize::Large,
        3 => ThumbnailSize::Large,
        2 => ThumbnailSize::Larger,
        _ => ThumbnailSize::Small,
    }
}
fn write_thumbnail<R: std::io::Seek + std::io::Read>(
    reader: BufReader<R>,
    extension: &str,
    mut output_file: File,
    pictures_per_row: usize,
) -> ThumbResult<()> {
    let mime = match extension {
        "jpg" | "jpeg" | "JPG" | "JPEG" => mime::IMAGE_JPEG,
        "png" | "PNG" => mime::IMAGE_PNG,
        _ => panic!("wrong extension"),
    };
    let size: ThumbnailSize = thumbnail_size_for(pictures_per_row);
    let mut thumbnails = match create_thumbnails(reader, mime, [size]) {
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
pub fn create_thumbnail_file(
    thumbnail_file_path: &str,
    picture_file_path: &str,
    pictures_per_row: usize,
) -> IOResult<()> {
    match File::open(picture_file_path) {
        Err(err) => Err(err),
        Ok(picture_file) => {
            let path = Path::new(&picture_file_path);
            let extension = match path.extension().and_then(OsStr::to_str) {
                None => return Err(std::io::Error::other("picture file has no extension")),
                Some(ext) => ext,
            };
            let reader = BufReader::new(picture_file);

            let output_file = File::create(thumbnail_file_path)?;
            match write_thumbnail(reader, extension, output_file, pictures_per_row) {
                Err(err) => Err(std::io::Error::other(err)),
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

pub fn large_picture() -> DynamicImage {
    let mut image = RgbImage::new(2250, 2250);
    for cx in 0..2250 {
        for cy in 0..2250 {
            let color = Rgb([
                (cx % 256) as u8,
                (255 - (cy % 256)) as u8,
                ((cx * 7 + cy * 13) % 256) as u8,
            ]);
            image.put_pixel(cx, cy, color)
        }
    }
    DynamicImage::ImageRgb8(image)
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
#[allow(dead_code)]
pub fn save_large_picture() {
    let image = large_picture();
    image.save(LARGE_PICTURE).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    // uncomment test to save test pictures
    // #[test]
    fn gen_pictures() {
        save_nine_colors()
    }


    #[test]
    fn check_thumbnail_size_display() {
        assert_eq!("Icon", &thumbnail_size_display(ThumbnailSize::Icon));
        assert_eq!("Small", &thumbnail_size_display(ThumbnailSize::Small));
        assert_eq!("Medium", &thumbnail_size_display(ThumbnailSize::Medium));
        assert_eq!("Large", &thumbnail_size_display(ThumbnailSize::Large));
        assert_eq!("Larger", &thumbnail_size_display(ThumbnailSize::Larger));
        assert_eq!(
            "Custom(23,17)",
            &thumbnail_size_display(ThumbnailSize::Custom((23, 17)))
        );
    }

    #[test]
    fn thumbnail_size_for_different_number_of_pictures_per_row() {
        assert_eq!("Small", &thumbnail_size_display(thumbnail_size_for(10)));
        assert_eq!("Small", &thumbnail_size_display(thumbnail_size_for(9)));
        assert_eq!("Small", &thumbnail_size_display(thumbnail_size_for(8)));
        assert_eq!("Medium", &thumbnail_size_display(thumbnail_size_for(7)));
        assert_eq!("Medium", &thumbnail_size_display(thumbnail_size_for(6)));
        assert_eq!("Medium", &thumbnail_size_display(thumbnail_size_for(5)));
        assert_eq!("Large", &thumbnail_size_display(thumbnail_size_for(4)));
        assert_eq!("Large", &thumbnail_size_display(thumbnail_size_for(3)));
        assert_eq!("Larger", &thumbnail_size_display(thumbnail_size_for(2)));
    }
}
