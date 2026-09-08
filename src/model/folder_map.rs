use crate::file::paths::parent_directory;
use crate::model::folder::Folder;
use crate::model::id_dispenser::IdDispenser;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]

pub struct FolderMap {
    map: BTreeMap<String, Folder>,
}

impl FolderMap {
    pub fn from_file_paths(file_paths: &Vec<String>) -> Self {
        let mut map: BTreeMap<String, Folder> = BTreeMap::new();
        let mut id_dispenser = IdDispenser::new(1);
        for file_path in file_paths.iter() {
            let mut current_directory = file_path.clone();
            while let Some(directory) = parent_directory(&current_directory)
                && !directory.is_empty()
            {
                map.entry(directory.clone())
                    .and_modify(|folder| folder.increase_count(1))
                    .or_insert(Folder::new(id_dispenser.next(), &directory, 0, 1));
                current_directory = directory;
            }
        }
        let id_map: BTreeMap<String, usize> = map
            .iter()
            .map(|(file_path, folder)| (file_path.clone(), folder.id()))
            .collect();

        for (file_path, folder) in map.iter_mut() {
            if let Some(directory) = parent_directory(&file_path)
                && !directory.is_empty()
            {
                if let Some(id) = id_map.get(&directory) {
                    folder.set_parent_id(*id);
                }
            }
        }
        Self { map: map.clone() }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn map(&self) -> BTreeMap<String, Folder> {
        self.map.clone()
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
        assert_eq!(11, folders.len());
        dbg!(&folders);
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
}
