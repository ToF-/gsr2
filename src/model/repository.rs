use crate::model::tags::Tags;
use crate::env::configuration::Configuration;
use crate::file::database::Database;
use std::io::Result as IOResult;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Repository {
    database: Database,
    tags_rc: RefCell<Tags>,
}

impl Repository {
    pub fn new(congiguration: Configuration) -> Self {
        let database = Database::from_connection(&congiguration.database_file, false).unwrap();
        Repository {
            database,
            tags_rc: RefCell::new(crate::model::tags::empty()),
        }
    }

    pub fn initialize(&mut self) {
        let mut tags = self.tags_rc.try_borrow_mut().expect("can't mutably repository tags");
        *tags = match self.database.retrieve_all_labels() {
            Ok(result) => result,
            Err(e) => panic!("{}", &format!("{}", e)),
        };
    }

    pub fn all_labels(&self) -> Tags {
        let tags = self.tags_rc.try_borrow().expect("can't borrow repository tags");
        tags.clone()
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

    #[test]
    #[serial]
    fn given_a_db_once_initialized_it_provides_the_set_of_all_labels() {
        let cfg = my_cfg();
        let mut repository = Repository::new(my_cfg());
        repository.initialize();
        assert!(repository.all_labels().contains("a"));
        assert!(repository.all_labels().contains("a_rather_long_tag"));
        assert!(repository.all_labels().contains("bar"));
        assert!(repository.all_labels().contains("bunch"));
        assert!(repository.all_labels().contains("dot"));
        assert!(repository.all_labels().contains("foo"));
        assert!(repository.all_labels().contains("large_picture"));
        assert!(repository.all_labels().contains("nine-colors"));
        assert!(repository.all_labels().contains("of"));
        assert!(repository.all_labels().contains("qux"));
        assert!(repository.all_labels().contains("white_square"));
    }
}
