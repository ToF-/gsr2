use crate::file::database::Database;
use crate::file::operation::execute;
use crate::file::operation::move_picture;
use crate::file::picture_file::delete_picture_files;
use crate::model::selection::Selection;
use std::io::Result as IOResult;
pub mod database;
pub mod operation;
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

pub fn move_pictures(
    database: &Database,
    selection: &Selection,
    source_dir: &str,
    target_dir: &str,
) -> IOResult<usize> {
    database
        .retrieve_all_pictures_with_parent(source_dir)
        .and_then(|pictures| {
            for picture in &pictures {
                println!("moving {} to {}", picture.file_path(), target_dir);
                let operations = move_picture(&picture.file_path(), target_dir);
                match execute(database, &operations) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            Ok(pictures.len())
        })
}
