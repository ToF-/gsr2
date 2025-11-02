  use crate::file::operation::execute;
use crate::file::operation::move_picture;
use crate::file_exists;
use crate::file::picture_file::collect_picture_data;
use crate::file::paths::parent_directory;
use crate::cli::args::Args;
use crate::env::configuration::Configuration;
use crate::env::default_values::THUMB_SUFFIX;
use crate::file::database::Database;
use crate::file::paths::check_path;
use crate::file::paths::check_picture_path_extension;
use crate::file::paths::file_path_to_string;
use crate::file::picture_file::get_all_picture_file_paths;
use crate::file::picture_file::get_picture_file_path;
use crate::model::gallery::Gallery;
use crate::model::picture::Picture;
use crate::model::selection::Selection;
use crate::model::tags::Tags;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::io::Result as IOResult;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Repository {
    args: Args,
    database: Database,
    tags_rc: RefCell<Tags>,
    gallery_rc: RefCell<Gallery>,
    parent_dirs: HashMap<String, usize>,
    selection: Selection,
}

impl Repository {
    pub fn new(configuration: Configuration, args: Args) -> Self {
        let database = Database::from_connection(&configuration.database_file, false).unwrap();
        Repository {
            args: args.clone(),
            database,
            tags_rc: RefCell::new(crate::model::tags::empty()),
            gallery_rc: RefCell::new(Gallery::new()),
            parent_dirs: HashMap::new(),
            selection: Selection::from_args(&args),
        }
    }

    fn retrieve_all_labels(&mut self) -> IOResult<()> {
        match self.tags_rc.try_borrow_mut() {
            Ok(mut tags) => match self.database.retrieve_all_labels() {
                Ok(labels) => {
                    *tags = Tags::from(labels);
                    Ok(())
                }
                Err(e) => return Err(e),
            },
            Err(e) => Err(IOError::other(format!("{}", e))),
        }
    }

    fn retrieve_all_pictures(&mut self, args: &Args) -> IOResult<()> {
        let selection = Selection::from_args(args);
        match self.gallery_rc.try_borrow_mut() {
            Ok(mut gallery) => {
                *gallery = match self.database.retrieve_all_pictures(
                    selection.clone(),
                    args.label.clone(),
                    args.cover,
                    args.directory.clone(),
                ) {
                    Ok(pictures) => {
                        let mut gallery = Gallery::new_with_pictures(pictures);
                        gallery.sort_by(args.order);
                        gallery
                    }
                    Err(e) => return Err(e),
                };
                Ok(())
            }
            Err(e) => panic!("{}", &format!("{}", e)),
        }
    }

