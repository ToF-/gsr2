use crate::gallery::Gallery;
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

    pub fn rusqlite_delete_picture_with_file_path(&self, file_path: &str) -> Result<usize> {
        self.connection.execute(
            "DELETE FROM Picture        \n\
            WHERE FilePath = ?1;",
            params![file_path.to_string()],
        )
    }

    pub fn rusqlite_retrieve_picture_with_file_path(&self, file_path: &str) -> Result<Picture> {
        self.connection.query_row(
            "SELECT FilePath,           \n\
             Label                      \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;",
            params![file_path],
            |row| Self::rusqlite_to_picture(row),
        )
    }

    pub fn rusqlite_load_image_data_for_directory(
        &self,
        dir: &str,
        gallery: &Gallery,
    ) -> Result<Gallery> {
        let result: Gallery = Gallery::new();
        Ok(result)
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
pub mod tests {
    use super::*;
    use crate::default_values::TEST_DATABASE_FILE;
    use crate::gen_image::NINE_COLORS;

    pub fn delete_nine_colors_from_db() {
        let database: Database = Database::rusqlite_from_connection(TEST_DATABASE_FILE)
            .expect("test database can't be open");
        let _ = database.rusqlite_delete_picture_with_file_path(NINE_COLORS);
    }

    pub fn insert_nine_colors_sample_into_db() {
        let database: Database = Database::rusqlite_from_connection(TEST_DATABASE_FILE)
            .expect("test database can't be open");
        let picture: Picture = Picture::new_with_image_data(NINE_COLORS, "sample");
        let _ = database.rusqlite_insert_picture(&picture);
    }

    #[test]
    fn insert_and_retrieve_a_picture() {
        let database: Database = Database::rusqlite_from_connection(TEST_DATABASE_FILE)
            .expect("test database can't be open");
        delete_nine_colors_from_db();
        insert_nine_colors_sample_into_db();
        if let Ok(retrieved) = database.rusqlite_retrieve_picture_with_file_path(NINE_COLORS) {
            if let Some(image_data) = retrieved.image_data() {
                assert_eq!(String::from("sample"), image_data.label())
            } else {
                assert!(false, "there was no label")
            }
        } else {
            assert!(false, "could not retrieve the picture")
        }
    }
}
