use crate::file_system::get_all_picture_file_paths;
use crate::picture::Picture;
use std::io::Result;

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

    pub fn load_from_directory(&mut self, path: &str) -> Result<usize> {
        match get_all_picture_file_paths(path) {
            Ok(list) => {
                for file_name in list {
                    self.pictures.push(Picture::new(&file_name))
                }
                Ok(self.pictures.len())
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn loading_from_a_directory_collect_all_the_picture_files_from_that_directory() {
        let mut gallery = Gallery::new();
        gallery
            .load_from_directory("./testdata/")
            .expect("can't load from directory");
        assert_eq!(2, gallery.len());
    }
}
