use crate::model::order::Order;
use crate::file::picture_file::copy_picture_file_to_directory;
use regex::Regex;
use crate::file::picture_file::delete_picture_files;
use crate::cli::command::Command;
use crate::file::operation::execute;
use crate::file::operation::move_picture;
use crate::file_exists;
use crate::file::picture_file::collect_picture_data;
use crate::file::paths::parent_directory;
use crate::cli::args::Args;
use crate::env::configuration::Configuration;
use crate::file::database::Database;
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

#[derive(Debug, Clone)]
pub struct Repository {
    args: Args,
    on_database: bool,
    database: Database,
    tags_rc: RefCell<Tags>,
    gallery_rc: RefCell<Gallery>,
    parent_dirs: HashMap<String, (usize, usize)>,
    len: usize,
    temp_dir: String,
}

impl Repository {
    pub fn new(configuration: Configuration, args: Args) -> Self {
        let database = Database::from_connection(&configuration.database_file, false).unwrap();
        Repository {
            args: args.clone(),
            on_database: true,
            database,
            tags_rc: RefCell::new(crate::model::tags::empty()),
            gallery_rc: RefCell::new(Gallery::new()),
            parent_dirs: HashMap::new(),
            len: 0,
            temp_dir: configuration.temp_dir,
        }
    }

