use crate::file::database::Database;
use crate::file::picture_file::delete_picture_files;
use std::io::Result as IOResult;
pub mod database;
mod operation;
pub mod paths;
pub mod picture_file;

pub fn delete_picture(database: &Database, file_path: &str) -> IOResult<()> {
    match database.delete_picture_with_file_path(file_path) {
        Ok(_) => match delete_picture_files(file_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Err(err) => Err(std::io::Error::other(err)),
    }
}
