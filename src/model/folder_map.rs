use crate::env::default_values::BASE_DIRECTORY_SYMBOL;
use crate::env::default_values::BASED_PATH_SYMBOL;
use crate::env::default_values::NEAR_DIRECTORY_SYMBOL;
use crate::file::paths::base_directory;
use crate::file::paths::file_path_as_stored;
use crate::file::paths::parent_directory;
use crate::model::folder::Folder;
use crate::model::id_dispenser::FolderId;
use crate::model::id_dispenser::IdDispenser;
use std::collections::BTreeMap;
#[derive(Debug, Default, Clone)]

pub struct FolderMap {
    map: BTreeMap<String, Folder>,
}

impl FolderMap {
    pub fn from_file_paths(file_paths: &[String]) -> Self {
        let mut folder_map = Self::new();
        folder_map.add_from_file_paths(file_paths);
        folder_map
    }

    pub fn new() -> Self {
        let mut folder_map = Self {
            map: BTreeMap::new(),
        };
        let directory = file_path_as_stored(&base_directory());
        folder_map.insert(1, &directory, 0, 0, "");
        folder_map
    }

    pub fn add_from_file_path(&mut self, file_path: &str) {
        let mut id_dispenser = IdDispenser::new(self.last_folder_id() + 1);
        let mut current_file_path: String = file_path.to_string();
        while let Some(parent_directory) = parent_directory(&current_file_path)
            && !parent_directory.is_empty()
        {
            if let Some(folder) = self.map.get_mut(&parent_directory) {
                folder.increase_count(1)
            } else {
                let folder_id = id_dispenser.next_id();
                self.map.insert(
                    parent_directory.clone(),
                    Folder::new(folder_id, &parent_directory, 0, 1, file_path),
                );
            }
            current_file_path = parent_directory;
        }
        self.update_parent_ids();
    }

    fn update_parent_ids(&mut self) {
        let id_map: BTreeMap<String, usize> = self
            .map
            .iter()
            .map(|(file_path, folder)| (file_path.clone(), folder.id()))
            .collect();

        for (file_path, folder) in self.map.iter_mut() {
            if let Some(directory) = parent_directory(file_path)
                && !directory.is_empty()
                && let Some(id) = id_map.get(&directory)
            {
                folder.set_parent_id(*id);
            }
        }
    }
    pub fn add_from_file_paths(&mut self, file_paths: &[String]) {
        let mut id_dispenser = IdDispenser::new(self.last_folder_id() + 1);
        println!("collecting folders from {} files", file_paths.len());
        for file_path in file_paths.iter() {
            self.add_from_file_path(file_path);
        }
    }

    pub fn insert(
        &mut self,
        folder_id: usize,
        file_path: &str,
        parent_id: usize,
        picture_count: usize,
        first_file_path: &str,
    ) {
        self.map.insert(
            file_path.to_string(),
            Folder::new(
                folder_id,
                file_path,
                parent_id,
                picture_count,
                first_file_path,
            ),
        );
    }

    pub fn map(&self) -> BTreeMap<String, Folder> {
        self.map.clone()
    }

    pub fn update(&mut self, directory: &str, folder: &Folder) {
        if let Some(entry) = self.map.get_mut(directory) {
            *entry = folder.clone()
        };
    }

    pub fn get(&self, directory: &str) -> Option<Folder> {
        if directory.is_empty() {
            self.map.get(&format!("{}", BASE_DIRECTORY_SYMBOL)).cloned()
        } else {
            let mut chars = directory.chars();
            let first_char = chars.next().unwrap();
            if first_char == NEAR_DIRECTORY_SYMBOL {
                let target: String = chars.collect();
                self.map
                    .iter()
                    .find(|(key, _)| key.contains(&format!("/{}", &target)))
                    .map(|(_, value)| value)
                    .cloned()
            } else if first_char == BASED_PATH_SYMBOL {
                let target: String = chars.collect();
                self.map
                    .get(&format!("{}/{}", BASE_DIRECTORY_SYMBOL, target))
                    .cloned()
            } else {
                let target: String = directory.to_string();
                self.map.get(&target).cloned()
            }
        }
    }

    pub fn folder(&self, folder_id: FolderId) -> Option<Folder> {
        self.map
            .values()
            .find(|folder| folder.id() == folder_id)
            .cloned()
    }

    pub fn last_folder_id(&self) -> FolderId {
        self.map
            .values()
            .map(|folder| folder.id())
            .max()
            .unwrap_or_default()
    }

