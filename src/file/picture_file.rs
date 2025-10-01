use crate::env::default_values::THUMB_SUFFIX;
use crate::model::gallery::Gallery;
use crate::model::gen_image::create_thumbnail_file;
use crate::file::paths::check_path_exists;
use crate::file::paths::{check_path, check_path_is_a_jpg_or_png_file, check_picture_file};
use crate::model::image_data::get_palette;
use crate::model::image_data::Palette;
use std::fs;
use crate::model::image_data::PictureFileData;
use std::io::Error;
use std::io::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_all_picture_file_paths(path: &str) -> Result<Vec<String>> {
    check_path(path).map(|directory| {
        let mut file_paths: Vec<String> = Vec::new();
        for _entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.into_path())
            .filter(|path| {
                path.is_file()
                    && check_path_is_a_jpg_or_png_file(path).is_ok()
                    && !path.display().to_string().contains(THUMB_SUFFIX)
            })
        {
            file_paths.push(_entry.display().to_string())
        }
        file_paths
    })
}

pub fn get_picture_file_path(file_path: &str) -> Result<String> {
    check_picture_file(file_path)
}

pub fn create_missing_thumbnails(gallery: &Gallery) {
    for picture in gallery.pictures() {
        let file_path = picture.file_path();
        let thumbnail_file_path = picture.thumbnail_file_path();
        match check_path_exists(&PathBuf::from(&thumbnail_file_path)) {
            Ok(_) => {}
            Err(_) => match create_thumbnail_file(&thumbnail_file_path, &file_path) {
                Ok(_) => println!("creating thumbnail {}", thumbnail_file_path),
                Err(err) => {
                    eprintln!("{}", err);
                }
            },
        }
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

