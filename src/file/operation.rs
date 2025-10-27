use crate::Database;
use std::fs::remove_file;
use std::fs::copy;
use crate::model::picture::Picture;
use crate::file::paths::{file_exists, file_path_as_retrieved, file_path_as_stored, thumbnail_name_from};
use std::path::PathBuf;
use std::io::Result as IOResult;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Operation {
    Delete(PathBuf),
    Copy(PathBuf, PathBuf),
    MovePictureData(String, String),
}

pub fn delete_operation(file_path: &str) -> Operation {
    Operation::Delete(PathBuf::from(file_path_as_retrieved(file_path)))
}

fn target_file_path(file_path: &str, target_dir: &str) -> PathBuf {
    let mut target_path = PathBuf::from(file_path_as_retrieved(target_dir));
    let source_path = PathBuf::from(file_path_as_retrieved(file_path));
    if let Some(file_name) = source_path.file_name() {
        target_path.push(file_name);
    } else {
        panic!("{} has no file_name", file_path)
    };
    target_path
}

pub fn copy_operation(file_path: &str, target_dir: &str) -> Operation {
    let source_path = PathBuf::from(file_path_as_retrieved(file_path));
    let target_path = target_file_path(file_path, target_dir);
    Operation::Copy(source_path, target_path)
}

pub fn delete_operations(file_path: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    for size in [10, 7, 4, 2] {
        let as_retrieved = file_path_as_retrieved(file_path);
        let file_path_to_delete =thumbnail_name_from(&as_retrieved, size);
        if file_exists(&file_path_to_delete) {
            operations.push(delete_operation(&file_path_to_delete))
        }
    };
    operations.push(delete_operation(file_path));
    operations
}

pub fn copy_operations(file_path: &str, target_dir: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    for size in [10, 7, 4, 2] {
        let as_retrieved = file_path_as_retrieved(file_path);
        let file_path_to_copy =thumbnail_name_from(&as_retrieved, size);
        if file_exists(&file_path_to_copy) {
            operations.push(copy_operation(&file_path_to_copy, target_dir))
        }
    };
    operations.push(copy_operation(file_path, target_dir));
    operations
}

pub fn move_operations(file_path: &str, target_dir: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    let mut copies = copy_operations(file_path, target_dir);
    let mut deletions = delete_operations(file_path);
    operations.append(&mut copies);
    operations.append(&mut deletions);
    operations
}
pub fn move_picture(file_path: &str, target_dir: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    let mut moves = move_operations(file_path, target_dir);
    operations.append(&mut moves);
    let source_file = file_path_as_stored(file_path);
    let target_path = target_file_path(file_path, target_dir);
    let target_file = file_path_as_stored(&target_path.into_os_string().to_str().unwrap().to_string());
    if source_file == target_file {
        println!("same source and target: {}, move cancelled", file_path_as_stored(&source_file))
    } else {
        operations.push(
            Operation::MovePictureData( file_path_as_stored(file_path), target_file)
        )
    };
    operations
}

