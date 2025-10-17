use crate::file::paths::thumbnail_names_from;
use std::path::Path;
use std::fs::remove_file;
use crate::env::default_values::THUMB_SUFFIX;
use crate::file::paths::check_path_exists;
use crate::file::paths::{check_path, check_path_is_a_jpg_or_png_file, check_picture_file};
use crate::model::palette::{Palette};
use crate::model::gallery::Gallery;
use crate::model::gen_image::create_thumbnail_file;
use crate::model::image_data::PictureFileData;
use std::fs;
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

pub fn create_missing_thumbnails(gallery: &Gallery, pictures_per_row: usize) {
    let mut count: usize = 0;
    for picture in gallery.pictures() {
        let file_path = picture.file_path();
        let thumbnail_file_path = picture.thumbnail_file_path_for_size(pictures_per_row);
        match check_path_exists(&PathBuf::from(&thumbnail_file_path)) {
            Ok(_) => {}
            Err(_) => {
                match create_thumbnail_file(&thumbnail_file_path, &file_path, pictures_per_row) {
                    Ok(_) => {
                        println!("creating thumbnail {}", thumbnail_file_path);
                        count += 1
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                }
            }
        }
    }
    println!("{} thumbnails created", count)
}

pub fn collect_data(gallery: &Gallery) -> Result<()> {
    for picture in gallery.pictures() {
    };
    Ok(())
}
pub fn delete_picture_file(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    if path.exists() {
        let _ = remove_file(path);
        Ok(())
    } else {
        Err(std::io::Error::other(format!("cannot delete file {}", file_path)))
    }
}

pub fn delete_picture_files(file_path: &str) -> Result<()> {
    delete_picture_file(file_path)
        .and_then(|_| {
            let thumbnails = thumbnail_names_from(file_path);
            for thumbnail_file_path in thumbnails {
                if let Err(err) = delete_picture_file(&thumbnail_file_path) {
                    return Err(err)
                }
            }
            Ok(())
        })
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_palette_from_picture_file(file_path: &str) -> Result<Palette> {
    match image::open(file_path) {
        Ok(image) => {
            let palette = Palette::from(&image);
            Ok(palette)
        }
        Err(_) => Err(Error::other(format!(
            "can't open image file {} for palette extraction",
            file_path
        ))),
    }
}
