use crate::file::paths::parent_directory;
use crate::file::paths::{file_exists, file_path_as_retrieved, file_path_as_stored};
use crate::model::color_range::ColorRange;
use crate::model::cover::{bool_to_cover, cover_to_bool};
use crate::model::image_data::ImageData;
use crate::model::palette::Palette;
use crate::model::picture::Picture;
use crate::model::rank::Rank;
use crate::model::selection_criteria::SelectionCriteria;
use crate::model::tags::Tags;
use regex::Regex;
use rusqlite::Error::InvalidPath;
use rusqlite::{Connection, Result as SqlResult, Row, params};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IOError;
use std::io::Result as IOResult;
use std::path::PathBuf;
use std::rc::Rc;

pub type ImageDataMap = HashMap<String, ImageData>;

#[derive(Debug, Clone)]
pub struct Database {
    connection_rc: Rc<RefCell<Connection>>,
}

pub struct RetrieveCriteria {
    pub selection_criteria: SelectionCriteria,
    pub label: Option<String>,
    pub extraction: Option<Vec<String>>,
    pub filter: Option<String>,
    pub pattern: Option<Regex>,
    pub cover: bool,
    pub parent_opt: Option<String>,
}

impl Database {
    pub fn from_connection(connection_string: &str, create: bool) -> IOResult<Self> {
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

    pub fn rusqlite_create_schema(&self) -> SqlResult<usize> {
        let connection = self.connection_rc.borrow();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS Picture (      \n\
            FilePath TEXT NOT NULL PRIMARY KEY,        \n\
            Label TEXT NOT NULL,                       \n\
            FileSize INTEGER,                          \n\
            ModifiedTime INTEGER,                      \n\
            Rank INTEGER,                              \n\
            Sample BLOB,                               \n\
            ColorCount INTEGER,                        \n\
            Cover BOOLEAN,
            Score INTEGER NOT NULL DEFAULT 0);", // ""
                params![],
            )
            .and_then(|_| {
                connection.execute(
                    "CREATE TABLE IF NOT EXISTS Tag (    \n\
                    FilePath TEXT NOT NULL,              \n\
                    Label TEXT NOT NULL,                \n\
                    PRIMARY KEY (FilePath, Label));", // ""
                    params![],
                )
            })
    }

