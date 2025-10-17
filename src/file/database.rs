use crate::model::image_data::ImageData;
use crate::model::picture::Picture;
use rusqlite::{Connection, Result as SqlResult, Row, params};
use std::collections::HashMap;
use std::env;
use std::io::Result as IOResult;

pub type ImageDataMap = HashMap<String, ImageData>;

#[derive(Debug)]
pub struct Database {
    home_dir: Option<String>,
    connection: Connection,
}

impl Database {
    pub fn from_connection(connection_string: &str) -> std::io::Result<Self> {
        match Self::rusqlite_from_connection(connection_string) {
            Ok(database) => Ok(database),
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    fn rusqlite_from_connection(connection_string: &str) -> SqlResult<Self> {
        println!("connecting to {connection_string}…");
        match Connection::open(connection_string) {
            Ok(connection) => Ok(Database {
                home_dir: env::home_dir().map(|path| path.display().to_string()),
                connection,
            }),
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub fn rusqlite_insert_picture(&self, picture: &Picture) -> SqlResult<usize> {
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

    pub fn rusqlite_update_picture(&self, picture: &Picture) -> SqlResult<usize> {
        self.connection.execute(
            "UPDATE Picture            \n\
               SET Label = ?2             \n\
               WHERE FilePath = ?1;",
            params![picture.file_path(), picture.label(),],
        )
    }

    pub fn rusqlite_delete_picture_with_file_path(&self, file_path: &str) -> SqlResult<usize> {
        self.connection.execute(
            "DELETE FROM Picture        \n\
            WHERE FilePath = ?1;",
            params![file_path.to_string()],
        )
    }

    #[allow(dead_code)]
    pub fn rusqlite_retrieve_picture_with_file_path(&self, file_path: &str) -> SqlResult<Picture> {
        self.connection.query_row(
            "SELECT FilePath,           \n\
             Label                      \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;",
            params![file_path],
            Self::rusqlite_row_to_picture,
        )
    }

    pub fn rusqlite_retrieve_all_pictures(&self) -> SqlResult<ImageDataMap> {
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

    pub fn retrieve_all_pictures(&self) -> IOResult<Vec<Picture>> {
        match self.rusqlite_retrieve_all_pictures() {
            Ok(map) => {
                let mut pictures: Vec<Picture> = vec![];
                for (file_path, image_data) in map.iter() {
                    match Picture::new_with_file_image_data(file_path, &image_data.label()) {
                        Ok(picture) => pictures.push(picture),
                        Err(err) => return Err(err),
                    }
                }
                Ok(pictures)
            }
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    fn rusqlite_row_to_picture(row: &Row) -> SqlResult<Picture, rusqlite::Error> {
        row.get(0).and_then(|file_path: String| {
            row.get(1)
                .and_then(|label: String| Ok(Picture::new_with_label(&file_path, &label)))
        })
    }

    pub fn file_path_as_stored(&self, file_path: &str) -> String {
        if let Some(home_dir) = &self.home_dir {
            if file_path.starts_with(&home_dir.to_string()) {
                let result = file_path.to_string();
                result.replace(&home_dir.to_string(), "~")
            } else {
                file_path.to_string()
            }
        } else {
            file_path.to_string()
        }
    }

    pub fn file_path_as_retrieved(&self, file_path: &str) -> String {
        if let Some(home_dir) = &self.home_dir {
            if file_path.starts_with("~") {
                let result = file_path.to_string();
                result.replace("~", &home_dir.to_string())
            } else {
                file_path.to_string()
            }
        } else {
            file_path.to_string()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::env::default_values::TEST_DATABASE_FILE;
    use crate::test_data::*;

    pub fn my_db() -> Database {
        let database = Database::rusqlite_from_connection(TEST_DATABASE_FILE)
            .expect("test database can't be open");
        database
            .connection
            .execute("DELETE FROM Picture;", [])
            .expect("db error");
        database.connection.execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/nine_colors.png', 'sample');",
            [],
        ).expect("db error");
        database
            .connection
            .execute(
                "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/single_dot.png', '');",
                [],
            )
            .expect("db error");
        database.connection.execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/white_square.png', 'foo');",
            [],
        ).expect("db error");
        database.connection.execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/large_picture.png', 'foo');",
            [],
        ).expect("db error");
        database
    }

    pub fn delete_nine_colors_from_db() {
        let database = my_db();
        let _ = database.rusqlite_delete_picture_with_file_path(NINE_COLORS);
    }

    pub fn insert_nine_colors_sample_into_db() {
        let database = my_db();
        let picture: Picture =
            Picture::new_with_file_image_data(NINE_COLORS, "sample").expect("can't create picture");
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
    fn update_a_picture() {
        let database = my_db();
        delete_nine_colors_from_db();
        insert_nine_colors_sample_into_db();
        if let Ok(mut retrieved) = database.rusqlite_retrieve_picture_with_file_path(NINE_COLORS) {
            if let Some(image_data) = retrieved.image_data() {
                assert_eq!(String::from("sample"), image_data.label())
            } else {
                assert!(false, "there was no label")
            }
            retrieved.set_label("bingo");
            assert_eq!("bingo", &retrieved.label());
            match database.rusqlite_update_picture(&retrieved) {
                Ok(updated) => {
                    if let Ok(updated) =
                        database.rusqlite_retrieve_picture_with_file_path(NINE_COLORS)
                    {
                        assert_eq!(String::from("bingo"), updated.label())
                    } else {
                        assert!(false, "could not retrieve the picture")
                    }
                }
                Err(err) => assert!(false, "{}", err),
            }
        } else {
            assert!(false, "could not retrieve the picture")
        };
    }

    #[test]
    fn retrieve_all_pictures_ordered_by_file_path() {
        let database = my_db();
        let status: SqlResult<ImageDataMap> = database.rusqlite_retrieve_all_pictures();
        assert!(status.is_ok());
        let map = status.unwrap();
        assert_eq!(4, map.len());
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

    #[test]
    fn file_path_starting_with_home_dir_are_tilded_as_stored() {
        let database = my_db();
        if let Some(path) = env::home_dir() {
            let this_file_path = path.display().to_string() + "/test_file.jpg";
            let expected = "~/test_file.jpg";
            assert_eq!(expected, database.file_path_as_stored(&this_file_path))
        }
    }

    #[test]
    fn file_path_starting_with_tilde_are_developped_as_retrieved() {
        let database = my_db();
        if let Some(path) = env::home_dir() {
            let this_file_path = "~/test_file.jpg";
            let expected = path.display().to_string() + "/test_file.jpg";
            assert_eq!(expected, database.file_path_as_retrieved(&this_file_path));
        }
    }
}
