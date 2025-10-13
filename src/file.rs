use crate::file::picture_file::delete_picture_files;
use crate::file::database::Database;
use std::io::Result as IOResult;
pub mod database;
pub mod paths;
pub mod picture_file;

pub fn delete_picture(database: &Database, file_path: &str) -> IOResult<()> {
    match database.rusqlite_delete_picture_with_file_path(file_path) {
        Ok(_) => match delete_picture_files(file_path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            },
        Err(err) => Err(std::io::Error::other(err)),
    }
}
