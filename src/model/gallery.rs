use crate::model::cover::cover_sort_key;
use crate::model::label::sort_key;
use crate::file::picture_file::{get_all_picture_file_paths, get_picture_file_path};
use crate::model::order::Order;
use crate::model::picture::Picture;
use crate::model::selection::Selection;
use rand::prelude::SliceRandom;
use rand::rng;
use std::cmp::Reverse;
use std::io::Result;

#[derive(Debug, Clone)]
pub struct Gallery {
    pictures: Vec<Picture>,
    order: Order,
    selection: Selection,
}

impl Gallery {
    pub fn new() -> Self {
        Gallery {
            pictures: Vec::new(),
            order: Order::Name,
            selection: Selection::empty(),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_pictures(pictures: Vec<Picture>) -> Self {
        Gallery {
            pictures,
            order: Order::Name,
            selection: Selection::empty(),
        }
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

    pub fn order(&self) -> Order {
        self.order
    }

    pub fn picture(&self, index: usize) -> Picture {
        assert!(index < self.len());
        self.pictures[index].clone()
    }

    pub fn set_picture(&mut self, index: usize, picture: Picture) {
        self.pictures[index] = picture
    }

    pub fn load_from_directory(&mut self, path: &str) -> Result<usize> {
        println!("loading directory…");
        match get_all_picture_file_paths(path) {
            Ok(list) => {
                for file_path in list {
                    match Picture::new_with_file_image_data(&file_path, "") {
                        Ok(picture) => self.pictures.push(picture),
                        Err(err) => return Err(err),
                    }
                }
                Ok(self.pictures.len())
            }
            Err(err) => Err(err),
        }
    }

    pub fn load_from_file_path(&mut self, file_path: &str) -> Result<usize> {
        match get_picture_file_path(file_path) {
            Ok(path) => match Picture::new_with_file_image_data(&path, "") {
                Ok(picture) => {
                    self.pictures.push(picture);
                    Ok(1)
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = selection.clone();
        self.sort_by(self.order)
    }

    pub fn selection(&self) -> Selection {
        self.selection.clone()
    }

    pub fn sort_by(&mut self, order: Order) {
        self.order = order;
        let selection = self.selection.clone();
        match order {
            Order::Name => self
                .pictures
                .sort_by_key(|picture| (!picture.selected(&selection), picture.file_path())),

            Order::Size => self.pictures.sort_by_key(|picture| {
                (
                    !picture.selected(&selection),
                    picture.image_data().map(|image_data| image_data.size()),
                )
            }),

            Order::Date => self.pictures.sort_by_key(|picture| {
                picture.image_data().map(|image_data| {
                    (
                        !picture.selected(&selection),
                        (true, Reverse(image_data.modified_time())),
                    )
                })
            }),
            Order::Label => self
                .pictures
                .sort_by_key(|picture| (!picture.selected(&selection), sort_key(&picture.label()))),
            Order::Value => self.pictures.sort_by_key(|picture| {
                picture
                    .image_data()
                    .map(|image_data| (!picture.selected(&selection), image_data.rank()))
            }),
            Order::ColorCount => self.pictures.sort_by_key(|picture| {
                picture
                    .image_data()
                    .map(|image_data| (!picture.selected(&selection), image_data.palette().count()))
            }),
            Order::Palette => self.pictures.sort_by_key(|picture| {
                picture.image_data().map(|image_data| {
                    (
                        !picture.selected(&selection),
                        image_data.palette().sample_as_array(),
                    )
                })
            }),
            Order::Cover => self.pictures.sort_by_key(|picture| {
                    cover_sort_key(picture.cover())
            }),
            _ => self.pictures.shuffle(&mut rng()),
        }
    }

    pub fn find_file_path(&self, file_path: &str) -> Option<usize> {
        self.pictures
            .clone()
            .into_iter()
            .position(|picture| picture.file_path() == file_path)
    }

    pub fn print(&self) {
        for picture in self.pictures.clone() {
            println!("{}", picture.file_path())
        }
    }

}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::env::default_values::TEST_DATABASE_FILE;
    use crate::file::database::Database;
    use crate::file::database::tests::{dummy_args, my_args, my_db};
    use crate::file::paths::test::current_directory;
    use crate::test_data;
    use crate::test_data::*;
    use serial_test::serial;
    use std::env::current_dir;

    #[test]
    #[serial]
    fn loading_from_a_directory_collect_all_the_picture_files_from_that_directory() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory(&test_directory())
            .expect("can't load from directory");
        gallery.sort_by(Order::Name);
        gallery.print();
        assert_eq!(4, gallery.len());
        assert_eq!(
            String::from(&large_picture_file_path()),
            gallery.picture(0).file_path()
        );
        assert_eq!(
            String::from(&nine_colors_file_path()),
            gallery.picture(1).file_path()
        );
        assert_eq!(
            String::from(&single_dot_file_path()),
            gallery.picture(2).file_path()
        );
        assert_eq!(
            String::from(white_square_file_path()),
            gallery.picture(3).file_path()
        );
    }

    #[test]
    #[serial]
    fn loading_from_a_single_file_path_collect_that_single_picture_file() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_file_path(&nine_colors_file_path())
            .expect("can't load the file");
        assert_eq!(1, gallery.len());
    }

    fn sort_and_compare_lists() -> bool {
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory(&test_directory())
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
    #[serial]
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
