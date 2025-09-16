use crate::file_system::{get_all_picture_file_paths, get_picture_file_path};
use crate::picture::Picture;
use std::io::Result;

#[derive(Debug)]
pub struct Gallery {
    pictures: Vec<Picture>,
}

impl Gallery {
    pub fn new() -> Gallery {
        Gallery {
            pictures: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.pictures.len()
    }

    pub fn picture(&self, index: usize) -> Picture {
        self.pictures[index].clone()
    }

    pub fn load_from_directory(&mut self, path: &str) -> Result<usize> {
        match get_all_picture_file_paths(path) {
            Ok(list) => {
                for file_name in list {
                    self.pictures.push(Picture::new(&file_name))
                }
                self.pictures.sort();
                Ok(self.pictures.len())
            }
            Err(err) => Err(err),
        }
    }

    pub fn load_from_file_name(&mut self, file_name: &str) -> Result<usize> {
        match get_picture_file_path(file_name) {
            Ok(path) => {
                self.pictures.push(Picture::new(&path));
                Ok(self.pictures.len())
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::gen_image::NINE_COLORS;

    #[test]
    fn loading_from_a_directory_collect_all_the_picture_files_from_that_directory() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory("./testdata/")
            .expect("can't load from directory");
        assert_eq!(2, gallery.len());
        assert_eq!(
            String::from("./testdata/nine_colors.png"),
            gallery.picture(0).file_name()
        );
        assert_eq!(
            String::from("./testdata/single_dot.png"),
            gallery.picture(1).file_name()
        );
    }

    #[test]
    fn loading_from_a_single_file_name_collect_that_single_picture_file() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_file_name(NINE_COLORS)
            .expect("can't load the file");
        assert_eq!(1, gallery.len());
    }
}
