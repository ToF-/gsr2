use crate::model::selection::{ALL_TAGS, SOME_TAGS, Selection};
use crate::model::gallery::Gallery;
use crate::model::tags::Tags;
use crate::env::configuration::Configuration;
use crate::file::database::Database;
use std::io::Result as IOResult;
use std::cell::RefCell;
use crate::cli::args::Args;


#[derive(Debug)]
pub struct Repository {
    args: Args,
    database: Database,
    tags_rc: RefCell<Tags>,
    gallery_rc: RefCell<Gallery>,
}

impl Repository {
    pub fn new(configuration: Configuration, args: Args) -> Self {
        let database = Database::from_connection(&configuration.database_file, false).unwrap();
        Repository {
            args,
            database,
            tags_rc: RefCell::new(crate::model::tags::empty()),
            gallery_rc: RefCell::new(Gallery::new()),
        }
    }

    fn retrieve_all_tags(&mut self) {
        let mut tags = self.tags_rc.try_borrow_mut().expect("can't mutably repository tags");
        *tags = match self.database.retrieve_all_labels() {
            Ok(result) => result,
            Err(e) => panic!("{}", &format!("{}", e)),
        };
    }

    fn retrieve_all_pictures(&mut self) {
        let selection: Selection = if let Some(labels) = &self.args.select {
            Selection::from(&labels, SOME_TAGS)
        } else if let Some(labels) = &self.args.restrict {
            Selection::from(&labels, ALL_TAGS)
        } else {
            Selection::empty()
        };
        let mut gallery = self.gallery_rc.try_borrow_mut().expect("can't mutably repository gallery");
        *gallery = match self.database.retrieve_all_pictures(
            selection,
            self.args.label.clone(),
            self.args.cover,
            self.args.directory.clone()) {
            Ok(pictures) => Gallery::new_with_pictures(pictures),
            Err(e) => panic!("{}", &format!("{}", e)),
        }
    }

    pub fn initialize(&mut self) {
        self.retrieve_all_tags();
        self.retrieve_all_pictures();

    }

    pub fn all_labels(&self) -> Tags {
        let tags = self.tags_rc.try_borrow().expect("can't borrow repository tags");
        tags.clone()
    }

    pub fn gallery_rc(&self) -> RefCell<Gallery> {
        self.gallery_rc.clone()
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


    #[test]
    #[serial]
    fn given_a_db_once_initialized_it_provides_the_set_of_all_labels() {
        let args = my_args().expect("can't access to test args");
        let cfg = my_cfg();
        let mut repository = Repository::new(my_cfg(), args);
        repository.initialize();
        assert!(repository.all_labels().contains("a_rather_long_tag"));
        assert!(repository.all_labels().contains("white_square"));
    }

    #[test]
    #[serial]
    fn given_initial_args_it_provides_the_gallery_of_all_picture_matching_the_args() {
        let args = my_args().expect("can't access to test args");
        let cfg = my_cfg();
        let mut repository = Repository::new(my_cfg(), args);
        repository.initialize();
        let gallery = repository.gallery_rc.try_borrow().expect("can't borrow repository gallery");
        assert_eq!(4, gallery.len())
    }
}
