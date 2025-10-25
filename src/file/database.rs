use crate::cli::args::Args;
use crate::file::paths::{
    file_exists, file_path_as_retrieved, file_path_as_stored, home_directory,
};
use crate::model::image_data::ImageData;
use crate::model::palette::Palette;
use crate::model::picture::Picture;
use crate::model::rank::Rank;
use crate::model::selection::Selection;
use crate::model::tags::Tags;
use rusqlite::Error::InvalidPath;
use rusqlite::{Connection, Result as SqlResult, Row, params};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Result as IOResult;
use std::path::PathBuf;
use std::rc::Rc;

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
        if !file_exists(connection_string) && !create {
            return Err(InvalidPath(PathBuf::from(connection_string)));
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
        let result = self
            .connection()
            .execute(
                "CREATE TABLE IF NOT EXISTS Picture (      \n\
            FilePath TEXT NOT NULL PRIMARY KEY,        \n\
            Label TEXT NOT NULL,                       \n\
            FileSize INTEGER,                          \n\
            ModifiedTime INTEGER,                      \n\
            Rank INTEGER,                              \n\
            Sample BLOB,                               \n\
            ColorCount INTEGER,                        \n\
            Cover BOOLEAN);",
                params![],
            )
            .and_then(|_| {
                self.connection().execute(
                    "CREATE TABLE IF NOT EXISTS Tag (    \n\
                    FilePath TEXT NOT NULL,              \n\
                    Label TEXT NOT NULL,                \n\
                    PRIMARY KEY (FilePath, Label));",
                    params![],
                )
            });
        result
    }

    pub fn rusqlite_insert_picture(&self, picture: &Picture) -> SqlResult<usize> {
        let image_data = match picture.image_data() {
            Some(data) => data,
            None => ImageData::new(""),
        };
        self.connection()
            .execute(
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
                    file_path_as_stored(&picture.file_path()),
                    image_data.label(),
                    image_data.size(),
                    image_data.modified_time(),
                    <Rank as Into<i64>>::into(image_data.rank()),
                    image_data.palette().sample_as_array(),
                    image_data.palette.count(),
                    image_data.cover(),
                ],
            )
            .and_then(|count| {
                let mut tag_count = 0;
                for tag in image_data.tags() {
                    match self.connection().execute(
                        "INSERT INTO Tag(         \n\
                    FilePath,                 \n\
                    Label)                    \n\
                    VALUES (?1, ?2);",
                        params![file_path_as_stored(&picture.file_path()), tag],
                    ) {
                        Ok(n) => tag_count += n,
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
                Ok(count + tag_count)
            })
    }

    pub fn rusqlite_update_picture(&self, picture: &Picture) -> SqlResult<usize> {
        let image_data = match picture.image_data() {
            Some(data) => data,
            None => ImageData::new(""),
        };
        self.connection()
            .execute(
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
                    file_path_as_stored(&picture.file_path()),
                    image_data.label(),
                    image_data.size(),
                    image_data.modified_time(),
                    <Rank as Into<i64>>::into(image_data.rank()),
                    image_data.palette().sample_as_array(),
                    image_data.palette.count(),
                    image_data.cover
                ],
            )
            .and_then(|_| {
                self.rusqlite_delete_tags(&picture.file_path())
                    .and_then(|_| self.rusqlite_add_tags(&picture.file_path(), &image_data.tags))
            })
    }

    fn rusqlite_delete_tags(&self, file_path: &str) -> SqlResult<usize> {
        self.connection().execute(
            "DELETE FROM Tag        \n\
            WHERE FilePath = ?1;",
            params![file_path_as_stored(file_path)],
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
                params![file_path_as_stored(file_path), label,],
            ) {
                Ok(n) => {
                    count += n;
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        Ok(count)
    }

    pub fn rusqlite_delete_picture_with_file_path(&self, file_path: &str) -> SqlResult<usize> {
        self.connection()
            .execute(
                "DELETE FROM Picture        \n\
            WHERE FilePath = ?1;",
                params![file_path_as_stored(file_path)],
            )
            .and_then(|count| {
                self.connection().execute(
                    "DELETE FROM Tag        \n\
            WHERE FilePath = ?1;",
                    params![file_path_as_stored(file_path)],
                )
            })
    }

    pub fn rusqlite_check_picture_with_file_path(&self, file_path: &str) -> SqlResult<String> {
        self.connection().query_one(
            "SELECT                     \n\
             FilePath                   \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;",
            params![file_path],
            |row| row.get(0),
        )
    }

    #[cfg(test)]
    pub fn rusqlite_retrieve_picture_with_file_path(&self, file_path: &str) -> SqlResult<Picture> {
        self.connection()
            .query_row(
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
                params![file_path_as_stored(file_path)],
                Self::rusqlite_row_to_picture,
            )
            .and_then(|mut picture| {
                let connection = self.connection();
                let mut statement = connection.prepare(
                    "SELECT                   \n\
                Label                     \n\
                FROM Tag                  \n\
                WHERE FilePath = ?1;",
                )?;
                let rows = statement.query_map(params![file_path_as_stored(file_path)], |row| {
                    Ok(row.get(0).expect("can't get column Label"))
                })?;
                for tag in rows {
                    let label: String = tag.unwrap();
                    picture.add_tag(&label)
                }
                Ok(picture)
            })
    }

    const SELECT_STAR_FROM_PICTURE: &str = "SELECT                     \n\
             FilePath,                  \n\
             Label,                     \n\
             FileSize,                  \n\
             ModifiedTime,              \n\
             Rank,                      \n\
             Sample,                    \n\
             ColorCount,                \n\
             Cover                      \n\
             FROM Picture              \n";

    const WHERE_COVER: &str = "WHERE Cover = true \n";

    const ORDER_FILE_PATH: &str = "ORDER BY FilePath \n";

    // select * from picture where concat(substring(filepath,1,23), substring(filepath,24)) = filepath ;
    fn select_parent_dir(parent_dir: String) -> String {
        let parent = file_path_as_stored(&parent_dir);
        let file_name_start = parent.len() + 2; // to account for /
        format!(
            "FilePath like '{}%' AND Instr(Substring(FilePath, {}), '/') = 0",
            parent, file_name_start
        )
    }

    pub fn rusqlite_retrieve_all_pictures(
        &self,
        cover: bool,
        parent_opt: Option<String>,
    ) -> SqlResult<ImageDataMap> {
        let sql_query = format!(
            "{} WHERE true AND {} AND {} ORDER BY FilePath",
            Self::SELECT_STAR_FROM_PICTURE,
            if cover { "Cover = true" } else { "true" },
            if let Some(parent) = parent_opt {
                &Self::select_parent_dir(parent)
            } else {
                "true"
            }
        );
        self.connection()
            .prepare(&sql_query)
            .and_then(|mut statement| {
                let mut map: ImageDataMap = HashMap::new();
                statement.query([]).and_then(|mut rows| {
                    while let Some(row) = rows.next().unwrap() {
                        match Self::rusqlite_row_to_picture(row) {
                            Ok(picture) => {
                                let _ = map.insert(
                                    file_path_as_retrieved(&picture.file_path()),
                                    picture.image_data().unwrap(),
                                );
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

    pub fn rusqlite_retrieve_all_tags(&self) -> SqlResult<HashMap<String, HashSet<String>>> {
        self.connection()
            .prepare(
                "SELECT                \n\
                FilePath,              \n\
                Label                  \n\
                FROM Tag;",
            )
            .and_then(|mut statement| {
                let mut map: HashMap<String, HashSet<String>> = HashMap::new();
                statement.query([]).and_then(|mut rows| {
                    while let Some(row) = rows.next().unwrap() {
                        let file_path: String =
                            row.get(0).expect("can't access to column FilePath");
                        let file_path_as_retrieved = file_path_as_retrieved(&file_path);
                        let label: String = row.get(1).expect("can't access to column Label");
                        if let Some(tags) = map.get_mut(&file_path_as_retrieved) {
                            let _ = tags.insert(label);
                        } else {
                            let mut tags = HashSet::new();
                            let _ = tags.insert(label);
                            map.insert(file_path_as_retrieved, tags);
                        };
                    }
                    Ok(map)
                })
            })
    }

    pub fn retrieve_all_pictures(
        &self,
        selection: Selection,
        cover: bool,
        parent_opt: Option<String>,
    ) -> IOResult<Vec<Picture>> {
        match self.rusqlite_retrieve_all_pictures(cover, parent_opt) {
            Ok(picture_map) => match self.rusqlite_retrieve_all_tags() {
                Ok(tag_map) => {
                    let mut pictures: Vec<Picture> = vec![];
                    for (file_path, image_data) in picture_map.iter() {
                        let new_tags = if let Some(tags) = tag_map.get(file_path) {
                            tags.clone()
                        } else {
                            HashSet::new()
                        };
                        let new_image_data = ImageData {
                            tags: new_tags.clone(),
                            ..image_data.clone()
                        };
                        if selection.is_empty() || selection.matches(new_tags.clone()) {
                            let picture = Picture::new_with_image_data(file_path, &new_image_data);
                            pictures.push(picture)
                        }
                    }
                    pictures.sort_by_key(|picture| picture.file_path());
                    Ok(pictures)
                }
                Err(err) => Err(std::io::Error::other(err)),
            },
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    pub fn retrieve_all_pictures_with_parent(&self, parent_dir: &str) -> IOResult<Vec<Picture>> {
        self.retrieve_all_pictures(Selection::empty(), false, Some(parent_dir.to_string()))
    }

    fn rusqlite_row_to_picture(row: &Row) -> SqlResult<Picture, rusqlite::Error> {
        let file_path: String = row.get(0).expect("can't get column FilePath");
        let file_path_as_retrieved = file_path_as_retrieved(&file_path);
        let label: String = row.get(1).expect("can't get column Label");
        let size: u64 = match row.get(2) {
            Ok(n) => n,
            Err(err) => {
                eprintln!("rusqlite error: {}", err);
                0
            }
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
}
// ""
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::env::default_values::TEST_DATABASE_FILE;
    use crate::file::paths::current_directory;
    use crate::file::picture_file::get_data_from_picture_file;
    use crate::get_configuration;
    use crate::model::image_data::TimeStamp;
    use crate::model::image_data::timestamp;
    use crate::model::palette::Palette;
    use crate::test_data::*;
    use chrono::naive::*;
    use chrono::prelude::*;
    use palette_extract::Color;
    use std::collections::HashSet;
    use std::env;
    use std::time::SystemTime;

    pub fn my_db() -> Database {
        let database = Database::rusqlite_from_connection(TEST_DATABASE_FILE, false)
            .expect("test database can't be open");
        database
    }

    pub fn my_args() -> Args {
        let cmd: Option<Vec<&str>> = None;
        let config = get_configuration().unwrap();
        Args::parse_and_check(cmd, &config).unwrap()
    }
    #[test]
    fn retrieve_all_pictures_ordered_by_file_path() {
        let database = my_db();
        let status: SqlResult<ImageDataMap> = database.rusqlite_retrieve_all_pictures(false, None);
        assert!(status.is_ok());
        let map = status.unwrap();
        assert_eq!(4, map.len());
    }

    #[test]
    fn insert_and_retrieve_a_picture_with_image_data() {
        let database = my_db();
        let mut picture = Picture::new("testdata/some_pic.jpeg");
        let file_path = picture.file_path();
        let picture_file_data = get_data_from_picture_file(&nine_colors_file_path())
            .expect("can't access to file data");
        let image_data = ImageData {
            label: "some_label".to_string(),
            size: picture_file_data.0,
            modified_time: picture_file_data.1,
            rank: Rank::ThreeStars,
            palette: Palette::new(
                [
                    Color { r: 4, g: 4, b: 4 },
                    Color { r: 4, g: 4, b: 252 },
                    Color {
                        r: 4,
                        g: 132,
                        b: 132,
                    },
                    Color {
                        r: 136,
                        g: 100,
                        b: 76,
                    },
                    Color {
                        r: 156,
                        g: 204,
                        b: 52,
                    },
                    Color {
                        r: 236,
                        g: 132,
                        b: 236,
                    },
                    Color { r: 252, g: 4, b: 4 },
                    Color {
                        r: 252,
                        g: 140,
                        b: 4,
                    },
                    Color {
                        r: 252,
                        g: 252,
                        b: 4,
                    },
                ]
                .to_vec(),
                100,
            ),
            cover: true,
            tags: HashSet::from([String::from("foo"), String::from("bar")]),
        };
        picture.set_image_data(image_data.clone());
        assert_eq!(100, picture.image_data().unwrap().palette().count());
        let initial: TimeStamp = picture_file_data.1;
        database.rusqlite_delete_picture_with_file_path(&file_path);

        assert!(database.rusqlite_insert_picture(&picture).is_ok());
        let result = database.rusqlite_retrieve_picture_with_file_path(&file_path);
        database.rusqlite_delete_picture_with_file_path(&file_path);
        assert!(result.is_ok(), "could not retrieve picture in db");
        let retrieved_picture = result.unwrap();
        assert_eq!("testdata/some_pic.jpeg", retrieved_picture.file_path());
        assert_eq!("some_label", retrieved_picture.label());
        assert_eq!(49746, retrieved_picture.image_data().unwrap().size);
        let retrieved: TimeStamp = retrieved_picture.image_data().unwrap().modified_time();
        assert_eq!(initial, retrieved);
        assert_eq!(
            100,
            retrieved_picture.image_data().unwrap().palette().count()
        );
        assert_eq!(true, retrieved_picture.image_data().unwrap().cover());
        assert_eq!(2, retrieved_picture.image_data().unwrap().tags.len());
    }

    #[test]
    fn update_a_picture_image_data() {
        let database = my_db();
        let mut picture = database
            .rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path())
            .unwrap();
        let old_picture = picture.clone();
        let mut image_data = picture
            .image_data()
            .expect("can't access picture image data");
        image_data.rank = Rank::TwoStars;
        picture.set_image_data(image_data);
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let new_picture = database
            .rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path())
            .unwrap();
        assert_eq!(
            Rank::TwoStars,
            new_picture
                .image_data()
                .expect("can't access image data")
                .rank()
        );
        database.rusqlite_update_picture(&old_picture);
    }

    #[test]
    fn add_a_tag_to_a_picture_image_data() {
        let database = my_db();
        let mut picture = database
            .rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path())
            .unwrap();
        let old_picture = picture.clone();
        let mut image_data = picture
            .image_data()
            .expect("can't access picture image data");
        picture.add_tag("foo");
        picture.add_tag("bar");
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let new_picture = database
            .rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path())
            .unwrap();
        assert!(new_picture.image_data().unwrap().tags.contains("foo"));
        assert!(new_picture.image_data().unwrap().tags.contains("bar"));
        database.rusqlite_update_picture(&old_picture);
    }

    #[test]
    fn find_all_the_tags_in_the_database() {
        let database = my_db();
        let mut picture = database
            .rusqlite_retrieve_picture_with_file_path(&nine_colors_file_path())
            .unwrap();
        let mut image_data = picture
            .image_data()
            .expect("can't access picture image data");
        picture.add_tag("foo");
        picture.add_tag("bar");
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let mut picture = database
            .rusqlite_retrieve_picture_with_file_path(&single_dot_file_path())
            .unwrap();
        let mut image_data = picture
            .image_data()
            .expect("can't access picture image data");
        picture.add_tag("dot");
        picture.add_tag("bar");
        assert!(database.rusqlite_update_picture(&picture).is_ok());
        let mut picture = database
            .rusqlite_retrieve_picture_with_file_path(&white_square_file_path())
            .unwrap();
        let mut image_data = picture
            .image_data()
            .expect("can't access picture image data");
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

        let result = database.retrieve_all_pictures(Selection::empty(), false, None);
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

    #[test]
    fn finding_all_pictures_with_file_path_having_a_parent_directory() {
        let database = my_db();
        let args = my_args();
        let parent_dir = format!("{}/testdata", current_directory());
        let result = database.retrieve_all_pictures_with_parent(&parent_dir);
        assert!(result.is_ok());
        let pictures = result.unwrap();
        assert_eq!(4, pictures.len());
    }
    #[test]
    fn find_no_picture_for_a_parent_directory_where_no_picture_exists() {
        let database = my_db();
        let args = my_args();
        let parent_dir = format!("{}", current_directory());
        let result = database.retrieve_all_pictures_with_parent(&parent_dir);
        assert!(result.is_ok());
        let pictures = result.unwrap();
        assert_eq!(0, pictures.len());
    }
}
