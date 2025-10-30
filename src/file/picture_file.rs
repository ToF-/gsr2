use crate::env::default_values::THUMB_SUFFIX;
use crate::file::Database;
use crate::file::paths::check_collectable;
use crate::file::paths::thumbnail_name_from;
use crate::file::paths::thumbnail_names_from;
use crate::file::paths::{check_path, check_path_is_a_jpg_or_png_file, check_picture_file};
use crate::file::paths::{check_path_exists, file_exists};
use crate::model::gallery::Gallery;
use crate::model::image_data::ImageData;
use crate::model::image_data::PictureFileData;
use crate::model::image_data::timestamp;
use crate::model::palette::Palette;
use crate::model::picture::Picture;
use crate::model::rank::Rank;
use crate::model::thumbnail::create_thumbnail_file;
use std::collections::HashSet;
use std::fs;
use std::fs::{copy, remove_file};
use std::io::Error;
use std::io::Result as IOResult;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_all_picture_file_paths(path: &str) -> IOResult<Vec<String>> {
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

pub fn get_picture_file_path(file_path: &str) -> IOResult<String> {
    check_picture_file(file_path)
}

pub fn create_missing_thumbnails(gallery: &Gallery, pictures_per_row: usize) {
    let mut count: usize = 0;
    for picture in gallery.pictures() {
        let file_path = picture.file_path();
        if file_exists(&file_path) {
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
                            eprintln!("{} with thumbnail {} for picture {}", err, thumbnail_file_path, file_path);
                        }
                    }
                }
            }
        }
    }
    println!("{} thumbnails created", count)
}

pub fn collect_picture_data(picture: &Picture) -> IOResult<Picture> {
    let image =
        image::open(&picture.file_path()).expect(&format!("can't load {}", picture.file_path()));
    let palette = Palette::from(&image);
    let new_image_data = match picture.image_data() {
        Some(image_data) => ImageData {
            palette,
            ..image_data
        },
        None => {
            if let Ok(file_data) = get_data_from_picture_file(&picture.file_path()) {
                ImageData {
                    label: "".to_string(),
                    size: file_data.0,
                    modified_time: file_data.1,
                    rank: Rank::NoStar,
                    palette,
                    tags: HashSet::new(),
                    cover: None,
                }
            } else {
                return Err(std::io::Error::other(format!(
                    "cannot get file data for {}",
                    picture.file_path()
                )));
            }
        }
    };
    let mut new_picture = Picture::new(&picture.file_path());
    new_picture.set_image_data(new_image_data);
    Ok(new_picture)
}

pub fn collect_data(gallery: &Gallery, database: &Database) -> IOResult<()> {
    let mut count: usize = 0;
    let total: usize = gallery.pictures().len();
    for picture in gallery.pictures() {
        count += 1;
        match database.rusqlite_check_picture_with_file_path(&picture.file_path()) {
            Ok(file_path) => {
                println!("already in db: {}", file_path)
            }
            Err(_) => {
                match collect_picture_data(&picture) {
                    Ok(picture) => match database.insert_picture(&picture) {
                        Ok(_) => {
                            println!("{:?}", picture);
                        }
                        Err(err) => {
                            eprintln!("{}:\n{}", picture.file_path(), err)
                        }
                    },
                    Err(err) => {
                        println!("{}", err)
                    }
                };
            }
        }
        println!("{}/{}:{}", count, total, picture.file_path());
    }
    Ok(())
}

