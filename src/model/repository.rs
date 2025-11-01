use std::collections::HashMap;
use crate::file::picture_file::get_all_picture_file_paths;
use crate::model::picture::Picture;
use crate::file::picture_file::get_picture_file_path;
use crate::file::paths::file_path_to_string;
use crate::file::paths::check_path;
use walkdir::WalkDir;
use crate::file::paths::check_picture_path_extension;
use crate::env::default_values::THUMB_SUFFIX;
use crate::model::selection::Selection;
use crate::model::gallery::Gallery;
use crate::model::tags::Tags;
use crate::env::configuration::Configuration;
use crate::file::database::Database;
use std::io::Result as IOResult;
use std::io::Error as IOError;
use std::cell::RefCell;
use crate::cli::args::Args;


#[derive(Debug)]
pub struct Repository {
    args: Args,
    database: Database,
    tags_rc: RefCell<Tags>,
    gallery_rc: RefCell<Gallery>,
    parent_dirs: HashMap<String, usize>,
}

impl Repository {
    pub fn new(configuration: Configuration, args: Args) -> Self {
        let database = Database::from_connection(&configuration.database_file, false).unwrap();
        Repository {
            args,
            database,
            tags_rc: RefCell::new(crate::model::tags::empty()),
            gallery_rc: RefCell::new(Gallery::new()),
            parent_dirs: HashMap::new(),
        }
    }

    fn retrieve_all_labels(&mut self) -> IOResult<()> {
        match self.tags_rc.try_borrow_mut() {
            Ok(mut tags) => match self.database.retrieve_all_labels() {
                Ok(labels) => {
                    *tags = Tags::from(labels);
                    Ok(())
                },
                Err(e) => return Err(e),
            },
            Err(e) => Err(IOError::other(format!("{}",e))),
        }
    }

    fn retrieve_all_pictures(&mut self) -> IOResult<()> {
        let selection = Selection::from_args(&self.args);
        match self.gallery_rc.try_borrow_mut() {
            Ok(mut gallery) => { 
                *gallery = match self.database.retrieve_all_pictures(
                    selection.clone(),
                    self.args.label.clone(),
                    self.args.cover,
                    self.args.directory.clone()) {
                    Ok(pictures) => {
                        let mut gallery = Gallery::new_with_pictures(pictures);
                        gallery.sort_by(self.args.order);
                        gallery
                    },
                    Err(e) => return Err(e),
                };
                Ok(())
            },
            Err(e) => panic!("{}", &format!("{}", e)),
        }
    }

    fn retrieve_all_parent_dirs(&mut self) -> IOResult<()> {
       match self.database.retrieve_all_parent_dirs() {
           Ok(map) => {
               self.parent_dirs = map;
               Ok(())
           },
           Err(e) => Err(e)
       }
    }

    pub fn initialize(&mut self) -> IOResult<()> {
        self.retrieve_all_labels().and_then(|()|
            self.retrieve_all_parent_dirs().and_then(|()|
                self.retrieve_all_pictures()))
    }

    pub fn pictures_in_directory(&self, dir: &str) -> IOResult<Gallery> {
        let mut pictures: Vec<Picture> = vec![];
        get_all_picture_file_paths(dir)
            .and_then(|list| {
                for file_path in list {
                    match Picture::new_with_file_image_data(&file_path, "") {
                        Ok(picture) => pictures.push(picture),
                        Err(err) => return Err(err),
                    }
                };
                Ok(Gallery::new_with_pictures(pictures))
            })
    }

    pub fn picture_from_file_path(&self, file_path: &str) -> IOResult<Gallery> {
        get_picture_file_path(file_path)
            .and_then(|path| {
                Picture::new_with_file_image_data(&path, "").map(|picture| {
                    Gallery::new_with_pictures(vec![picture])
                })
            })
    }

    pub fn all_labels(&self) -> Tags {
        let tags = self.tags_rc.try_borrow().expect("can't borrow repository tags");
        tags.clone()
    }

    pub fn add_label(&self, label: &str) {
        let mut tags = self.tags_rc.try_borrow_mut().expect("can't borrow mutably repository tags");
        tags.insert(label.to_string());
    }

    pub fn gallery_rc(&self) -> RefCell<Gallery> {
        self.gallery_rc.clone()
    }

    pub fn parent_dirs(&self) -> HashMap<String, usize> {
        self.parent_dirs.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::database::tests::my_db;
    use crate::test_data::TEST_DATA_DIR;
    use serial_test::serial;
    use crate::env::configuration::tests::my_cfg;
    use crate::file::paths::current_directory;
    use crate::file::database::tests::my_args;
    use crate::model::order::Order;
    use crate::test_data::NINE_COLORS;

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
        let gallery = gallery_rc.try_borrow().expect("can't borrow repository gallery");
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
    fn given_a_file_path_it_provides_the_picture_with_only_size_and_modified_time()  {
        let mut args = my_args().expect("can't access to test args");
        args.order = Order::Size;
        let cfg = my_cfg();
        let mut repository = Repository::new(my_cfg(), args);
        assert!(repository.initialize().is_ok());
        let result = repository.picture_from_file_path(&format!("testdata/{}",NINE_COLORS));
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
        let gallery = gallery_rc.try_borrow().expect("can't borrow repository gallery");
        assert_eq!(2, gallery.len()); // only 2 pics have both bar and foo tags, see sql/update_test_data.sql 

        args.restrict = None;
        args.label = Some("dot".to_string());
        let mut repository = Repository::new(my_cfg(), args.clone());
        assert!(repository.initialize().is_ok());
        let gallery_rc = repository.gallery_rc();
        let gallery = gallery_rc.try_borrow().expect("can't borrow repository gallery");
        assert_eq!(1, gallery.len()); // only 1 pic has label "dot"
        args.label = None;
        args.cover = true;
        let mut repository = Repository::new(my_cfg(), args.clone());
        assert!(repository.initialize().is_ok());
        let gallery_rc = repository.gallery_rc();
        let gallery = gallery_rc.try_borrow().expect("can't borrow repository gallery");
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
        let gallery = gallery_rc.try_borrow().expect("can't borrow repository gallery");
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
        let count: usize = *map.get(&format!("{}/{}", current_directory(), TEST_DATA_DIR)).expect("can't access parent dir count");
        assert_eq!(4, count);
    }
}
