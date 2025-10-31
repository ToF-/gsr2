use crate::Database;
use crate::model::tags::Tags;
use std::io::Result as IOResult;

pub struct Repository {
    database: Database,
}

impl Repository {
    pub fn new(database: Database) -> Self {
        Repository { database }
    }

    pub fn all_tags(&self) -> IOResult<Tags> {
        self.database.retrieve_all_labels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::database::tests::my_db;
    use serial_test::serial;

    #[test]
    #[serial]
    fn given_a_db_it_provides_the_set_of_all_labels() {
        let db = my_db();
        let repository = Repository::new(db);
        let result = repository.all_tags();
        assert!(result.is_ok());
        let tags = result.unwrap();

        assert!(tags.contains("a"));
        assert!(tags.contains("a_rather_long_tag"));
        assert!(tags.contains("bar"));
        assert!(tags.contains("bunch"));
        assert!(tags.contains("dot"));
        assert!(tags.contains("foo"));
        assert!(tags.contains("large_picture"));
        assert!(tags.contains("nine-colors"));
        assert!(tags.contains("of"));
        assert!(tags.contains("qux"));
        assert!(tags.contains("tags"));
        assert!(tags.contains("white_square"));
    }
}
