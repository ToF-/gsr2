use crate::Database;
use crate::file::paths::renamed_file_path;
use crate::file::paths::{
    file_exists, file_path_as_retrieved, file_path_as_stored, thumbnail_name_from,
};
use crate::model::picture::Picture;
use std::fs::copy;
use std::fs::remove_file;
use std::io::Result as IOResult;
use std::path::PathBuf;

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

pub fn copy_at_operation(file_path: &str, target_dir: &str) -> Operation {
    let source_path = PathBuf::from(file_path_as_retrieved(file_path));
    let target_path = target_file_path(file_path, target_dir);
    Operation::Copy(source_path, target_path)
}

pub fn copy_to_operation(file_path: &str, target_file_path: &str) -> Operation {
    let source_path = PathBuf::from(file_path_as_retrieved(file_path));
    let target_path = PathBuf::from(file_path_as_retrieved(&target_file_path));
    Operation::Copy(source_path, target_path)
}
pub fn copy_to_renamed_operation(file_path: &str, target_name: &str) -> Operation {
    copy_to_operation(file_path, &renamed_file_path(file_path, target_name))
}

pub fn delete_operations(file_path: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    for size in [10, 7, 4, 2] {
        let as_retrieved = file_path_as_retrieved(file_path);
        let file_path_to_delete = thumbnail_name_from(&as_retrieved, size);
        if file_exists(&file_path_to_delete) {
            operations.push(delete_operation(&file_path_to_delete))
        }
    }
    operations.push(delete_operation(file_path));
    operations
}

pub fn copy_at_operations(file_path: &str, target_dir: &str) -> Vec<Operation> {
    let as_retrieved = file_path_as_retrieved(file_path);
    let mut operations: Vec<Operation> = vec![];
    for size in [10, 7, 4, 2] {
        let file_path_to_copy = thumbnail_name_from(&as_retrieved, size);
        if file_exists(&file_path_to_copy) {
            operations.push(copy_at_operation(&file_path_to_copy, target_dir))
        }
    }
    operations.push(copy_at_operation(file_path, target_dir));
    operations
}

pub fn copy_to_operations(file_path: &str, target_name: &str) -> Vec<Operation> {
    let as_retrieved = file_path_as_retrieved(file_path);
    let target_as_retrieved = file_path_as_retrieved(&renamed_file_path(file_path, target_name));
    let mut operations: Vec<Operation> = vec![];
    for size in [10, 7, 4, 2] {
        let file_path_to_copy = thumbnail_name_from(&as_retrieved, size);
        let target_file_path = thumbnail_name_from(&target_as_retrieved, size);
        if file_exists(&file_path_to_copy) {
            operations.push(copy_to_operation(&file_path_to_copy, &target_file_path))
        }
    }
    operations.push(copy_to_renamed_operation(file_path, target_name));
    operations
}

pub fn move_operations(file_path: &str, target_dir: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    let mut copies = copy_at_operations(file_path, target_dir);
    let mut deletions = delete_operations(file_path);
    operations.append(&mut copies);
    operations.append(&mut deletions);
    operations
}
pub fn move_picture(file_path: &str, target_dir: &str) -> Vec<Operation> {
    let source_file = file_path_as_stored(file_path);
    let target_path = target_file_path(file_path, target_dir);
    let target_file = file_path_as_stored(target_path.into_os_string().to_str().unwrap());
    if source_file == target_file {
        println!(
            "same source and target: {}, move cancelled",
            file_path_as_stored(&source_file)
        );
        vec![]
    } else {
        let mut operations: Vec<Operation> = vec![];
        let mut moves = move_operations(file_path, target_dir);
        operations.append(&mut moves);
        operations.push(Operation::MovePictureData(
                file_path_as_stored(file_path),
                target_file,
        ));
        operations
    }
}

