use crate::model::image_data::timestamp;
use crate::model::picture::Picture;
use std::collections::HashSet;
use crate::model::image_data::ImageData;
use crate::file::Database;
use crate::env::default_values::THUMB_SUFFIX;
use crate::file::paths::check_path_exists;
use crate::file::paths::thumbnail_names_from;
use crate::file::paths::{check_path, check_path_is_a_jpg_or_png_file, check_picture_file};
use crate::model::gallery::Gallery;
use crate::model::image_data::PictureFileData;
use crate::model::palette::Palette;
use crate::model::thumbnail::create_thumbnail_file;
use std::fs;
use std::fs::remove_file;
use std::io::Error;
use std::io::Result;
use std::path::Path;
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

pub fn collect_picture_data(picture: &Picture) -> Result<Picture> {
    let image = image::open(&picture.file_path()).expect(&format!("can't load {}", picture.file_path()));
    let palette = Palette::from(&image);
    let new_image_data =  match picture.image_data() {
        Some(image_data) => ImageData {
            palette: palette,
            .. image_data
        },
        None =>
            if let Ok(file_data) = get_data_from_picture_file(&picture.file_path()) {
                ImageData {
                    label: "".to_string(),
                    size: file_data.0,
                    modified_time: file_data.1,
                    palette: palette,
                    tags: HashSet::new(),
                    cover: false,
                }
            } else {
                return Err(std::io::Error::other(format!("cannot get file data for {}", picture.file_path())))
            },
    };
    let mut new_picture = Picture::new(&picture.file_path());
    new_picture.set_image_data(new_image_data);
    Ok(new_picture)
}

pub fn collect_data(gallery: &Gallery, database: &Database) -> Result<()> {
    let mut count: usize = 0;
    let total: usize = gallery.pictures().len();
    for picture in gallery.pictures() {
        count += 1;
        if database.rusqlite_retrieve_picture_with_file_path(&picture.file_path()).is_err() {
            match collect_picture_data(&picture) {
                Ok(picture) => {
                    println!("{:?}", picture);
                },
                Err(err) => {
                    println!("{}", err)
                },
            };
            println!("{}/{}:{}", count, total, picture.file_path());
        }
    };
    Ok(())
}

pub fn delete_picture_file(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    if path.exists() {
        let _ = remove_file(path);
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "cannot delete file {}",
            file_path
        )))
    }
}

pub fn delete_picture_files(file_path: &str) -> Result<()> {
    delete_picture_file(file_path).and_then(|_| {
        let thumbnails = thumbnail_names_from(file_path);
        for thumbnail_file_path in thumbnails {
            if let Err(err) = delete_picture_file(&thumbnail_file_path) {
                return Err(err);
            }
        }
        Ok(())
    })
}

pub fn get_data_from_picture_file(file_path: &str) -> Result<PictureFileData> {
    let path = PathBuf::from(file_path);
    match fs::metadata(path.clone()) {
        Ok(metadata) => {
            let file_size = metadata.len();
            let modified_time = timestamp(metadata.modified().unwrap());
            Ok(PictureFileData(file_size, modified_time))
        }
        Err(err) => Err(err),
    }
}

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
