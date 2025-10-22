use crate::model::image_data::Tags;
use crate::model::rank::Rank;
use crate::model::palette::Palette;
use std::collections::HashSet;
use crate::model::image_data::ImageData;
use crate::model::picture::Picture;
use rusqlite::{Connection, Result as SqlResult, Row, params};
use std::collections::HashMap;
use std::io::Result as IOResult;
use std::rc::Rc;
use std::cell::{RefCell,Ref};
use crate::file::paths::{file_exists, home_directory};
use std::path::PathBuf;
use rusqlite::Error::InvalidPath;


pub type ImageDataMap = HashMap<String, ImageData>;

#[derive(Debug, Clone)]
pub struct Database {
    connection_rc: Rc<RefCell<Connection>>,
}

impl Database {
    pub fn from_connection(connection_string: &str, create: bool) -> std::io::Result<Self> {
        match Self::rusqlite_from_connection(connection_string, create) {
            Ok(database) => Ok(database),
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    fn rusqlite_from_connection(connection_string: &str, create: bool) -> SqlResult<Self> {
        if ! file_exists(connection_string) && ! create {
            return Err(InvalidPath(PathBuf::from(connection_string)))
        };
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

    pub fn rusqlite_create_schema(&self) -> SqlResult<usize> {
        let result = self.connection().execute(
            "CREATE TABLE IF NOT EXISTS Picture (      \n\
            FilePath TEXT NOT NULL PRIMARY KEY,        \n\
            Label TEXT NOT NULL,                       \n\
            FileSize INTEGER,                          \n\
            ModifiedTime INTEGER,                      \n\
            Rank INTEGER,                              \n\
            Sample BLOB,                               \n\
            ColorCount INTEGER,                        \n\
            Cover BOOLEAN);",params![])
            .and_then(|_|
                self.connection().execute(
                    "CREATE TABLE IF NOT EXISTS Tag (    \n\
                    FilePath TEXT NOT NULL,              \n\
                    Label TEXT NOT NULL,                \n\
                    PRIMARY KEY (FilePath, Label));",
                    params![])
                );
        result
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
        ).and_then(|_| {
            self.rusqlite_delete_tags(&picture.file_path())
                .and_then(|_| {
                    self.rusqlite_add_tags(&picture.file_path(), &image_data.tags)
                })
        })
    }

    fn rusqlite_delete_tags(&self, file_path: &str) -> SqlResult<usize> {
        self.connection().execute(
            "DELETE FROM Tag        \n\
            WHERE FilePath = ?1;",
            params![self.file_path_as_stored(file_path)]
        )
    }

    fn rusqlite_add_tags(&self, file_path: &str, tags: &Tags) -> SqlResult<usize> {
        let mut count: usize = 0;
        for label in tags.iter() {
            match self.connection().execute(
                "INSERT INTO Tag(          \n\
                 FilePath,                 \n\
                 Label)                    \n\
                 VALUES (?1, ?2);",
                 params![self.file_path_as_stored(file_path),
                 label,]) {
                Ok(n) => { count+= n; },
                Err(err) => {
                    eprintln!("{}", err);
                },
            }
        }
        Ok(count)
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

    #[cfg(test)]
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
        ).and_then(|mut picture| {
            let connection = self.connection();
            let mut statement = connection.prepare(
                "SELECT                   \n\
                Label                     \n\
                FROM Tag                  \n\
                WHERE FilePath = ?1;")?;
                let rows = statement.query_map(
                    params![self.file_path_as_stored(file_path)],
                    |row| Ok(row.get(0).expect("can't get column Label")))?;
                for tag in rows {
                    let label: String = tag.unwrap();
                    picture.add_tag(&label)
                }
                Ok(picture)
        })
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

    pub fn rusqlite_retrieve_all_tags(&self) -> SqlResult<HashMap<String, HashSet<String>>> {
        self.connection()
            .prepare(
                "SELECT                \n\
                FilePath,              \n\
                Label                  \n\
                FROM Tag;"
            ).and_then(|mut statement| {
                let mut map: HashMap<String, HashSet<String>> = HashMap::new();
                statement.query([]).and_then(|mut rows| {
                    while let Some(row) = rows.next().unwrap() {
                        let file_path: String = row.get(0).expect("can't access to column FilePath");
                        let file_path_as_retrieved = Self::file_path_as_retrieved(&file_path);
                        let label: String = row.get(1).expect("can't access to column Label");
                        if let Some(tags) = map.get_mut(&file_path_as_retrieved) {
                            let _ = tags.insert(label);
                        } else {
                            let mut tags = HashSet::new();
                            let _ = tags.insert(label);
                            map.insert(file_path_as_retrieved, tags);
                        };
                    };
                    Ok(map)
                })
            })
    }

    pub fn retrieve_all_pictures(&self) -> IOResult<Vec<Picture>> {
        match self.rusqlite_retrieve_all_pictures() {
            Ok(picture_map) => {
                match self.rusqlite_retrieve_all_tags() {
                    Ok(tag_map) => {
                        let mut pictures: Vec<Picture> = vec![];
                        for (file_path, image_data) in picture_map.iter() {
                            let new_tags = if let Some(tags) = tag_map.get(file_path) {
                                tags.clone()
                            } else {
                                HashSet::new()
                            };
                            let new_image_data = ImageData {
                                tags: new_tags,
                                .. image_data.clone()
                            };
                            let picture = Picture::new_with_image_data(file_path, &new_image_data);
                            pictures.push(picture)
                        };
                        pictures.sort_by_key(|picture| picture.file_path());
                        Ok(pictures)
                    },
                    Err(err) => Err(std::io::Error::other(err)),
                }
            },
            Err(err) => Err(std::io::Error::other(err)),
        }
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
            label,
            size,
            modified_time,
            rank: Rank::from(rank_value),
            palette,
            tags: HashSet::new(),
            cover,
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
// ""
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
    use std::env;

    pub fn my_db() -> Database {
        let database = Database::rusqlite_from_connection(TEST_DATABASE_FILE, false)
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
        let picture_file_data = get_data_from_picture_file(&nine_colors_file_path()).expect("can't access to file data");
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
        let mut picture = database.rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path()).unwrap();
        let old_picture = picture.clone();
        let mut image_data = picture.image_data().expect("can't access picture image data");
        image_data.set_rank(Rank::TwoStars);
        picture.set_image_data(image_data);
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let new_picture = database.rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path()).unwrap();
        assert_eq!(Rank::TwoStars, new_picture.image_data().expect("can't access image data").rank());
        database.rusqlite_update_picture(&old_picture);
   }

    #[test]
    fn add_a_tag_to_a_picture_image_data() {
        let database = my_db();
        let mut picture = database.rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path()).unwrap();
        let old_picture = picture.clone();
        let mut image_data = picture.image_data().expect("can't access picture image data");
        picture.add_tag("foo");
        picture.add_tag("bar");
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let new_picture = database.rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path()).unwrap();
        assert!(new_picture.image_data().unwrap().tags.contains("foo"));
        assert!(new_picture.image_data().unwrap().tags.contains("bar"));
        database.rusqlite_update_picture(&old_picture);
    }

    #[test]
    fn find_all_the_tags_in_the_database() {
        let database = my_db();
        let mut picture = database.rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path()).unwrap();
        let mut image_data = picture.image_data().expect("can't access picture image data");
        picture.add_tag("foo");
        picture.add_tag("bar");
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let mut picture = database.rusqlite_retrieve_picture_with_file_path(&single_dot_file_path()).unwrap();
        let mut image_data = picture.image_data().expect("can't access picture image data");
        picture.add_tag("dot");
        picture.add_tag("bar");
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let mut picture = database.rusqlite_retrieve_picture_with_file_path(&white_square_file_path()).unwrap();
        let mut image_data = picture.image_data().expect("can't access picture image data");
        picture.add_tag("qux");
        picture.add_tag("foo");
        assert!(database.rusqlite_update_picture(&picture).is_ok());

        let mut result = database.rusqlite_retrieve_all_tags();
        assert!(result.is_ok());
        let map = result.unwrap();
        let mut file_path = nine_colors_file_path();
        assert!(map.get(&file_path).unwrap().contains("foo"));
        assert!(map.get(&file_path).unwrap().contains("bar"));
        let mut file_path = white_square_file_path();
        assert!(map.get(&file_path).unwrap().contains("qux"));
        assert!(map.get(&file_path).unwrap().contains("foo"));
        let mut file_path = single_dot_file_path();
        assert!(map.get(&file_path).unwrap().contains("dot"));
        assert!(map.get(&file_path).unwrap().contains("bar"));

        let result = database.retrieve_all_pictures();
        assert!(result.is_ok());
        let pictures = result.unwrap();
        assert_eq!(nine_colors_file_path(), pictures[1].file_path());
        assert!(pictures[1].image_data().unwrap().tags.contains("foo"));
        assert!(pictures[1].image_data().unwrap().tags.contains("bar"));
        assert_eq!(single_dot_file_path(), pictures[2].file_path());
        assert!(pictures[2].image_data().unwrap().tags.contains("dot"));
        assert!(pictures[2].image_data().unwrap().tags.contains("bar"));
        assert_eq!(white_square_file_path(), pictures[3].file_path());
        assert!(pictures[3].image_data().unwrap().tags.contains("qux"));
        assert!(pictures[3].image_data().unwrap().tags.contains("foo"));
    }

}