pub fn rename_picture(file_path: &str, target_name: &str) -> Vec<Operation> {
    let source_file = file_path_as_stored(file_path);
    let target_file = file_path_as_stored(&renamed_file_path(file_path, target_name));
    let mut operations: Vec<Operation> = vec![];
    if source_file == target_file {
        println!(
            "same source and target: {}, rename cancelled",
            file_path_as_stored(&source_file)
        );
        vec![]
    } else {
        let mut copies = copy_to_operations(file_path, target_name);
        let mut deletions = delete_operations(file_path);
        operations.append(&mut copies);
        operations.append(&mut deletions);
        operations.push(Operation::MovePictureData(
                file_path_as_stored(file_path),
                target_file));
            operations
    }
}

fn execute_operation(database: &Database, operation: &Operation) -> IOResult<usize> {
    match operation {
        Operation::Delete(path) => match remove_file(path) {
            Ok(()) => Ok(0),
            Err(err) => Err(err),
        },
        Operation::Copy(source_path, target_path) => {
            copy(source_path, target_path).map(|n| n as usize)
        }
        Operation::MovePictureData(source_file_path, target_file_path) => database
            .retrieve_picture_with_file_path(source_file_path)
            .and_then(|original| {
                let picture = Picture::copy(&original, target_file_path);
                database
                    .insert_picture(&picture)
                    .and_then(|_| database.delete_picture_with_file_path(&original.file_path()))
            }),
    }
}
pub fn execute(database: &Database, operations: &Vec<Operation>) -> IOResult<()> {
    for operation in operations {
        match execute_operation(database, operation) {
            Ok(_) => {}
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
    use crate::file::database::tests::my_db;
    use crate::file::paths::home_directory;
    use crate::file::paths::test::current_directory;
    use crate::test_data::*;
    use serial_test::serial;
    use std::fs::{File, remove_file};
    use std::io::prelude::*;
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
    fn copy_at_operation_from_a_file_path_as_stored_to_a_target_dir() {
        let source_file_path_as_retrieved = format!("{}/foo/bar.jpg", home_directory());
        let target_file_path_as_retrieved = format!("{}/other/bar.jpg", home_directory());
        assert_eq!(
            Operation::Copy(
                source_file_path_as_retrieved.into(),
                target_file_path_as_retrieved.into()
            ),
            copy_at_operation("~/foo/bar.jpg", "~/other")
        )
    }

    #[test]
    fn copy_to_operation_from_a_file_path_as_stored_to_a_target_name() {
        let source_file_path_as_retrieved = format!("{}/foo/bar.jpg", home_directory());
        let target_file_path_as_retrieved = format!("{}/foo/qux.jpg", home_directory());
        assert_eq!(
            Operation::Copy(
                source_file_path_as_retrieved.into(),
                target_file_path_as_retrieved.into()
            ),
            copy_to_renamed_operation("~/foo/bar.jpg", "qux")
        )
    }

    #[test]
    #[serial]
    fn batch_delete_operation_for_thumbnails_if_existing() {
        let file_path_to_delete = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "my_file1.foo"
        );
        let other_file_path_to_delete = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "my_file1THUMBLarge.foo"
        );
        create_dummy_file(&file_path_to_delete);
        create_dummy_file(&other_file_path_to_delete);
        let operations = delete_operations(&file_path_to_delete);
        assert_eq!(2, operations.len());
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_delete, 4)),
            operations[0]
        );
        assert_eq!(delete_operation(&file_path_to_delete), operations[1]);
        remove_dummy_file(&file_path_to_delete);
        remove_dummy_file(&other_file_path_to_delete);
    }

    #[test]
    #[serial]
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
        assert_eq!(delete_operation(&file_path_to_delete), operations[4]);
    }

    #[test]
    #[serial]
    fn batch_copy_at_operation_for_thumbnails_if_existing() {
        let file_path_to_copy = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "my_file2.foo"
        );
        let other_file_path_to_copy = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "my_file2THUMBLarge.foo"
        );
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        create_dummy_file(&file_path_to_copy);
        create_dummy_file(&other_file_path_to_copy);
        let operations = copy_at_operations(&file_path_to_copy, &target_dir);
        assert_eq!(2, operations.len());
        assert_eq!(
            copy_at_operation(&thumbnail_name_from(&file_path_to_copy, 4), &target_dir),
            operations[0]
        );
        assert_eq!(
            copy_at_operation(&file_path_to_copy, &target_dir),
            operations[1]
        );
        remove_dummy_file(&file_path_to_copy);
        remove_dummy_file(&other_file_path_to_copy);
    }
    #[test]
    #[serial]
    fn batch_copy_to_operation_for_thumbnails_if_existing() {
        let file_path_to_copy = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "foo.jpg");
        let target_file_path = format!("{}/{}/{}", current_directory(), TEST_DATA_DIR, "bar.jpg");
        let other_file_path_to_copy = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "fooTHUMBLarge.jpg"
        );
        let other_target_file = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "barTHUMBLarge.jpg"
        );
        create_dummy_file(&file_path_to_copy);
        create_dummy_file(&other_file_path_to_copy);
        let operations = copy_to_operations(&file_path_to_copy, "bar");
        assert_eq!(2, operations.len());
        assert_eq!(
            copy_to_operation(
                &thumbnail_name_from(&file_path_to_copy, 4),
                &thumbnail_name_from(&target_file_path, 4)
            ),
            operations[0]
        );
        assert_eq!(
            copy_to_operation(&file_path_to_copy, &target_file_path),
            operations[1]
        );
        remove_dummy_file(&file_path_to_copy);
        remove_dummy_file(&other_file_path_to_copy);
    }

    #[test]
    #[serial]
    fn move_operation_for_thumbnails_if_existing() {
        let file_path_to_move = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "my_file3.foo"
        );
        let other_file_path_to_move = format!(
            "{}/{}/{}",
            current_directory(),
            TEST_DATA_DIR,
            "my_file3THUMBLarge.foo"
        );
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        create_dummy_file(&file_path_to_move);
        create_dummy_file(&other_file_path_to_move);
        assert!(file_exists(&file_path_to_move));
        assert!(file_exists(&other_file_path_to_move));
        let operations = move_operations(&file_path_to_move, &target_dir);
        assert_eq!(4, operations.len());
        assert_eq!(
            copy_at_operation(&thumbnail_name_from(&file_path_to_move, 4), &target_dir),
            operations[0]
        );
        assert_eq!(
            copy_at_operation(&file_path_to_move, &target_dir),
            operations[1]
        );
        assert_eq!(
            delete_operation(&thumbnail_name_from(&file_path_to_move, 4)),
            operations[2]
        );
        assert_eq!(delete_operation(&file_path_to_move), operations[3]);
        remove_dummy_file(&file_path_to_move);
        remove_dummy_file(&other_file_path_to_move);
    }

    #[test]
    #[serial]
    fn moving_a_picture_takes_all_necessary_operations() {
        let source_dir = format!("{}/{}/", current_directory(), TEST_DATA_DIR);
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        let operations = move_picture(&nine_colors_file_path(), &target_dir);
        let source_file = file_path_as_stored(&nine_colors_file_path());
        let target_file = file_path_as_stored(&format!(
            "{}/{}/subdir/{}",
            current_directory(),
            TEST_DATA_DIR,
            NINE_COLORS
        ));
        assert_eq!(11, operations.len());
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBSmall.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}/nine_colorsTHUMBSmall.png", target_dir)).into()
            ),
            operations[0]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBMedium.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}/nine_colorsTHUMBMedium.png", target_dir))
                    .into()
            ),
            operations[1]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarge.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}/nine_colorsTHUMBLarge.png", target_dir)).into()
            ),
            operations[2]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarger.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}/nine_colorsTHUMBLarger.png", target_dir))
                    .into()
            ),
            operations[3]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&source_file).into(),
                file_path_as_retrieved(&target_file).into()
            ),
            operations[4]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBSmall.png", source_dir)).into()
            ),
            operations[5]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBMedium.png", source_dir)).into()
            ),
            operations[6]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarge.png", source_dir)).into()
            ),
            operations[7]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarger.png", source_dir)).into()
            ),
            operations[8]
        );
        assert_eq!(
            Operation::Delete(file_path_as_retrieved(&source_file).into()),
            operations[9]
        );
        assert_eq!(
            Operation::MovePictureData(source_file.clone(), target_file.clone()),
            operations[10]
        );
    }

    #[test]
    fn renaming_a_picture_takes_all_necessary_operations() {
        let source_dir = format!("{}/{}/", current_directory(), TEST_DATA_DIR);
        let picture: Picture = Picture::new(&nine_colors_file_path());
        let target_name = "nine_colors_foo";
        let operations = rename_picture(&nine_colors_file_path(), &target_name);
        let source_file = file_path_as_stored(&nine_colors_file_path());
        let target_file = file_path_as_stored(&format!(
            "{}/{}/nine_colors_foo.png",
            current_directory(),
            TEST_DATA_DIR
        ));
        assert_eq!(11, operations.len());
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBSmall.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}nine_colors_fooTHUMBSmall.png", source_dir))
                    .into()
            ),
            operations[0]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBMedium.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}nine_colors_fooTHUMBMedium.png", source_dir))
                    .into()
            ),
            operations[1]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarge.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}nine_colors_fooTHUMBLarge.png", source_dir))
                    .into()
            ),
            operations[2]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarger.png", source_dir)).into(),
                file_path_as_retrieved(&format!("{}nine_colors_fooTHUMBLarger.png", source_dir))
                    .into()
            ),
            operations[3]
        );
        assert_eq!(
            Operation::Copy(
                file_path_as_retrieved(&source_file).into(),
                file_path_as_retrieved(&target_file).into()
            ),
            operations[4]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBSmall.png", source_dir)).into()
            ),
            operations[5]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBMedium.png", source_dir)).into()
            ),
            operations[6]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarge.png", source_dir)).into()
            ),
            operations[7]
        );
        assert_eq!(
            Operation::Delete(
                file_path_as_retrieved(&format!("{}nine_colorsTHUMBLarger.png", source_dir)).into()
            ),
            operations[8]
        );
        assert_eq!(
            Operation::Delete(file_path_as_retrieved(&source_file).into()),
            operations[9]
        );
        assert_eq!(
            Operation::MovePictureData(source_file.clone(), target_file.clone()),
            operations[10]
        );
    }
    #[test]
    #[serial]
    fn executing_operation() {
        let database = my_db();
        let picture: Picture = Picture::new(&nine_colors_file_path());
        let target_dir = format!("{}/{}/subdir", current_directory(), TEST_DATA_DIR);
        let operations = move_picture(&nine_colors_file_path(), &target_dir);
        execute(&database, &operations);
        assert!(file_exists(&format!(
            "{}/{}/subdir/{}",
            current_directory(),
            TEST_DATA_DIR,
            NINE_COLORS
        )));
        let source_dir = format!("{}/{}", current_directory(), TEST_DATA_DIR);
        let new_file_path = format!(
            "{}/{}/subdir/{}",
            current_directory(),
            TEST_DATA_DIR,
            NINE_COLORS
        );
        let roll_back = move_picture(&new_file_path, &source_dir);
        execute(&database, &roll_back);
    }

    #[test]
    fn moving_picture_to_the_same_directory_not_allowed() {
        let target_dir = format!("{}/{}", current_directory(), TEST_DATA_DIR);
        let operations = move_picture(&nine_colors_file_path(), &target_dir);
        assert_eq!(0, operations.len());
    }
    #[test]
    fn renaming_picture_to_the_same_name_not_allowed() {
        let operations = rename_picture(&nine_colors_file_path(), "nine_colors");
        assert_eq!(0, operations.len());
    }
}
