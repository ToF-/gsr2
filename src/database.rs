use crate::image_data::ImageData;
use crate::picture::Picture;
use rusqlite::{Connection, Result, Row, params};
use std::collections::HashMap;

pub type ImageDataMap = HashMap<String, ImageData>;

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn from_connection(connection_string: &str) -> std::io::Result<Self> {
        match Self::rusqlite_from_connection(connection_string) {
            Ok(database) => Ok(database),
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    fn rusqlite_from_connection(connection_string: &str) -> Result<Self> {
        println!("connecting to {connection_string}…");
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
            |row| Self::rusqlite_row_to_picture(row),
        )
    }

    pub fn rusqlite_retrieve_all_pictures(&self) -> Result<ImageDataMap> {
        self.connection
            .prepare(
                "SELECT FilePath, Label           \n\
            FROM Picture ORDER BY FilePath;",
            )
            .and_then(|mut statement| {
                let mut map: ImageDataMap = HashMap::new();
                statement.query([]).and_then(|mut rows| {
                    while let Some(row) = rows.next().unwrap() {
                        match Self::rusqlite_row_to_picture(row) {
                            Ok(picture) => {
                                let _ =
                                    map.insert(picture.file_path(), picture.image_data().unwrap());
                            }
                            Err(err) => {
                                eprintln!("{}", err);
                                return Err(err);
                            }
                        }
                    }
                    Ok(map)
                })
            })
    }

    pub fn retrieve_all_pictures(&self) -> Result<Vec<Picture>> {
        let result = self.rusqlite_retrieve_all_pictures().and_then(|map| {
            let mut pictures: Vec<Picture> = vec![];
            for (file_path, image_data) in map.iter() {
                pictures.push(Picture::new_with_image_data(file_path, &image_data.label()))
            }
            Ok(pictures)
        });
        match result {
            Ok(pictures) => Ok(pictures),
            Err(err) => Err(err),
        }
    }

    fn rusqlite_row_to_picture(row: &Row) -> Result<Picture> {
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
    use crate::gen_image::{NINE_COLORS, SINGLE_DOT, WHITE_SQUARE};
    use crate::image_data;

    pub fn my_db() -> Database {
        let database = Database::rusqlite_from_connection(TEST_DATABASE_FILE)
            .expect("test database can't be open");
        database.connection.execute("DELETE FROM Picture;", []);
        database.connection.execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/nine_colors.png', 'sample');",
            [],
        );
        database.connection.execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/single_dot.png', '');",
            [],
        );
        database.connection.execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/white_square.png', 'foo');",
            [],
        );
        database
    }

    pub fn delete_nine_colors_from_db() {
        let database = my_db();
        let _ = database.rusqlite_delete_picture_with_file_path(NINE_COLORS);
    }

    pub fn insert_nine_colors_sample_into_db() {
        let database = my_db();
        let picture: Picture = Picture::new_with_image_data(NINE_COLORS, "sample");
        let _ = database.rusqlite_insert_picture(&picture);
    }

    #[test]
    fn insert_and_retrieve_a_picture() {
        let database = my_db();
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
    #[test]
    fn retrieve_all_pictures_ordered_by_file_path() {
        let database = my_db();
        let status: Result<ImageDataMap> = database.rusqlite_retrieve_all_pictures();
        assert!(status.is_ok());
        let map = status.unwrap();
        assert_eq!(3, map.len());
        assert_eq!(
            "sample".to_string(),
            map.get(NINE_COLORS).unwrap().clone().label()
        );
        assert_eq!(
            "foo".to_string(),
            map.get(WHITE_SQUARE).unwrap().clone().label()
        );
        assert_eq!("".to_string(), map.get(SINGLE_DOT).unwrap().clone().label());
    }
}