pub fn delete_picture_file(file_path: &str) -> IOResult<()> {
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

pub fn delete_picture_files(file_path: &str) -> IOResult<()> {
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

pub fn get_data_from_picture_file(file_path: &str) -> IOResult<PictureFileData> {
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

fn move_file(source: &str, target: &str) {
    if file_exists(source) {
        copy(PathBuf::from(source), PathBuf::from(target));
        remove_file(PathBuf::from(source));
    }
}

#[allow(dead_code)]
pub fn move_picture_files(file_path: &str, target_dir: &str) -> IOResult<u64> {
    check_path_exists(&PathBuf::from(file_path)).and_then(|source_file_path| {
        check_path_exists(&PathBuf::from(target_dir)).and_then(|existing_path| {
            check_collectable(existing_path).and_then(|target_path| {
                if let Some(file_name) = source_file_path.file_name() {
                    let target_file_path = target_path.join(file_name);
                    if *source_file_path != target_file_path {
                        let source_file_path_name: String = source_file_path.display().to_string();
                        let target_file_path_name: String = target_file_path.display().to_string();

                        move_file(&source_file_path_name, &target_file_path_name);
                        move_file(
                            &thumbnail_name_from(&source_file_path_name, 10),
                            &thumbnail_name_from(&target_file_path_name, 10),
                        );
                        move_file(
                            &thumbnail_name_from(&source_file_path_name, 7),
                            &thumbnail_name_from(&target_file_path_name, 7),
                        );
                        move_file(
                            &thumbnail_name_from(&source_file_path_name, 4),
                            &thumbnail_name_from(&target_file_path_name, 4),
                        );
                        move_file(
                            &thumbnail_name_from(&source_file_path_name, 2),
                            &thumbnail_name_from(&target_file_path_name, 2),
                        );
                        Ok(1)
                    } else {
                        Err(Error::other(format!(
                            "same source and target: {}",
                            source_file_path.display()
                        )))
                    }
                } else {
                    Err(Error::other(format!(
                        "can't file file name in {}",
                        file_path
                    )))
                }
            })
        })
    })
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod test {

    use super::*;
    use crate::file::paths::current_directory;
    use crate::file::paths::thumbnail_name_from;
    use crate::test_data::*;
    use std::fs::File;
    use std::io::prelude::*;
    use serial_test::serial;
    use std::io::Error as IOError;

    fn create_dummy_file(file_path: &str) {
        let mut file = File::create(file_path).expect("can't create test file");
        file.write_all(b"Hello, world!")
            .expect("can't write to file");
    }

    fn create_dummy_files() {
        create_dummy_file("testdata/dummy_pic.png");
        create_dummy_file("testdata/dummy_picTHUMBLarge.png");
        create_dummy_file("testdata/dummy_picTHUMBLarger.png");
        create_dummy_file("testdata/dummy_picTHUMBMedium.png");
        create_dummy_file("testdata/dummy_picTHUMBSmall.png");
    }

    fn my_file_path() -> String {
        format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, NINE_COLORS)
    }

    fn my_test_dir() -> String {
        format!("{}/{}", current_directory(), TEST_DATA_DIR)
    }

    fn my_target_dir() -> String {
        format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR)
    }

    pub fn get_palette_from_picture_file(file_path: &str) -> IOResult<Palette> {
        match image::open(file_path) {
            Ok(image) => {
                let palette = Palette::from(&image);
                Ok(palette)
            }
            Err(_) => Err(IOError::other(format!(
                "can't open image file {} for palette extraction",
                file_path
            ))),
        }
    }

    #[test]
    #[serial]
    fn deleting_picture_files() {
        create_dummy_files();
        delete_picture_files("testdata/dummy_pic.png");
        assert!(!file_exists("testdata/dummy_pic.png"));
        assert!(!file_exists("testdata/dummy_picTHUMBLarge.png"));
        assert!(!file_exists("testdata/dummy_picTHUMBLarger.png"));
        assert!(!file_exists("testdata/dummy_picTHUMBMedium.png"));
        assert!(!file_exists("testdata/dummy_picTHUMBSmall.png"));
    }

    #[test]
    #[serial]
    fn moving_picture_files_to_a_directory_error_wrong_target() {
        let file_path = my_file_path();
        let target_dir = String::from("./test");
        let result = move_picture_files(&file_path, &target_dir);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn moving_picture_files_to_a_directory_error_wrong_source() {
        let file_path = String::from("a_file.jpg");
        let target_dir = my_target_dir();
        let result = move_picture_files(&file_path, &target_dir);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn moving_picture_files_to_a_directory_error_not_absolute_target() {
        let file_path = my_file_path();
        let target_dir = String::from("./testdata/subdir");
        let result = move_picture_files(&file_path, &target_dir);
        assert!(result.is_err());
    }
    #[test]
    #[serial]
    fn moving_picture_files_to_a_directory_error_same_source_and_target() {
        let file_path = my_file_path();
        let target_dir = test_directory();
        let result = move_picture_files(&file_path, &target_dir);
        assert!(result.is_err());
    }
    #[test]
    #[serial]
    fn moving_picture_files_to_a_directory() {
        let file_path = my_file_path();
        let target_dir = my_target_dir();
        let test_dir = my_test_dir();
        let target_file_path = format!("{}/{}", target_dir, NINE_COLORS);
        let result = move_picture_files(&file_path, &target_dir);
        assert!(result.is_ok());
        assert!(file_exists(&target_file_path));
        assert!(!file_exists(&file_path));
        assert!(file_exists(&thumbnail_name_from(&target_file_path, 10)));
        assert!(file_exists(&thumbnail_name_from(&target_file_path, 7)));
        assert!(file_exists(&thumbnail_name_from(&target_file_path, 4)));
        assert!(file_exists(&thumbnail_name_from(&target_file_path, 2)));

        move_picture_files(&target_file_path, &test_dir);
    }
}
