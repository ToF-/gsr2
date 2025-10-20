use crate::model::palette::Palette;
use std::time::SystemTime;
use std::time::Duration;
use std::collections::HashSet;
use std::time::UNIX_EPOCH;
use crate::model::image_data::ImageData;
use crate::model::picture::Picture;
use rusqlite::{Connection, Result as SqlResult, Row, params};
use std::collections::HashMap;
use std::env;
use std::io::Result as IOResult;
use std::rc::Rc;
use std::cell::{RefCell,Ref};



pub type ImageDataMap = HashMap<String, ImageData>;

#[derive(Debug, Clone)]
pub struct Database {
    home_dir: Option<String>,
    connection_rc: Rc<RefCell<Connection>>,
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
                connection_rc: Rc::new(RefCell::new(connection)),
            }),
            Err(err) => Err(err),
        }
    }


    pub fn connection(&self) -> Ref<Connection> {
        if let Ok(connection) = self.connection_rc.try_borrow() {
            connection
        } else {
            panic!("can't open database connection")
        }
    }

    pub fn rusqlite_insert_picture(&self, picture: &Picture) -> SqlResult<usize> {
        let image_data = match picture.image_data() {
            Some(data) => data,
            None => ImageData::new(""),
        };
        println!("image_data:\n{:?}\n", image_data);
        self.connection().execute(
            "INSERT INTO Picture (        \n\
             FilePath,                    \n\
             Label,                       \n\
             FileSize,                    \n\
             ModifiedTime,                \n\
             Rank,                        \n\
             SampleSize,                  \n\
             Sample,                      \n\
             ColorCount,                  \n\
             Cover)                       \n\
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
             params![
             self.file_path_as_stored(&picture.file_path()),
             image_data.label(),
             image_data.size(),
             image_data.modified_time(),
             0, // rank to do
             image_data.palette().sample().len(),
             [], // image_data.palette.sample(),
             image_data.palette.count(),
             false],
        )
    }

    pub fn rusqlite_update_picture(&self, picture: &Picture) -> SqlResult<usize> {
        self.connection().execute(
            "UPDATE Picture            \n\
               SET Label = ?2             \n\
               WHERE FilePath = ?1;",
            params![picture.file_path(), picture.label(),],
        )
    }

    pub fn rusqlite_delete_picture_with_file_path(&self, file_path: &str) -> SqlResult<usize> {
        self.connection().execute(
            "DELETE FROM Picture        \n\
            WHERE FilePath = ?1;",
            params![file_path.to_string()],
        )
    }

    pub fn rusqlite_retrieve_picture_with_file_path(&self, file_path: &str) -> SqlResult<Picture> {
        self.connection().query_row(
            "SELECT FilePath,           \n\
             Label,                     \n\
             FileSize,                  \n\
             ModifiedTime,              \n\
             Rank,                      \n\
             SampleSize,                \n\
             Sample,                    \n\
             ColorCount,                \n\
             Cover                      \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;",
            params![file_path],
            Self::rusqlite_row_to_picture,
        )
    }

    pub fn rusqlite_retrieve_all_pictures(&self) -> SqlResult<ImageDataMap> {
        self.connection()
            .prepare(
                "SELECT FilePath, Label           \n\
            FROM Picture ORDER BY FilePath;",
            )
            .and_then(|mut statement| {
                let mut map: ImageDataMap = HashMap::new();
                statement.query([]).and_then(|mut rows| {
                    while let Some(row) = rows.next().unwrap() {
                        match Self::rusqlite_row_to_mere_picture(row) {
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


    fn rusqlite_row_to_mere_picture(row: &Row) -> SqlResult<Picture, rusqlite::Error> {
        let file_path: String = row.get(0).expect("can't get column FilePath");
        let label: String = row.get(1).expect("can't get column Label");
        Ok(Picture::new_with_label(&file_path, &label))
    }

    fn rusqlite_row_to_picture(row: &Row) -> SqlResult<Picture, rusqlite::Error> {
        println!("this is the row {:?}", row);
        let file_path: String = row.get(0).expect("can't get column FilePath");
        let label: String = row.get(1).expect("can't get column Label");
        let size: u64 = match row.get(2) {
            Ok(n) => n,
            Err(err) => {
                eprintln!("rusqlite error: {}", err);
                0
            },
        };
        let modified_time = row.get(3).expect("can't get column ModifiedTime");
        // todo let rank = row.get(4).expect("can't get column Rank");
        let sample_size = row.get(5).expect("can't get column SampleSize");
        // todo let sample = row.get(6).expect("can't get column Sample");
        let color_count: usize = row.get(7).expect("can't get column ColorCount");
        let cover = row.get(8).expect("can't get column Cover");
        let mut picture = Picture::new_with_label(&file_path, &label);
        let palette = Palette::new(vec![], sample_size);
        let image_data = ImageData {
            label: label,
            size: size,
            modified_time: modified_time,
            palette: palette,
            tags: HashSet::new(),
            cover: cover,
        };
        picture.set_image_data(image_data);
        Ok(picture)
    }

    pub fn file_path_as_stored(&self, file_path: &str) -> String {
        if let Some(home_dir) = &self.home_dir {
            let home_dir_str = home_dir.to_string();
            if file_path.starts_with(&home_dir_str) {
                let mut home_dir_iter = home_dir_str.chars();
                let mut file_path_iter = file_path.chars();
                while let Some(_) = home_dir_iter.next() {
                    file_path_iter.next();
                };
                "~".to_owned() + file_path_iter.as_str()
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
                let mut remaining = file_path.chars();
                remaining.next();
                let result = home_dir.to_string() + remaining.as_str();
                result
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
    use crate::model::image_data::timestamp;
    use crate::model::image_data::TimeStamp;
    use crate::env::default_values::TEST_DATABASE_FILE;
    use crate::test_data::*;
    use chrono::prelude::*;
    use chrono::naive::*;
    use crate::model::palette::Palette;
    use std::collections::HashSet;
    use std::time::SystemTime;
    use palette_extract::Color;
    use crate::file::picture_file::get_data_from_picture_file;

    pub fn my_db() -> Database {
        let database = Database::rusqlite_from_connection(TEST_DATABASE_FILE)
            .expect("test database can't be open");
        database
            .connection()
            .execute("DELETE FROM Picture;", [])
            .expect("db error");
        database.connection().execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/nine_colors.png', 'sample');",
            [],
        ).expect("db error");
        database
            .connection()
            .execute(
                "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/single_dot.png', '');",
                [],
            )
            .expect("db error");
        database.connection().execute(
            "INSERT INTO Picture (FilePath, Label) VALUES ('testdata/white_square.png', 'foo');",
            [],
        ).expect("db error");
        database.connection().execute(
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
        if let Some(home) = env::home_dir() {
            let this_file_path = home.display().to_string() + "/test_file"+ &home.display().to_string() +"/file.jpg";
            let expected = "~/test_file".to_owned() + &home.display().to_string() + "/file.jpg";
            assert_eq!(expected, database.file_path_as_stored(&this_file_path))
        }
    }

    #[test]
    fn file_path_not_starting_with_home_dir_are_not_tilded_as_stored() {
        let database = my_db();
        if let Some(home) = env::home_dir() {
            let this_file_path = "/other/".to_owned() + &home.display().to_string() + "/test_file.jpg";
            assert_eq!(this_file_path, database.file_path_as_stored(&this_file_path))
        }
    }

    #[test]
    fn file_path_starting_with_tilde_are_developped_as_retrieved() {
        let database = my_db();
        if let Some(home) = env::home_dir() {
            let this_file_path = "~/test_file/~/.jpg";
            let expected = home.display().to_string() + "/test_file/~/.jpg";
            assert_eq!(expected, database.file_path_as_retrieved(&this_file_path));
        }
    }

    #[test]
    fn file_path_not_starting_with_tilde_are_not_developped_as_retrieved() {
        let database = my_db();
        if let Some(home) = env::home_dir() {
            let this_file_path = "/other/~/test_file.jpg";
            assert_eq!(this_file_path, database.file_path_as_retrieved(&this_file_path));
        }
    }

    #[test]
    fn insert_and_retrieve_a_picture_with_image_data() {
        let database = my_db();
        let mut picture = Picture::new("testdata/some_pic.jpeg");
        let file_path = picture.file_path();
        let picture_file_data = get_data_from_picture_file(NINE_COLORS).expect("can't access to file data");
        let image_data = ImageData {
            label: "some_label".to_string(),
            size: picture_file_data.0,
            modified_time: picture_file_data.1,
            palette: Palette::new([
                Color { r: 4, g: 4, b: 4 },
                Color { r: 4, g: 4, b: 252 },
                Color { r: 4, g: 132, b: 132 },
                Color { r: 136, g: 100, b: 76 },
                Color { r: 156, g: 204, b: 52 },
                Color { r: 236, g: 132, b: 236 },
                Color { r: 252, g: 4, b: 4 },
                Color { r: 252, g: 140, b: 4 },
                Color { r: 252, g: 252, b: 4 }].to_vec(),
                100),
                cover: false,
                tags: HashSet::new(),
        };
        picture.set_image_data(image_data.clone());
        assert_eq!(100, picture.image_data().unwrap().palette().count());
        let initial: TimeStamp = picture_file_data.1;
        database.rusqlite_delete_picture_with_file_path(&file_path);
        assert_eq!( Ok(1), database.rusqlite_insert_picture(&picture));
        let result = database.rusqlite_retrieve_picture_with_file_path(&file_path);
        assert!(result.is_ok(), "could not retrieve picture in db");
        let retrieved_picture = result.unwrap();
        assert_eq!("testdata/some_pic.jpeg", retrieved_picture.file_path());
        assert_eq!("some_label", retrieved_picture.label());
        assert_eq!(49746, retrieved_picture.image_data().unwrap().size);
        let retrieved: TimeStamp =  retrieved_picture.image_data().unwrap().modified_time();
        assert_eq!(initial, retrieved);
        assert_eq!(100, retrieved_picture.image_data().unwrap().palette().count());
    }
}
