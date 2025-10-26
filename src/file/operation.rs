use std::path::PathBuf;
use crate::file::paths::file_path_as_retrieved;

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Delete(PathBuf),
}

pub fn delete_operation(file_path: &str) -> Operation {
        Operation::Delete(PathBuf::from(file_path_as_retrieved(file_path)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file::paths::current_directory;

    fn delete_operation_from_a_file_path_as_stored_is_on_file_path_as_retrieved() {
        let file_path_as_stored = String::from("~/foo/bar.jpg");
        let file_path_as_retrieved = format!("{}/foo/bar.jpg", current_directory());
        assert_eq!(
            Operation::Delete(file_path_as_retrieved.into()),
            delete_operation(&file_path_as_stored));
    }
}

