extern crate image;
use crate::model::palette::Palette;
use gtk::gdk;
use gtk::glib;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::io::Result as IOResult;
use std::path::Path;
use thumbnailer::ThumbnailSize;
use thumbnailer::create_thumbnails;
use thumbnailer::error::ThumbResult;


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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::{NINE_COLORS,SINGLE_DOT};
    use palette_extract::Color;
    use crate::env::default_values::MAX_PALETTE_COLORS;

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