    fn rusqlite_insert_picture(&self, picture: &Picture) -> SqlResult<usize> {
        let connection = self.connection_rc.borrow();
        let image_data = match picture.image_data() {
            Some(data) => data,
            None => ImageData::default(),
        };
        connection
            .execute(
                "INSERT INTO Picture (    \n\
             FilePath,                    \n\
             Label,                       \n\
             FileSize,                    \n\
             ModifiedTime,                \n\
             Rank,                        \n\
             Sample,                      \n\
             ColorCount,                  \n\
             Cover,                       \n\
             Score)                       \n\
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);", // ""
                params![
                    file_path_as_stored(&picture.file_path()),
                    image_data.label(),
                    image_data.size(),
                    image_data.modified_time(),
                    <Rank as Into<i64>>::into(image_data.rank()),
                    image_data.palette().sample_as_array(),
                    image_data.palette.count(),
                    cover_to_bool(image_data.cover()),
                    image_data.score,
                ],
            )
            .map(|count| {
                let mut tag_count = 0;
                for tag in image_data.tags() {
                    match connection.execute(
                        "INSERT INTO Tag(         \n\
                         FilePath,                \n\
                         Label)                   \n\
                         VALUES (?1, ?2);", // ""
                        params![file_path_as_stored(&picture.file_path()), tag],
                    ) {
                        Ok(n) => tag_count += n,
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
                count + tag_count
            })
    }

    fn rusqlite_update_picture(&self, picture: &Picture) -> SqlResult<usize> {
        let connection = self.connection_rc.borrow();
        let image_data = match picture.image_data() {
            Some(data) => data,
            None => ImageData::default(),
        };
        connection
            .execute(
                "UPDATE Picture               \n\
             SET                          \n\
             Label = ?2,                  \n\
             FileSize = ?3,               \n\
             ModifiedTime = ?4,           \n\
             Rank = ?5,                   \n\
             Sample = ?6,                 \n\
             ColorCount =?7,              \n\
             Cover = ?8,                  \n\
             Score = ?9                   \n\
               WHERE FilePath = ?1;", // ""
                params![
                    file_path_as_stored(&picture.file_path()),
                    image_data.label(),
                    image_data.size(),
                    image_data.modified_time(),
                    <Rank as Into<i64>>::into(image_data.rank()),
                    image_data.palette().sample_as_array(),
                    image_data.palette.count(),
                    cover_to_bool(image_data.cover),
                    image_data.score,
                ],
            )
            .and_then(|_| {
                self.rusqlite_delete_tags(&picture.file_path())
                    .and_then(|_| self.rusqlite_add_tags(&picture.file_path(), &image_data.tags))
            })
    }

    fn rusqlite_delete_tags(&self, file_path: &str) -> SqlResult<usize> {
        let connection = self.connection_rc.borrow();
        connection.execute(
            "DELETE FROM Tag        \n\
            WHERE FilePath = ?1;",
            params![file_path_as_stored(file_path)],
        )
    }

    fn rusqlite_add_tags(&self, file_path: &str, tags: &Tags) -> SqlResult<usize> {
        let mut count: usize = 0;
        for label in tags.iter() {
            let connection = self.connection_rc.borrow();
            match connection.execute(
                "INSERT INTO Tag(          \n\
                 FilePath,                 \n\
                 Label)                    \n\
                 VALUES (?1, ?2);", // ""
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

    fn rusqlite_delete_picture_with_file_path(&self, file_path: &str) -> SqlResult<usize> {
        let connection = self.connection_rc.borrow();
        connection
            .execute(
                "DELETE FROM Picture        \n\
            WHERE FilePath = ?1;", // ""
                params![file_path_as_stored(file_path)],
            )
            .and_then(|_| {
                connection.execute(
                    "DELETE FROM Tag        \n\
            WHERE FilePath = ?1;", // ""
                    params![file_path_as_stored(file_path)],
                )
            })
    }

    pub fn delete_picture_with_file_path(&self, file_path: &str) -> IOResult<usize> {
        println!("DELETE {}", file_path_as_stored(file_path));
        match self.rusqlite_delete_picture_with_file_path(file_path) {
            Ok(n) => Ok(n),
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    pub fn rusqlite_check_picture_with_file_path(&self, file_path: &str) -> SqlResult<String> {
        let connection = self.connection_rc.borrow();
        connection.query_one(
            "SELECT                     \n\
             FilePath                   \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;", // ""
            params![&file_path_as_stored(file_path)],
            |row| row.get(0),
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
                Self::select_parent_dir(parent)
            } else {
                "true".to_string()
            }
        );
        let connection = self.connection_rc.borrow();
        connection.prepare(&sql_query).and_then(|mut statement| {
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

    pub fn rusqlite_retrieve_all_labels(&self) -> SqlResult<HashSet<String>> {
        let connection = self.connection_rc.borrow();
        connection
            .prepare(
                "SELECT DISTINCT Label         \n\
                FROM Picture WHERE Label <> '' \n\
                UNION                          \n\
                SELECT DISTINCT Label          \n\
                FROM Tag WHERE Label <> '';", // ""
            )
            .and_then(|mut statement| {
                let mut map: HashSet<String> = HashSet::new();
                statement.query([]).map(|mut rows| {
                    while let Some(row) = rows.next().unwrap() {
                        let label: String = row.get(0).expect("can't access to column Label");
                        let _ = map.insert(label);
                    }
                    map
                })
            })
    }

    pub fn rusqlite_retrieve_all_tags(&self) -> SqlResult<HashMap<String, HashSet<String>>> {
        let connection = self.connection_rc.borrow();
        connection
            .prepare(
                "SELECT                \n\
                FilePath,              \n\
                Label                  \n\
                FROM Tag;",
            )
            .and_then(|mut statement| {
                let mut map: HashMap<String, HashSet<String>> = HashMap::new();
                statement.query([]).map(|mut rows| {
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
                    map
                })
            })
    }

    fn rusqlite_retrieve_picture_with_file_path(&self, file_path: &str) -> SqlResult<Picture> {
        let connection = self.connection_rc.borrow();
        connection
            .query_row(
                "SELECT                     \n\
             FilePath,                  \n\
             Label,                     \n\
             FileSize,                  \n\
             ModifiedTime,              \n\
             Rank,                      \n\
             Sample,                    \n\
             ColorCount,                \n\
             Cover,                     \n\
             Score                      \n\
             FROM Picture               \n\
             WHERE FilePath = ?1;", // ""
                params![file_path_as_stored(file_path)],
                Self::rusqlite_row_to_picture,
            )
            .and_then(|mut picture| {
                let mut statement = connection.prepare(
                    "SELECT                   \n\
                Label                     \n\
                FROM Tag                  \n\
                WHERE FilePath = ?1;", // "
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

    pub fn retrieve_picture_with_file_path(&self, file_path: &str) -> IOResult<Picture> {
        match self.rusqlite_retrieve_picture_with_file_path(file_path) {
            Ok(picture) => Ok(picture),
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    pub fn insert_picture(&self, picture: &Picture) -> IOResult<usize> {
        match self.rusqlite_insert_picture(picture) {
            Ok(n) => Ok(n),
            Err(err) => Err(std::io::Error::other(err)),
        }
    }
    pub fn update_picture(&self, picture: &Picture) -> IOResult<usize> {
        match self.rusqlite_update_picture(picture) {
            Ok(n) => Ok(n),
            Err(err) => Err(std::io::Error::other(err)),
        }
    }

    const SELECT_STAR_FROM_PICTURE: &str = "SELECT                     \n\
             FilePath,                  \n\
             Label,                     \n\
             FileSize,                  \n\
             ModifiedTime,              \n\
             Rank,                      \n\
             Sample,                    \n\
             ColorCount,                \n\
             Cover,                     \n\
             Score                      \n\
             FROM Picture              \n"; // "

    // select * from picture where concat(substring(filepath,1,23), substring(filepath,24)) = filepath ;
    fn select_parent_dir(parent_dir: String) -> String {
        let parent = file_path_as_stored(&parent_dir);
        let file_name_start = parent.len() + 2; // to account for /
        format!(
            "FilePath like '{}%' AND Instr(Substring(FilePath, {}), '/') = 0",
            parent, file_name_start
        )
    }

    pub fn retrieve_all_pictures(
        &self,
        retrieve_criteria: RetrieveCriteria,
    ) -> IOResult<Vec<Picture>> {
        self.retrieve_all_parent_dirs().and_then(|parent_dirs| {
            match self.rusqlite_retrieve_all_pictures(
                retrieve_criteria.cover,
                retrieve_criteria.parent_opt,
            ) {
                Ok(picture_map) => match self.rusqlite_retrieve_all_tags() {
                    Ok(tag_map) => {
                        let extraction: HashSet<String> =
                            if let Some(list) = retrieve_criteria.extraction {
                                list.iter().cloned().collect()
                            } else {
                                HashSet::new()
                            };
                        let mut pictures: Vec<Picture> = vec![];
                        let color_range_opt = retrieve_criteria
                            .filter
                            .map(|spec| ColorRange::from_string(&spec));
                        let color_range: ColorRange = match color_range_opt {
                            Some(Ok(ref r)) => r.clone(),
                            Some(Err(_)) | None => ColorRange::default(),
                        };
                        let mut count: usize = 0;
                        for (file_path, image_data) in picture_map.iter() {
                            count += 1;
                            let new_tags = if let Some(tags) = tag_map.get(file_path) {
                                tags.clone()
                            } else {
                                HashSet::new()
                            };
                            let parent_dir = parent_directory(file_path).unwrap();
                            let new_image_data = ImageData {
                                tags: new_tags.clone(),
                                cover: match image_data.clone().cover {
                                    None => None,
                                    Some(_) => {
                                        if let Some(pair) = parent_dirs.get(&parent_dir) {
                                            let count = pair.0;
                                            Some(count)
                                        } else {
                                            Some(0)
                                        }
                                    }
                                },
                                ..image_data.clone()
                            };
                            if !retrieve_criteria.selection_criteria.is_empty()
                                && !retrieve_criteria
                                    .selection_criteria
                                    .matches(new_tags.clone())
                            {
                                continue;
                            };
                            if retrieve_criteria.label.clone().is_some()
                                && *retrieve_criteria.label.as_ref().unwrap()
                                    != new_image_data.label()
                            {
                                continue;
                            };
                            if retrieve_criteria.pattern.clone().is_some()
                                && !retrieve_criteria
                                    .pattern
                                    .as_ref()
                                    .unwrap()
                                    .is_match(file_path)
                            {
                                continue;
                            };
                            if !extraction.is_empty() && !extraction.contains(file_path) {
                                continue;
                            };
                            if color_range_opt.is_some() && !color_range.matches(count, file_path) {
                                continue;
                            };
                            let picture = Picture::new_with_image_data(file_path, &new_image_data);
                            pictures.push(picture)
                        }
                        pictures.sort_by_key(|picture| picture.file_path());
                        Ok(pictures)
                    }
                    Err(err) => Err(std::io::Error::other(err)),
                },
                Err(err) => Err(std::io::Error::other(err)),
            }
        })
    }

    pub fn retrieve_all_pictures_with_parent(&self, parent_dir: &str) -> IOResult<Vec<Picture>> {
        let retrieve_criteria = RetrieveCriteria {
            selection_criteria: SelectionCriteria::empty(),
            label: None,
            extraction: None,
            filter: None,
            pattern: None,
            cover: false,
            parent_opt: Some(parent_dir.to_string()),
        };
        self.retrieve_all_pictures(retrieve_criteria)
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
        let score = row.get(8).expect("can't get column Score");
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
            cover: bool_to_cover(cover),
            score,
        };
        picture.set_image_data(image_data);
        Ok(picture)
    }
    fn rusqulite_retrieve_all_file_paths(&self) -> SqlResult<HashMap<String, (usize, usize)>> {
        let sql_query = "SELECT FilePath, Cover FROM Picture;";
        let connection = self.connection_rc.borrow();
        connection.prepare(sql_query).and_then(|mut statement| {
            let mut map: HashMap<String, (usize, usize)> = HashMap::new();
            statement.query([]).map(|mut rows| {
                while let Some(row) = rows.next().unwrap() {
                    let file_path: String = row.get(0).unwrap();
                    let cover: bool = row.get(1).unwrap();
                    if let Some(directory) = parent_directory(&file_path_as_retrieved(&file_path)) {
                        if let Some(pair) = map.get_mut(&directory) {
                            let count = pair.0;
                            let covers = pair.1;
                            *pair = (count + 1, if cover { covers + 1 } else { covers });
                        } else {
                            let pair = (1, if cover { 1 } else { 0 });
                            let _ = map.insert(directory, pair);
                        }
                    }
                }
                map
            })
        })
    }

    pub fn retrieve_all_parent_dirs(&self) -> IOResult<HashMap<String, (usize, usize)>> {
        match self.rusqulite_retrieve_all_file_paths() {
            Ok(result) => Ok(result),
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn retrieve_all_labels(&self) -> IOResult<HashSet<String>> {
        match self.rusqlite_retrieve_all_labels() {
            Ok(result) => Ok(result),
            Err(e) => Err(IOError::other(e)),
        }
    }
}
// ""
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::cli::args::Args;
    use crate::env::configuration::Configuration;
    use crate::env::default_values::TEST_DATABASE_FILE;
    use crate::file::paths::test::current_directory;
    use crate::file::picture_file::get_data_from_picture_file;
    use crate::model::image_data::TimeStamp;
    use crate::model::image_data::timestamp;
    use crate::model::order::Order;
    use crate::model::palette::Palette;
    use crate::test_data::*;
    use chrono::naive::*;
    use chrono::prelude::*;
    use palette_extract::Color;
    use serial_test::serial;
    use std::collections::HashSet;
    use std::env;
    use std::time::SystemTime;

    pub fn my_db() -> Database {
        let database = Database::rusqlite_from_connection(TEST_DATABASE_FILE, false)
            .expect("test database can't be open");
        database
    }

    pub fn my_args() -> IOResult<Args> {
        let cmd: Option<Vec<&str>> = None;
        let config = Configuration {
            width: 1000,
            height: 1000,
            database_file: format!("{}/{}/gsr2.db", current_directory(), TEST_DATA_DIR),
            temp_dir: format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR),
            marked: HashMap::new(),
            cover: false,
            current_picture: None,
            current_order: Some(Order::Name),
            current_pictures_per_row: Some(1),
        };
        Args::parse_and_check(cmd, &config)
    }

    pub fn dummy_args() -> Args {
        Args {
            command: None,
            directory: None,
            all: false,
            grid: None,
            index: None,
            thumbnails: false,
            order: Some(Order::Name),
            names: false,
            r#move: None,
            label: None,
            extraction: None,
            filter: None,
            pattern: None,
            cover: false,
            height: None,
            width: None,
            select: None,
            restrict: None,
            slideshow: None,
        }
    }

    #[test]
    #[serial]
    fn retrieve_all_pictures_ordered_by_file_path() {
        let database = my_db();
        let status = database.rusqlite_retrieve_all_pictures(false, None);
        assert!(status.is_ok());
        let map = status.unwrap();
        assert_eq!(4, map.len());
    }

    #[serial]
    fn insert_and_retrieve_a_picture_with_image_data() {
        let database = my_db();
        let mut picture = Picture::new("testdata/some_pic.jpeg");
        let file_path = picture.file_path();
        let picture_file_data = get_data_from_picture_file(&nine_colors_file_path()).expect(
            &format!("can't access to file data: {}", nine_colors_file_path()),
        );
        let image_data = ImageData {
            label: "some_label".to_string(),
            size: picture_file_data.0,
            modified_time: picture_file_data.1,
            rank: Rank::ThreeStars,
            score: 0,
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
            cover: None,
            tags: HashSet::from([String::from("foo"), String::from("bar")]),
        };
        picture.set_image_data(image_data.clone());
        assert_eq!(100, picture.image_data().unwrap().palette().count());
        let initial: TimeStamp = picture_file_data.1;
        database.delete_picture_with_file_path(&file_path);

        assert!(database.rusqlite_insert_picture(&picture).is_ok());
        let result = database.rusqlite_retrieve_picture_with_file_path(&file_path);
        database.delete_picture_with_file_path(&file_path);
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
        assert_eq!(
            true,
            retrieved_picture.image_data().unwrap().cover().is_some()
        );
        assert_eq!(2, retrieved_picture.image_data().unwrap().tags.len());
    }

    #[test]
    #[serial]
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
    #[serial]
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
    #[serial]
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

        let criteria = RetrieveCriteria {
            selection_criteria: SelectionCriteria::empty(),
            label: None,
            extraction: None,
            filter: None,
            pattern: None,
            cover: false,
            parent_opt: None,
        };
        let result = database.retrieve_all_pictures(criteria);
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
    #[serial]
    fn finding_all_pictures_with_file_path_having_a_parent_directory() {
        let database = my_db();
        let parent_dir = format!("{}/testdata", current_directory());
        let result = database.retrieve_all_pictures_with_parent(&parent_dir);
        assert!(result.is_ok());
        let pictures = result.unwrap();
        assert_eq!(4, pictures.len());
    }
    #[test]
    #[serial]
    fn find_no_picture_for_a_parent_directory_where_no_picture_exists() {
        let database = my_db();
        let parent_dir = format!("{}", current_directory());
        let result = database.retrieve_all_pictures_with_parent(&parent_dir);
        assert!(result.is_ok());
        let pictures = result.unwrap();
        assert_eq!(0, pictures.len());
    }
}