    fn retrieve_all_labels(&mut self) -> IOResult<()> {
        match self.tags_rc.try_borrow_mut() {
            Ok(mut tags) => match self.database.retrieve_all_labels() {
                Ok(labels) => {
                    *tags = Tags::from(labels);
                    Ok(())
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(IOError::other(format!("{}", e))),
        }
    }

    fn retrieve_all_pictures(&mut self, args: &Args) -> IOResult<()> {
        let selection = Selection::from_args(args);
        match self.gallery_rc.try_borrow_mut() {
            Ok(mut gallery) => {
                let regex: Option<Regex> = match args.clone().pattern {
                    Some(pattern) => match Regex::new(&pattern) {
                        Ok(re) => Some(re),
                        Err(e) => { eprintln!("{}", e); None },
                    },
                    None => None,
                };
                *gallery = match self.database.retrieve_all_pictures(
                    selection.clone(),
                    args.label.clone(),
                    regex,
                    args.cover,
                    args.directory.clone(),
                ) {
                    Ok(pictures) => {
                        let mut gallery = Gallery::new_with_pictures(pictures);
                        gallery.sort_by(args.order.unwrap_or(Order::Name));
                        self.len = gallery.len();
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

   pub fn len(&self) -> usize {
        self.len
    }

   pub fn order(&self) -> Order {
       if let Ok(gallery) = self.gallery_rc.try_borrow() {
           gallery.order()
       } else {
           panic!("can't borrow")
       }
   }
   pub fn initialize(&mut self) -> IOResult<()> {
       match &self.args.command {
           Some(Command::File { file_path }) => {
               self.on_database = false;
               match self.picture_from_file_path(file_path) {
                   Ok(file_gallery) => match self.gallery_rc.try_borrow_mut() {
                       Ok(mut gallery) => {
                           *gallery = file_gallery.clone();
                           self.len = file_gallery.len();
                           Ok(())
                       },
                       Err(e) => Err(IOError::other(e)),
                   },
                   Err(e) => Err(e),
               }
           },
           Some(Command::Directory { directory }) => {
               self.on_database = false;
               match self.pictures_in_directory(directory) {
                   Ok(dir_gallery) => match self.gallery_rc.try_borrow_mut() {
                       Ok(mut gallery) => {
                           *gallery = dir_gallery.clone();
                           self.len = dir_gallery.len();
                           Ok(())
                       },
                       Err(e) => Err(IOError::other(e)),
                   },
                   Err(e) => Err(e),
               }
           }
           _ => {
               self.on_database = true;
               self.retrieve_all_labels().and_then(|()| {
                   self.retrieve_all_parent_dirs()
                       .and_then(|()| self.retrieve_all_pictures(&self.args.clone()))
               })
           },
       }
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
        println!("gallery count before collect:{}\n", self.len());
        if let Some(Command::Collect { directory }) = &self.args.command {
            match self.pictures_in_directory(directory) {
                Ok(dir_gallery) => {
                    println!("pictures in directory {} : {}\n", &directory, dir_gallery.clone().len());
                    let total: usize = dir_gallery.len();
                    let mut count: usize = 0;
                    for picture in dir_gallery.pictures() {
                        match self.database.rusqlite_check_picture_with_file_path(&picture.file_path()) {
                            Ok(_) => { }
                            Err(_) => {
                                match collect_picture_data(picture) {
                                    Ok(picture) => match self.database.insert_picture(&picture) {
                                        Ok(_) => {
                                            count += 1;
                                            println!("{}/{}:{}", count, total, &picture.file_path());
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
                    };
                    println!("{} pictures added", count);
                    Ok(())
                },
                Err(e) => Err(e),
            }
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

    pub fn parent_dirs(&self) -> HashMap<String, (usize,usize)> {
        self.parent_dirs.clone()
    }

    pub fn directory_count_at_index(&self, index: usize) -> (usize,usize) {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            let picture = &gallery.pictures()[index];
            if let Some(directory) = parent_directory(&picture.file_path()) {
                if let Some(count) = self.parent_dirs().get(&directory) {
                    *count
                } else {
                    (0,0)
                }
            } else {
                (0,0)
            }
        } else {
            panic!("can't borrow");
        }
    }

    pub fn covers(&self) -> usize {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            gallery.pictures().iter().map(|p| {
                if p.cover().is_some() { 1 } else { 0 }
            }).sum()
        } else {
            panic!("can't borrow")
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
        if let Ok(mut gallery) = self.gallery_rc.try_borrow_mut() {
            gallery.set_selection(selection.clone());
            self.len = gallery.len();
        } else {
            panic!("can't borrow mut")
        }
    }

    pub fn delete_picture_at_index(&mut self, index: usize) -> IOResult<()> {
            if let Ok(gallery) = self.gallery_rc.try_borrow() {
                let picture = gallery.pictures()[index].clone();
                    let file_path = picture.file_path();
                if self.on_database {
                    self.database.delete_picture_with_file_path(&file_path)
                            .and_then(|_| {
                                match delete_picture_files(&file_path) {
                                    Ok(_) => Ok(()),
                                    Err(err) => Err(err),
                                }
                            })
                } else {
                    match delete_picture_files(&file_path) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                }
            } else {
                panic!("can't borrow mut");
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
                        let counts = parent_dirs.get(&dir).unwrap();
                        let count = counts.0;
                        let covers = counts.1;
                        println!("{}:  {}({})", dir, count, covers)
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
                println!("checking pictures where picture file does not exists");
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
                println!("deleting picture data where picture file does not exists");
                let mut count: usize = 0;
                let mut deleted: usize = 0;
                let total = gallery.len();
                for picture in gallery.pictures() {
                    if !file_exists(&picture.file_path()) {
                        match self.database
                            .delete_picture_with_file_path(&picture.file_path()) {
                                Ok(_) => {
                                    println!("deleted from database: {}", picture.file_path());
                                    deleted += 1;
                                },
                                Err(e) => {
                                    eprintln!("{}",e);
                                },
                            }
                    }
                    count += 1;
                    println!("{}/{}…", count, total);
                };
                println!("{} picture records deleted", deleted); 
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


    pub fn move_picture_at_index(&self, index: usize, target_dir: &str) -> IOResult<usize> {
        match self.gallery_rc().try_borrow() {
            Ok(gallery) => {
                let picture = gallery.picture(index);
                let operations = move_picture(&picture.file_path(), target_dir);
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
    pub fn copy_picture_at_index_to_temp_dir(&self, index: usize) -> IOResult<()> {
        match self.gallery_rc().try_borrow() {
            Ok(gallery) => {
                let picture = gallery.picture(index);
                println!("copying {} to {}", &picture.file_path(), &self.temp_dir);
                copy_picture_file_to_directory(&picture.file_path(), &self.temp_dir)
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
        use crate::file::paths::test::current_directory;
        use crate::model::order::Order;
        use crate::test_data::NINE_COLORS;
        use crate::test_data::TEST_DATA_DIR;
        use serial_test::serial;

        #[test]
        #[serial]
        fn given_a_db_once_initialized_it_provides_the_set_of_all_labels() {
            let args = my_args().expect("can't access to test args");
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
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            let gallery_rc = repository.gallery_rc();
            let gallery = gallery_rc
                .try_borrow()
                .expect("can't borrow repository gallery");
            assert_eq!(4, gallery.len());
            println!("{:?}", args);
            assert!(gallery.picture(0).file_size() <= gallery.picture(1).file_size());
            assert!(gallery.picture(1).file_size() <= gallery.picture(2).file_size());
            assert!(gallery.picture(2).file_size() <= gallery.picture(3).file_size());
        }
        #[test]
        #[serial]
        fn given_a_dir_it_provides_the_gallery_of_pictures_with_only_size_and_modified_time() {
            let mut args = my_args().expect("can't access to test args");
            args.order = Order::Size;
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
            let mut args = my_args().expect("can't access to test args");
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            let map = repository.parent_dirs();
            let counts: (usize,usize) = *map
                .get(&format!("{}/{}", current_directory(), TEST_DATA_DIR))
                .expect("can't access parent dir count");
            assert_eq!((4,1), counts);
        }
        #[test]
        #[serial]
        fn can_tell_if_selection_has_covers() {
            let mut args = my_args().expect("can't access to test args");
            let mut repository = Repository::new(my_cfg(), args.clone());
            assert!(repository.initialize().is_ok());
            assert_eq!(1, repository.covers());
        }
        // #[test]
        #[serial]
        fn initializing_on_a_dir_command_sets_database_off() {
            let cfg = my_cfg();
            let cmd: Option<Vec<&str>> = Some(vec!["dir","testdata"]);
            let my_args = Args::parse_and_check(cmd, &cfg).unwrap();
            let mut repository = Repository::new(my_cfg(), my_args);

        }
    }