    fn retrieve_all_parent_dirs(&mut self) -> IOResult<()> {
        match self.database.retrieve_all_parent_dirs() {
            Ok(map) => {
                self.parent_dirs = map;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn initialize(&mut self) -> IOResult<()> {
        self.retrieve_all_labels().and_then(|()| {
            self.retrieve_all_parent_dirs()
                .and_then(|()| self.retrieve_all_pictures(&self.args.clone()))
        })
    }

    pub fn initialize_for_args(&mut self, args: &Args) -> IOResult<()> {
        self.retrieve_all_pictures(args)
    }
    pub fn pictures_in_directory(&self, dir: &str) -> IOResult<Gallery> {
        let mut pictures: Vec<Picture> = vec![];
        get_all_picture_file_paths(dir).and_then(|list| {
            for file_path in list {
                match Picture::new_with_file_image_data(&file_path, "") {
                    Ok(picture) => pictures.push(picture),
                    Err(err) => return Err(err),
                }
            }
            Ok(Gallery::new_with_pictures(pictures))
        })
    }

    pub fn collect_data(&self) -> IOResult<()> {
        let mut count: usize = 0;
        if let Ok(mut gallery) = self.gallery_rc.try_borrow_mut() {
            let total: usize = gallery.pictures().len();
            for picture in gallery.pictures() {
                count += 1;
                match self.database.rusqlite_check_picture_with_file_path(&picture.file_path()) {
                    Ok(file_path) => {
                        println!("already in db: {}", file_path)
                    }
                    Err(_) => {
                        match collect_picture_data(&picture) {
                            Ok(picture) => match self.database.insert_picture(&picture) {
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
        } else {
            panic!("can't borrow mut")
        }
    }
    pub fn picture_from_file_path(&self, file_path: &str) -> IOResult<Gallery> {
        get_picture_file_path(file_path).and_then(|path| {
            Picture::new_with_file_image_data(&path, "")
                .map(|picture| Gallery::new_with_pictures(vec![picture]))
        })
    }

    pub fn all_labels(&self) -> Tags {
        let tags = self
            .tags_rc
            .try_borrow()
            .expect("can't borrow repository tags");
        tags.clone()
    }

    pub fn add_label(&self, label: &str) {
        let mut tags = self
            .tags_rc
            .try_borrow_mut()
            .expect("can't borrow mutably repository tags");
        tags.insert(label.to_string());
    }

    pub fn gallery_rc(&self) -> &RefCell<Gallery> {
        &self.gallery_rc
    }

    pub fn parent_dirs(&self) -> HashMap<String, usize> {
        self.parent_dirs.clone()
    }

    pub fn directory_count_at_index(&self, index: usize) -> usize {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            let picture = &gallery.pictures()[index];
            if let Some(directory) = parent_directory(&picture.file_path()) {
                if let Some(count) = self.parent_dirs().get(&directory) {
                    *count
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            panic!("can't borrow");
        }
    }

    pub fn covers(&self) -> usize {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            gallery.pictures().into_iter().map(|p| {
                if p.cover().is_some() { 1 } else { 0 }
            }).sum()
        } else {
            panic!("can't borrow")
        }
    }
    pub fn save_picture_at(&mut self, index: usize) {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            println!(
                "updating picture at index {}:{}",
                index,
                gallery.pictures()[index].rank()
            )
        }
    }

    pub fn find_index_for_file_path(&self, file_path: &str) -> Option<usize> {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            gallery.find_file_path(file_path)
        } else {
            panic!("can'tc borrow")
        }
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.set_selection(selection.clone());
        if let Ok(mut gallery) = self.gallery_rc.try_borrow_mut() {
            gallery.set_selection(selection.clone());
        } else {
            panic!("can't borrow mut")
        }
    }

    pub fn delete_picture_at_index(&mut self, index: usize) -> IOResult<()> {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            let picture = gallery.pictures()[index].clone();
            match self.database
                .delete_picture_with_file_path(&picture.file_path()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
        } else {
            panic!("can't borrow mut")
        }
    }

    pub fn list(&self, directory: Option<String>) -> IOResult<()> {
        let result = match directory {
            Some(path) => {
                match self.pictures_in_directory(&path) {
                    Ok(gallery) => {
                        gallery.print();
                        Ok(())
                    },
                    Err(e) => Err(e),
                }
            }
            None => {
                match self.gallery_rc.try_borrow() {
                    Ok(gallery) => {
                        gallery.print();
                        Ok(())
                    },
                    Err(e) => Err(IOError::other(e)),
                }
            }
        };
        match result {
            Ok(_) => {
                let parent_dirs = self.parent_dirs();
                if !parent_dirs.is_empty() {
                    println!("----- directories:{} -----", parent_dirs.len());
                    let mut dirs: Vec<String> = vec![];
                    for dir in parent_dirs.keys() {
                        dirs.push(dir.to_string());
                    }
                    dirs.sort();
                    for dir in dirs {
                        let count = parent_dirs.get(&dir).unwrap();
                        println!("{}:  {}", dir, count)
                    }
                };
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn check(&self) -> IOResult<()> {
        match self.gallery_rc.try_borrow() {
            Ok(gallery) => {
                for picture in gallery.pictures() {
                    if !file_exists(&picture.file_path()) {
                        println!("{}", picture.file_path());
                    }
                };
                Ok(())
            },
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn clean(&self) -> IOResult<()> {
        match self.gallery_rc.try_borrow() {
            Ok(gallery) => {
                for picture in gallery.pictures() {
                    if !file_exists(&picture.file_path()) {
                        self.database
                            .delete_picture_with_file_path(&picture.file_path());
                        println!("deleted from database: {}", picture.file_path());
                    }
                };
                Ok(())
            }
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn move_pictures(&self, source_dir: &str, target_dir: &str) -> IOResult<()> {
        match self.database.retrieve_all_pictures_with_parent(source_dir) {
            Ok(pictures) => {
                let mut count = 0;
                for picture in &pictures {
                    println!("moving {} to {}", picture.file_path(), target_dir);
                    let operations = move_picture(&picture.file_path(), target_dir);
                    match execute(&self.database, &operations) {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    };
                    count += 1;
                };
                println!("{} pictures moved from {} to {}", count, source_dir, target_dir);
                Ok(())
            },
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn update_picture(&self, picture: &Picture) -> IOResult <()> {
        match self.database.update_picture(picture) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }


    pub fn move_picture_at_index(&self, index: usize, target_dir: &str) -> IOResult<(usize)> {
        match self.gallery_rc().try_borrow() {
            Ok(gallery) => {
                let picture = gallery.picture(index);
                let operations = move_picture(&picture.file_path(), &target_dir);
                if operations.is_empty() {
                    println!(
                        "no operation for move of {} to {}",
                        picture.file_path(),
                        target_dir
                    );
                    Ok(0)
                } else {
                    let count = operations.len();
                    match execute(&self.database, &operations) {
                        Ok(_) => Ok(count),
                        Err(err) => Err(err),
                    }
                }
            },
            Err(e) => Err(IOError::other(e)),
        }
    }
}

#[cfg(test)]
    mod tests {
        use super::*;
        use crate::env::configuration::tests::my_cfg;
        use crate::file::database::tests::my_args;
        use crate::file::database::tests::my_db;
        use crate::file::paths::current_directory;
        use crate::model::order::Order;
        use crate::test_data::NINE_COLORS;
        use crate::test_data::TEST_DATA_DIR;
        use serial_test::serial;

        #[test]
        #[serial]
        fn given_a_db_once_initialized_it_provides_the_set_of_all_labels() {
            let args = my_args().expect("can't access to test args");
            let cfg = my_cfg();
            let mut repository = Repository::new(my_cfg(), args);
            repository.initialize().expect("can't initialize");
            assert!(repository.all_labels().contains("a_rather_long_tag"));
            assert!(repository.all_labels().contains("white_square"));
        }

        #[test]
        #[serial]
        fn after_adding_a_label_the_set_includes_this_label() {
            let args = my_args().expect("can't access to test args");
            let cfg = my_cfg();
            let mut repository = Repository::new(my_cfg(), args);
            repository.initialize().expect("can't initialize");
            assert!(!repository.all_labels().contains("a-new-label"));
            repository.add_label("a-new-label");
            assert!(repository.all_labels().contains("a-new-label"));
        }

        #[test]
        #[serial]
        fn given_initial_args_it_provides_the_gallery_of_all_picture_matching_the_args() {
            let mut args = my_args().expect("can't access to test args");
            args.order = Order::Size;
            let cfg = my_cfg();
            let mut repository = Repository::new(my_cfg(), args);
            assert!(repository.initialize().is_ok());
            let gallery_rc = repository.gallery_rc();
            let gallery = gallery_rc
                .try_borrow()
                .expect("can't borrow repository gallery");
            assert_eq!(4, gallery.len());
            println!("{:?}", gallery);
            assert!(gallery.picture(0).file_size() <= gallery.picture(1).file_size());
            assert!(gallery.picture(1).file_size() <= gallery.picture(2).file_size());
            assert!(gallery.picture(2).file_size() <= gallery.picture(3).file_size());
        }
        #[test]
        #[serial]
        fn given_a_dir_it_provides_the_gallery_of_pictures_with_only_size_and_modified_time() {
            let mut args = my_args().expect("can't access to test args");
            args.order = Order::Size;
            let cfg = my_cfg();
            let mut repository = Repository::new(my_cfg(), args);
            assert!(repository.initialize().is_ok());
            let result = repository.pictures_in_directory("testdata");
            assert!(result.is_ok());
            let gallery = result.unwrap();
            assert_eq!(4, gallery.len());
        }
        #[test]
        #[serial]
        fn given_a_file_path_it_provides_the_picture_with_only_size_and_modified_time() {
            let mut args = my_args().expect("can't access to test args");
            args.order = Order::Size;
            let cfg = my_cfg();
            let mut repository = Repository::new(my_cfg(), args);
            assert!(repository.initialize().is_ok());
            let result = repository.picture_from_file_path(&format!("testdata/{}", NINE_COLORS));
            assert!(result.is_ok());
            let gallery = result.unwrap();
            assert_eq!(1, gallery.len());
            assert!(gallery.pictures()[0].file_size() > Some(0));
        }
        #[test]
        #[serial]
        fn given_a_restriction_in_initial_args_it_provides_only_the_matching_pictures() {
            let cfg = my_cfg();
            let mut args = my_args().expect("can't access to test args");
            args.restrict = Some("foo,bar".to_string());
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            let gallery_rc = repository.gallery_rc();
            let gallery = gallery_rc
                .try_borrow()
                .expect("can't borrow repository gallery");
            assert_eq!(2, gallery.len()); // only 2 pics have both bar and foo tags, see sql/update_test_data.sql 

            args.restrict = None;
            args.label = Some("dot".to_string());
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            let gallery_rc = repository.gallery_rc();
            let gallery = gallery_rc
                .try_borrow()
                .expect("can't borrow repository gallery");
            assert_eq!(1, gallery.len()); // only 1 pic has label "dot"
            args.label = None;
            args.cover = true;
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            let gallery_rc = repository.gallery_rc();
            let gallery = gallery_rc
                .try_borrow()
                .expect("can't borrow repository gallery");
            assert_eq!(1, gallery.len()); // only 1 pic is cover
            assert!(gallery.pictures()[0].file_path().contains(NINE_COLORS));
        }
        #[test]
        #[serial]
        fn a_picture_that_is_a_cover_has_the_len_of_its_parent_dir() {
            let cfg = my_cfg();
            let mut args = my_args().expect("can't access to test args");
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            let gallery_rc = repository.gallery_rc();
            let gallery = gallery_rc
                .try_borrow()
                .expect("can't borrow repository gallery");
            let cover_picture = gallery.pictures()[1].clone();
            assert!(cover_picture.file_path().contains(NINE_COLORS));
            assert!(cover_picture.cover().is_some());
            let count = cover_picture.cover().unwrap();
            assert_eq!(4, count);
        }
        #[test]
        #[serial]
        fn provides_the_list_of_all_parent_dirs() {
            let cfg = my_cfg();
            let mut args = my_args().expect("can't access to test args");
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            let map = repository.parent_dirs();
            let count: usize = *map
                .get(&format!("{}/{}", current_directory(), TEST_DATA_DIR))
                .expect("can't access parent dir count");
            assert_eq!(4, count);
        }
        #[test]
        #[serial]
        fn can_tell_if_selection_has_covers() {
            let cfg = my_cfg();
            let mut args = my_args().expect("can't access to test args");
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            assert_eq!(1, repository.covers());
        }
    }
