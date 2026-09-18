#[derive(Debug, Clone)]
pub struct Folder {
    id: usize,
    file_path: String,
    parent_id: usize,
    picture_count: usize,
    first_file_path: String,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            id: 0,
            file_path: "".to_string(),
            parent_id: 0,
            picture_count: 0,
            first_file_path: "".to_string(),
        }
    }
}
impl Folder {
    pub fn new(
        id: usize,
        file_path: &str,
        parent_id: usize,
        picture_count: usize,
        first_file_path: &str,
    ) -> Self {
        Self {
            id: id,
            file_path: file_path.to_string(),
            parent_id: parent_id,
            picture_count: picture_count,
            first_file_path: first_file_path.to_string(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn file_path(&self) -> String {
        self.file_path.clone()
    }

    pub fn parent_id(&self) -> usize {
        self.parent_id
    }

    pub fn picture_count(&self) -> usize {
        self.picture_count
    }

    pub fn first_file_path(&self) -> String {
        self.first_file_path.clone()
    }
    pub fn increase_count(&mut self, n: usize) {
        self.picture_count += n;
    }
    pub fn set_parent_id(&mut self, id: usize) {
        self.parent_id = id
    }
}
