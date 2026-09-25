use crate::env::default_values::BASE_DIRECTORY_SYMBOL;
use crate::env::default_values::BASED_PATH_SYMBOL;
use crate::env::default_values::NEAR_DIRECTORY_SYMBOL;
use crate::file::paths::parent_directory;
use crate::model::folder::Folder;
use crate::model::id_dispenser::IdDispenser;
use std::collections::BTreeMap;
#[derive(Debug, Default, Clone)]

pub struct FolderMap {
    map: BTreeMap<String, Folder>,
}

impl FolderMap {
    pub fn from_file_paths(file_paths: &[String]) -> Self {
        let mut map: BTreeMap<String, Folder> = BTreeMap::new();
        let mut id_dispenser = IdDispenser::new(1);
        for file_path in file_paths.iter() {
            let mut current_directory = file_path.clone();
            while let Some(directory) = parent_directory(&current_directory)
                && !directory.is_empty()
            {
                map.entry(directory.clone())
                    .and_modify(|folder| folder.increase_count(1))
                    .or_insert(Folder::new(id_dispenser.next_id(), &directory, 0, 1, ""));
                current_directory = directory;
            }
        }
        let id_map: BTreeMap<String, usize> = map
            .iter()
            .map(|(file_path, folder)| (file_path.clone(), folder.id()))
            .collect();

        for (file_path, folder) in map.iter_mut() {
            if let Some(directory) = parent_directory(file_path)
                && !directory.is_empty()
                && let Some(id) = id_map.get(&directory)
            {
                folder.set_parent_id(*id);
            }
        }
        Self { map: map.clone() }
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
            Some(20),
            folders.map().get("%/abc").map(|folder| folder.id())
        );
        assert_eq!(
            Some(20),
            folders
                .map()
                .get("%/abc/def")
                .map(|folder| folder.parent_id())
        );
    }

    #[test]
    fn getting_a_folder_via_directory_name_or_part() {
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
        let folder_opt = folders.get("bun");
        assert!(folder_opt.is_some());
        let folder_opt = folders.get("gus/bam");
        assert!(folder_opt.is_some());
        let folder_opt = folders.get("?def");
        assert!(folder_opt.is_some());
    }
}
