use crate::file::paths::parent_directory;
use crate::model::folder::Folder;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]

pub struct FolderMap {
    map: BTreeMap<String, Folder>,
}

impl FolderMap {
    pub fn from_file_paths(file_paths: &Vec<String>) -> Self {
        let mut map: BTreeMap<String, Folder> = BTreeMap::new();
        let mut id = 0;
        for file_path in file_paths {
            if let Some(parent_path) = parent_directory(&file_path) {
                if !parent_path.is_empty() {
                    map.entry(parent_path.clone())
                        .and_modify(|folder| folder.increase_count(1))
                        .or_insert(Folder::new(
                            {
                                id += 1;
                                id
                            },
                            &parent_path,
                            0,
                            1,
                        ));
                }
            }
        }
        let mut trace = 0;
        while map.len() > trace {
            let folders: Vec<Folder> = map.values().cloned().collect();
            trace = map.len();
            for folder in folders.iter() {
                let mut count = 0;
                if let Some(parent_path) = parent_directory(&folder.file_path()) {
                    if !parent_path.is_empty() {
                        if map.get(&parent_path.clone()).is_none() {
                            map.insert(
                                parent_path.clone(),
                                Folder::new(
                                    {
                                        id += 1;
                                        id
                                    },
                                    &parent_path,
                                    0,
                                    folder.picture_count(),
                                ),
                            );
                        }
                    }
                }
            }
        }
        Self { map: map }
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
    }
}
