use std::path::PathBuf;
use crate::file::paths::file_path_as_retrieved;

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Delete(PathBuf),
    Copy(PathBuf, PathBuf),
}

pub fn delete_operation(file_path: &str) -> Operation {
        Operation::Delete(PathBuf::from(file_path_as_retrieved(file_path)))
}

pub fn copy_operation(file_path: &str, target_dir: &str) -> Operation {
    let mut target_path = PathBuf::from(file_path_as_retrieved(target_dir));
    let source_path = PathBuf::from(file_path_as_retrieved(file_path));
    if let Some(file_name) = source_path.file_name() {
        target_path.push(file_name);
        Operation::Copy(source_path, target_path)
    } else {
        panic!("{} has no file_name", file_path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file::paths::{current_directory, home_directory};

    #[test]
    fn delete_operation_from_a_file_path_as_stored_is_on_file_path_as_retrieved() {
        let file_path_as_retrieved = format!("{}/foo/bar.jpg", home_directory());
        assert_eq!(
            Operation::Delete(file_path_as_retrieved.into()),
            delete_operation("~/foo/bar.jpg"));
    }

    #[test]
    fn copy_operation_from_a_file_path_as_stored_to_a_target_dir() {
        let source_file_path_as_retrieved = format!("{}/foo/bar.jpg", home_directory());
        let target_file_path_as_retrieved = format!("{}/other/bar.jpg", home_directory());
        assert_eq!(
            Operation::Copy(
                source_file_path_as_retrieved.into(),
                target_file_path_as_retrieved.into()),
            copy_operation("~/foo/bar.jpg", "~/other"))
    }
}

