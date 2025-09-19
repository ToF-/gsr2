use crate::image_data::ImageData;
use crate::picture::Picture;
use rusqlite::{Connection, Result, Row, params};

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn rusqlite_from_connection(connection_string: &str) -> Result<Self> {
        match Connection::open(connection_string) {
            Ok(connection) => Ok(Database { connection }),
            Err(err) => Err(err),
        }
    }

    pub fn rusqlite_insert_picture(&self, picture: &Picture) -> Result<usize> {
        self.connection.execute(
            "INSERT INTO Picture          \n\
           (FilePath,                    \n\
            Label)                       \n\
           VALUES (?1, ?2);",
            params![
                picture.file_path(),
                picture
                    .image_data()
                    .map(|data| data.label())
                    .unwrap_or(String::from(""))
            ],
        )
    }

    pub fn rusqlite_retrieve_picture(&self, file_path: &str) -> Result<Picture> {
        self.connection.query_row(
            "SELECT FilePath,           \n\
             Label                      \n\
             FROM Picture               \n\
             WHERE Id = ?1;",
            params![file_path],
            |row| Self::rusqlite_to_picture(row),
        )
    }

    fn rusqlite_to_picture(row: &Row) -> Result<Picture> {
        row.get(0).and_then(|file_path: String| {
            let file_path: String = file_path;
            row.get(1)
                .and_then(|label: String| Ok(Picture::new_with_image_data(&file_path, &label)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen_image::NINE_COLORS;
    use crate::default_values::TEST_DATABASE_FILE;

    #[test]
    fn insert_and_retrieve_a_picture() {
        let original: Picture = Picture::new_with_image_data(NINE_COLORS, "sample");
        let mut database: Database = Database::rusqlite_from_connection(TEST_DATABASE_FILE).expect("test database can't be open");
        assert_eq!(Ok(1), database.rusqlite_insert_picture(&original));

    }
}
