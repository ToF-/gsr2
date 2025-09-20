use crate::file_system::{get_all_picture_file_paths, get_picture_file_path};
use crate::order::Order;
use crate::picture::Picture;
use rand::prelude::SliceRandom;
use rand::rng;
use std::io::Result;

#[derive(Debug)]
pub struct Gallery {
    pictures: Vec<Picture>,
}

impl Gallery {
    pub fn new() -> Self {
        Gallery {
            pictures: Vec::new(),
        }
    }

    pub fn new_with_pictures(pictures: Vec<Picture>) -> Self {
        Gallery { pictures: pictures }
    }
    pub fn len(&self) -> usize {
        self.pictures.len()
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
            Order::Name => self
                .pictures
                .sort_by(|a, b| a.file_path().cmp(&b.file_path())),
            Order::Random => self.pictures.shuffle(&mut rng()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::database::Database;
    use crate::database::tests::{delete_nine_colors_from_db, insert_nine_colors_sample_into_db};
    use crate::default_values::TEST_DATABASE_FILE;
    use crate::gen_image::{NINE_COLORS, gen_white_square};

    #[test]
    fn loading_from_a_directory_collect_all_the_picture_files_from_that_directory() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory("./testdata/")
            .expect("can't load from directory");
        gallery.sort_by(Order::Name);
        assert_eq!(3, gallery.len());
        assert_eq!(
            String::from("./testdata/nine_colors.png"),
            gallery.picture(0).file_path()
        );
        assert_eq!(
            String::from("./testdata/single_dot.png"),
            gallery.picture(1).file_path()
        );
        assert_eq!(
            String::from("./testdata/white_square.png"),
            gallery.picture(2).file_path()
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
        let mut result = false;
        for _ in 0..10 {
            result |= sort_and_compare_lists()
        }
        assert!(result)
    }
}