    pub fn folders_with_parent_id(&self, parent_id: FolderId) -> Vec<&Folder> {
        self.map
            .values()
            .filter(|folder| folder.parent_id() == parent_id)
            .collect()
    }

    pub fn increase_picture_count(&mut self, folder_id: FolderId, count: usize) {
        if let Some(folder) = self.folder(folder_id) {
            let mut new_folder = folder.clone();
            new_folder.increase_count(count);
            self.update(&new_folder.file_path(), &new_folder);
            self.increase_picture_count(new_folder.parent_id(), count)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creating_a_folder_map_from_file_path() {
        let file_paths: Vec<String> = vec![
            String::from("%/foo.jpg"),
            String::from("%/bun/bar.jpg"),
            String::from("%/bun/qux.jpg"),
            String::from("%/gus/bam/blo.jpg"),
            String::from("%/gus/bim/blu.jpg"),
            String::from("%/gus/bam/bla.jpg"),
            String::from("%/gus/bum/jin/bla.jpg"),
            String::from("%/abc/def/qux.jpg"),
            String::from("%/abc/def/ghi/ijk/lmn.jpg"),
        ];
        let folders = FolderMap::from_file_paths(&file_paths);
        assert_eq!(11, folders.map().len());
        assert_eq!(
            Some(1),
            folders
                .map()
                .get("%/abc/def/ghi/ijk")
                .map(|folder| folder.picture_count())
        );
        assert_eq!(
            Some(1),
            folders
                .map()
                .get("%/abc/def/ghi")
                .map(|folder| folder.picture_count())
        );
        assert_eq!(
            Some(2),
            folders
                .map()
                .get("%/abc/def")
                .map(|folder| folder.picture_count())
        );
        assert_eq!(
            Some(4),
            folders
                .map()
                .get("%/gus")
                .map(|folder| folder.picture_count())
        );
        assert_eq!(
            Some(9),
            folders.map().get("%/abc").map(|folder| folder.id())
        );
        assert_eq!(
            Some(9),
            folders
                .map()
                .get("%/abc/def")
                .map(|folder| folder.parent_id())
        );
    }

    #[test]
    fn getting_a_folder_via_directory_name_or_part_depending_on_syntax() {
        let file_paths: Vec<String> = vec![
            String::from("%/foo.jpg"),
            String::from("%/bun/bar.jpg"),
            String::from("%/bun/qux.jpg"),
            String::from("%/gus/bam/blo.jpg"),
            String::from("%/gus/bim/blu.jpg"),
            String::from("%/gus/bam/bla.jpg"),
            String::from("%/gus/bum/jin/bla.jpg"),
            String::from("%/abc/def/qux.jpg"),
            String::from("%/abc/def/ghi/ijk/lmn.jpg"),
        ];
        let folders = FolderMap::from_file_paths(&file_paths);
        assert_eq!(None, folders.get("foo"));
        assert_eq!(None, folders.get("bun"));
        let folder_opt = folders.get("@bun");
        assert!(folder_opt.is_some());
        assert_eq!(2, folder_opt.unwrap().picture_count());
        let folder_opt = folders.get("@gus/bam");
        assert!(folder_opt.is_some());
        let folder_opt = folders.get("?def");
        assert!(folder_opt.is_some());
    }
    #[test]
    fn new_folder_map_has_base_directory() {
        let folders = FolderMap::new();
        let folder_opt = folders.get("");
        assert!(folder_opt.is_some());
        assert_eq!(1, folder_opt.unwrap().id());
    }
    #[test]
    fn inserting_a_file_path_for_a_new_folder() {
        let mut folders = FolderMap::new();
        folders.add_from_file_path("%/foo/bar/qux.jpg");
        let folder = folders
            .get("@foo/bar")
            .expect("fail: %foo/bar not in folders");
        assert_eq!(1, folder.picture_count());
        let folder = folders.get("@foo").expect("fail: %foo not in folders");
        assert_eq!(1, folder.picture_count());
    }
    #[test]
    fn inserting_several_file_paths_for_a_new_folder_set_the_first_as_cover() {
        let mut folders = FolderMap::new();
        folders.add_from_file_path("%/foo/bar/qux.jpg");
        folders.add_from_file_path("%/foo/bar/law.jpg");
        let folder = folders
            .get("@foo/bar")
            .expect("fail: %foo/bar not in folders");
        assert_eq!(2, folder.picture_count());
        assert_eq!("%/foo/bar/qux.jpg", folder.first_file_path());
    }
}
