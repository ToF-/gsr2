#[derive(Debug, Clone)]
pub struct Folder {
    id: usize,
    file_path: String,
    parent_id: usize,
    picture_count: usize,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            id: 0,
            file_path: "".to_string(),
            parent_id: 0,
            picture_count: 0,
        }
    }
}