fn execute_operation(database: &Database, operation: &Operation) -> IOResult<usize> {
    match operation {
        Operation::Delete(path) => {
            match remove_file(path) {
                Ok(()) => Ok(0),
                Err(err) => Err(err),
            }
        },
        Operation::Copy(source_path, target_path) => {
            copy(source_path, target_path).map(|n| n as usize)
        },
        Operation::MovePictureData(source_file_path, target_file_path) => {
            database.retrieve_picture_with_file_path(source_file_path)
                .and_then(|original| {
                    let picture = Picture::copy(&original, target_file_path);
                    database.insert_picture(&picture)
                        .and_then(|_| {
                            database.delete_picture_with_file_path(&original.file_path())
                        })
                })
        }
    }
}
pub fn execute(database: &Database, operations: &Vec<Operation>) -> IOResult<()> {
    for operation in operations {
        match execute_operation(database, operation) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file::paths::{current_directory, home_directory};
    use crate::test_data::*;
    use std::fs::{File, remove_file};
    use std::io::prelude::*;
    use crate::file::database::tests::my_db;
    use std::process::Command;



    fn create_dummy_file(file_path: &str) {
        let mut file = File::create(file_path).expect("can't create test file");
        file.write_all(b"Hello, world!")
            .expect("can't write to file");
    }

    fn remove_dummy_file(file_path: &str) {
        let path = PathBuf::from(file_path);
        remove_file(path);
    }

    #[test]
    fn delete_operation_from_a_file_path_as_stored_is_on_file_path_as_retrieved() {
        let file_path_as_retrieved = format!("{}/foo/bar.jpg", home_directory());
        assert_eq!(
            Operation::Delete(file_path_as_retrieved.into()),
            delete_operation("~/foo/bar.jpg")
        );
    }

    #[test]
    fn copy_operation_from_a_file_path_as_stored_to_a_target_dir() {
        let source_file_path_as_retrieved = format!("{}/foo/bar.jpg", home_directory());
        let target_file_path_as_retrieved = format!("{}/other/bar.jpg", home_directory());
        assert_eq!(
            Operation::Copy(
                source_file_path_as_retrieved.into(),
                target_file_path_as_retrieved.into()
            ),
            copy_operation("~/foo/bar.jpg", "~/other")
        )
    }

    #[test]
    fn batch_delete_operation_for_thumbnails_if_existing() {
        let file_path_to_delete = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "my_file1.foo");
        let other_file_path_to_delete = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "my_file1THUMBLarge.foo");
        create_dummy_file(&file_path_to_delete);
        create_dummy_file(&other_file_path_to_delete);
        let operations = delete_operations(&file_path_to_delete);
        assert_eq!(2, operations.len());
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_delete, 4)),
            operations[0]
        );
        assert_eq!(
            delete_operation(&file_path_to_delete),
            operations[1]
        );
        remove_dummy_file(&file_path_to_delete);
        remove_dummy_file(&other_file_path_to_delete);
    }

    #[test]
    fn batch_delete_operation_for_existing_thumbnails() {
        let file_path_to_delete =
            format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, NINE_COLORS);
        let operations = delete_operations(&file_path_to_delete);
        assert_eq!(5, operations.len());
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_delete, 10)),
            operations[0]
        );
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_delete, 7)),
            operations[1]
        );
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_delete, 4)),
            operations[2]
        );
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_delete, 2)),
            operations[3]
        );
        assert_eq!(
            delete_operation(&file_path_to_delete),
            operations[4]
        );
    }

    #[test]
    fn batch_copy_operation_for_thumbnails_if_existing() {
        let file_path_to_copy = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "my_file2.foo");
        let other_file_path_to_copy = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "my_file2THUMBLarge.foo");
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        create_dummy_file(&file_path_to_copy);
        create_dummy_file(&other_file_path_to_copy);
        let operations = copy_operations(&file_path_to_copy, &target_dir);
        assert_eq!(2, operations.len());
        assert_eq!(
            copy_operation(&thumbnail_name_from(&file_path_to_copy, 4), &target_dir),
            operations[0]);
        assert_eq!(
            copy_operation(&file_path_to_copy, &target_dir),
            operations[1]);
        remove_dummy_file(&file_path_to_copy);
        remove_dummy_file(&other_file_path_to_copy);
    }

    #[test]
    fn move_operation_for_thumbnails_if_existing() {
        let file_path_to_move = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "my_file3.foo");
        let other_file_path_to_move = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "my_file3THUMBLarge.foo");
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        create_dummy_file(&file_path_to_move);
        create_dummy_file(&other_file_path_to_move);
        assert!(file_exists(&file_path_to_move));
        assert!(file_exists(&other_file_path_to_move));
        let operations = move_operations(&file_path_to_move, &target_dir);
        assert_eq!(4, operations.len());
        assert_eq!(
            copy_operation(&thumbnail_name_from(&file_path_to_move, 4), &target_dir),
            operations[0]);
        assert_eq!(
            copy_operation(&file_path_to_move, &target_dir),
            operations[1]);
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_move, 4)),
            operations[2]
        );
        assert_eq!(
            delete_operation(&file_path_to_move),
            operations[3]
        );
        remove_dummy_file(&file_path_to_move);
        remove_dummy_file(&other_file_path_to_move);
    }

    #[test]
    fn moving_a_picture_takes_all_necessary_operations() {
        let picture: Picture = Picture::new(&nine_colors_file_path());
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        let operations = move_picture(&nine_colors_file_path(), &target_dir);
        assert_eq!(11, operations.len());
        assert_eq!(Operation::MovePictureData(
                file_path_as_stored(&nine_colors_file_path()),
                format!("{}/{}/subdir/{}", current_directory(), TEST_DATA_DIR, NINE_COLORS)),
                operations[10]);
    }
    fn executing_operation() {
        let database = my_db();
        let picture: Picture = Picture::new(&nine_colors_file_path());
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        let operations = move_picture(&nine_colors_file_path(), &target_dir);
        execute(&database, operations);
        assert!(file_exists(&format!("{}/{}/subdir/{}", current_directory(), TEST_DATA_DIR, NINE_COLORS)));
        let source_dir = format!("{}/{}", current_directory(), TEST_DATA_DIR);
        let new_file_path = format!("{}/{}/subdir/{}", current_directory(), TEST_DATA_DIR, NINE_COLORS);
        let roll_back = move_picture(&new_file_path, &source_dir);

    }
}
