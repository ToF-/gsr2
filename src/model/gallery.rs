use crate::file::database::Database;
use crate::file::picture_file::{get_all_picture_file_paths, get_picture_file_path};
use crate::model::order::Order;
use crate::model::picture::Picture;
use rand::prelude::SliceRandom;
use rand::rng;
use std::io::{Error, Result};

#[derive(Debug, Clone)]
pub struct Gallery {
    pictures: Vec<Picture>,
}

impl Gallery {
    pub fn new() -> Self {
        Gallery {
            pictures: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_pictures(pictures: Vec<Picture>) -> Self {
        Gallery { pictures }
    }
    pub fn len(&self) -> usize {
        self.pictures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pictures.is_empty()
    }

    pub fn pictures(&self) -> &Vec<Picture> {
        &self.pictures
    }

    pub fn picture(&self, index: usize) -> Picture {
        self.pictures[index].clone()
    }

    pub fn load_from_directory(&mut self, path: &str) -> Result<usize> {
        println!("loading directory…");
        match get_all_picture_file_paths(path) {
            Ok(list) => {
                for file_path in list {
                    self.pictures.push(Picture::new(&file_path))
                }
                Ok(self.pictures.len())
            }
            Err(err) => Err(err),
        }
    }

    pub fn load_from_database(&mut self, database: &Database) -> Result<usize> {
        println!("loading from database…");
        match database.retrieve_all_pictures() {
            Ok(pictures) => {
                self.pictures = pictures;
                Ok(self.len())
            }
            Err(_) => Err(Error::other("can't retrieve pictures from database")),
        }
    }

    pub fn load_from_file_path(&mut self, file_path: &str) -> Result<usize> {
        match get_picture_file_path(file_path) {
            Ok(path) => {
                self.pictures.push(Picture::new(&path));
                Ok(self.pictures.len())
            }
            Err(err) => Err(err),
        }
    }

    pub fn sort_by(&mut self, order: Order) {
        match order {
            Order::Name => self.pictures.sort_by_key(|picture| picture.file_path()),
            Order::Random => self.pictures.shuffle(&mut rng()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::env::default_values::TEST_DATABASE_FILE;
    use crate::file::database::Database;
    use crate::file::database::tests::my_db;
    use crate::file::database::tests::{
        delete_nine_colors_from_db, insert_nine_colors_sample_into_db,
    };
    use crate::model::gen_image::{NINE_COLORS, SINGLE_DOT, WHITE_SQUARE};

    #[test]
    fn loading_from_a_directory_collect_all_the_picture_files_from_that_directory() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory("./testdata/")
            .expect("can't load from directory");
        gallery.sort_by(Order::Name);
        assert_eq!(4, gallery.len());
        assert_eq!(
            String::from("./testdata/large_picture.png"),
            gallery.picture(0).file_path()
        );
        assert_eq!(
            String::from("./testdata/nine_colors.png"),
            gallery.picture(1).file_path()
        );
        assert_eq!(
            String::from("./testdata/single_dot.png"),
            gallery.picture(2).file_path()
        );
        assert_eq!(
            String::from("./testdata/white_square.png"),
            gallery.picture(3).file_path()
        );
    }

    #[test]
    fn loading_from_a_single_file_path_collect_that_single_picture_file() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_file_path(NINE_COLORS)
            .expect("can't load the file");
        assert_eq!(1, gallery.len());
    }

    #[test]
    fn loading_from_database_collect_all_the_picture_file_paths_stored() {
        let database: Database = my_db();
        let mut gallery = Gallery::new();
        gallery
            .load_from_database(&database)
            .expect("can't load from database");
        assert_eq!(3, gallery.len());
    }

    fn sort_and_compare_lists() -> bool {
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory("./testdata/")
            .expect("can't load from directory");
        gallery.sort_by(Order::Name);
        let list_by_name: Vec<String> = Vec::from(gallery.pictures.clone())
            .into_iter()
            .map(|p| p.file_path())
            .collect();
        gallery.sort_by(Order::Random);
        let list_by_random: Vec<String> = Vec::from(gallery.pictures.clone())
            .into_iter()
            .map(|p| p.file_path())
            .collect();
        let differences = list_by_name
            .iter()
            .zip(&list_by_random)
            .filter(|&(a, b)| a != b)
            .count();
        differences > 0
    }
    #[test]
    fn sorting_by_different_criteria() {
        // gen_white_square(); // uncomment if test file missing
        // gen_large_picture(); // ditto
        let mut result = false;
        for _ in 0..10 {
            result |= sort_and_compare_lists()
        }
        assert!(result)
    }
}
