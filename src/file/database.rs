use crate::model::rank::Rank;
use std::env::current_dir;
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
use crate::file::paths::home_directory;



pub type ImageDataMap = HashMap<String, ImageData>;

#[derive(Debug, Clone)]
pub struct Database {
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
        self.connection().execute(
            "INSERT INTO Picture (        \n\
             FilePath,                    \n\
             Label,                       \n\
             FileSize,                    \n\
             ModifiedTime,                \n\
             Rank,                        \n\
             Sample,                      \n\
             ColorCount,                  \n\
             Cover)                       \n\
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
             params![
             self.file_path_as_stored(&picture.file_path()),
             image_data.label(),
             image_data.size(),
             image_data.modified_time(),
             <Rank as Into<i64>>::into(image_data.rank()),
             image_data.palette().sample_as_array(),
             image_data.palette.count(),
             false],
        )
    }

    pub fn rusqlite_update_picture(&self, picture: &Picture) -> SqlResult<usize> {
        let image_data = match picture.image_data() {
            Some(data) => data,
            None => ImageData::new(""),
        };
        self.connection().execute(
            "UPDATE Picture               \n\
             SET                          \n\
             Label = ?2,                  \n\
             FileSize = ?3,               \n\
             ModifiedTime = ?4,           \n\
             Rank = ?5,                   \n\
             Sample = ?6,                 \n\
             ColorCount =?7,              \n\
             Cover = ?8                   \n\
               WHERE FilePath = ?1;",
            params![
             self.file_path_as_stored(&picture.file_path()),
             image_data.label(),
             image_data.size(),
             image_data.modified_time(),
             <Rank as Into<i64>>::into(image_data.rank()),
             image_data.palette().sample_as_array(),
             image_data.palette.count(),
             false],
        )
    }

    pub fn rusqlite_delete_picture_with_file_path(&self, file_path: &str) -> SqlResult<usize> {
        self.connection().execute(
            "DELETE FROM Picture        \n\
            WHERE FilePath = ?1;",
            params![file_path.to_string()],
        )
    }

    pub fn rusqlite_check_picture_with_file_path(&self, file_path: &str) -> SqlResult<String> {
        self.connection().query_one(
            "SELECT                     \n\
             FilePath                   \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;",
            params![file_path],
            |row| row.get(0)
        )
    }

    pub fn rusqlite_retrieve_picture_with_file_path(&self, file_path: &str) -> SqlResult<Picture> {
        self.connection().query_row(
            "SELECT                     \n\
             FilePath,                  \n\
             Label,                     \n\
             FileSize,                  \n\
             ModifiedTime,              \n\
             Rank,                      \n\
             Sample,                    \n\
             ColorCount,                \n\
             Cover                      \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;",
            params![self.file_path_as_stored(file_path)],
            Self::rusqlite_row_to_picture,
        )
    }

    pub fn rusqlite_retrieve_all_pictures(&self) -> SqlResult<ImageDataMap> {
        self.connection()
            .prepare(
            "SELECT                     \n\
             FilePath,                  \n\
             Label,                     \n\
             FileSize,                  \n\
             ModifiedTime,              \n\
             Rank,                      \n\
             Sample,                    \n\
             ColorCount,                \n\
             Cover                      \n\
             FROM Picture ORDER BY FilePath;",
            )
            .and_then(|mut statement| {
                let mut map: ImageDataMap = HashMap::new();
                statement.query([]).and_then(|mut rows| {
                    while let Some(row) = rows.next().unwrap() {
                        match Self::rusqlite_row_to_picture(row) {
                            Ok(picture) => {
                                let _ =
                                    map.insert(
                                        Self::file_path_as_retrieved(&picture.file_path()), picture.image_data().unwrap());
                            }
                            Err(err) => {
                                eprintln!("{}", err);
                                return Err(err);
                            }
                        }
                    };
                    Ok(map)
                })
            })
    }

    pub fn retrieve_all_pictures(&self) -> IOResult<Vec<Picture>> {
        match self.rusqlite_retrieve_all_pictures() {
            Ok(map) => {
                let mut pictures: Vec<Picture> = vec![];
                for (file_path, image_data) in map.iter() {
                    let picture = Picture::new_with_image_data(file_path, &image_data);
                    pictures.push(picture)
                };
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
        let file_path: String = row.get(0).expect("can't get column FilePath");
        let file_path_as_retrieved = Self::file_path_as_retrieved(&file_path);
        let label: String = row.get(1).expect("can't get column Label");
        let size: u64 = match row.get(2) {
            Ok(n) => n,
            Err(err) => {
                eprintln!("rusqlite error: {}", err);
                0
            },
        };
        let modified_time = row.get(3).expect("can't get column ModifiedTime");
        let rank_value: i64 = row.get(4).expect("can't get column Rank");
        let sample_array = row.get(5).expect("can't get column Sample");
        let color_count: usize = row.get(6).expect("can't get column ColorCount");
        let cover = row.get(7).expect("can't get column Cover");
        let mut picture = Picture::new_with_label(&file_path_as_retrieved, &label);
        let mut palette = Palette::new(vec![], color_count);
        palette.set_sample_from_array(sample_array);
        let image_data = ImageData {
            label: label,
            size: size,
            modified_time: modified_time,
            rank: Rank::from(rank_value),
            palette: palette,
            tags: HashSet::new(),
            cover: cover,
        };
        picture.set_image_data(image_data);
        Ok(picture)
    }

    pub fn file_path_as_stored(&self, file_path: &str) -> String {
        let home_dir_str = home_directory();
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
    }

    pub fn file_path_as_retrieved(file_path: &str) -> String {
        if file_path.starts_with("~") {
            let mut remaining = file_path.chars();
            remaining.next();
            let result = home_directory() + remaining.as_str();
            result
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
    use crate::file::paths::current_directory;

    pub fn my_db() -> Database {
        let database = Database::rusqlite_from_connection(TEST_DATABASE_FILE)
            .expect("test database can't be open");
        database
    }

    #[test]
    fn retrieve_all_pictures_ordered_by_file_path() {
        let database = my_db();
        let status: SqlResult<ImageDataMap> = database.rusqlite_retrieve_all_pictures();
        assert!(status.is_ok());
        let map = status.unwrap();
        assert_eq!(4, map.len());
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
            assert_eq!(expected, Database::file_path_as_retrieved(&this_file_path));
        }
    }

    #[test]
    fn file_path_not_starting_with_tilde_are_not_developped_as_retrieved() {
        let database = my_db();
        if let Some(home) = env::home_dir() {
            let this_file_path = "/other/~/test_file.jpg";
            assert_eq!(this_file_path, Database::file_path_as_retrieved(&this_file_path));
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
            rank: Rank::ThreeStars,
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
        database.rusqlite_delete_picture_with_file_path(&file_path);
        assert!(result.is_ok(), "could not retrieve picture in db");
        let retrieved_picture = result.unwrap();
        assert_eq!("testdata/some_pic.jpeg", retrieved_picture.file_path());
        assert_eq!("some_label", retrieved_picture.label());
        assert_eq!(49746, retrieved_picture.image_data().unwrap().size);
        let retrieved: TimeStamp =  retrieved_picture.image_data().unwrap().modified_time();
        assert_eq!(initial, retrieved);
        assert_eq!(100, retrieved_picture.image_data().unwrap().palette().count());
    }

    #[test]
    fn update_a_picture_image_data() {
        let database = my_db();
        let file_path = current_directory() + "/" + NINE_COLORS;
        let mut picture = database.rusqlite_retrieve_picture_with_file_path(&file_path).expect(&format!("can't retrieve picture: {}", file_path));
        let old_picture = picture.clone();
        let mut image_data = picture.image_data().expect("can't access picture image data");
        image_data.set_rank(Rank::TwoStars);
        picture.set_image_data(image_data);
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let new_picture = database.rusqlite_retrieve_picture_with_file_path(&file_path).expect(&format!("can't retrieve updated picture: {}", file_path));
        assert_eq!(Rank::TwoStars, new_picture.image_data().expect("can't access image data").rank());
        database.rusqlite_update_picture(&picture);
    }
}
