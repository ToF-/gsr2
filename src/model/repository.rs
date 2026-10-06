use crate::cli::command::Command;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::CONFIGURATION;
use crate::env::configuration::Configuration;
use crate::env::configuration::set_configuration_updated_flag;
use crate::file::database::Database;
use crate::file::operation::execute;
use crate::file::operation::move_picture;
use crate::file::operation::rename_picture;
use crate::file::paths::file_exists;
use crate::file::paths::file_path_as_retrieved;
use crate::file::paths::file_path_as_stored;
use crate::file::paths::parent_directory;
use crate::file::paths::timestamp_filename;
use crate::file::picture_file::collect_picture_data;
use crate::file::picture_file::copy_picture_file_to_directory;
use crate::file::picture_file::delete_picture_files;
use crate::file::picture_file::get_all_picture_file_paths;
use crate::file::picture_file::get_picture_file_path;
use crate::model::catalog::Catalog;
use crate::model::category::Category;
use crate::model::folder::Folder;
use crate::model::folder_map::FolderMap;
use crate::model::gallery::Gallery;
use crate::model::id_dispenser::FolderId;
use crate::model::order::Order;
use crate::model::picture::Picture;
use crate::model::predicate::Predicate;
use crate::model::rank::Rank;
use crate::model::retrieve_criteria::RetrieveCriteria;
use crate::model::tag_selection_criteria::TagSelectionCriteria;
use crate::model::tags::Tags;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Error as IOError;
use std::io::Result as IOResult;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

pub const CREATE_DATABASE: bool = false;
pub const DATABASE_EXISTS: bool = true;

#[derive(Debug, Clone)]
pub struct Repository {
    command_line_arguments: CommandLineArguments,
    database: Database,
    tags_rc: RefCell<Tags>,
    categories_rc: RefCell<Tags>,
    gallery_rc: Rc<RefCell<Gallery>>,
    parent_dirs_rc: RefCell<HashMap<String, (usize, usize)>>,
    folder_map_rc: RefCell<FolderMap>,
}

impl Repository {
    pub fn new(clargs: CommandLineArguments, create: bool) -> Self {
        let configuration = CONFIGURATION.get().expect("configuration not set");
        let database = Database::from_connection(&configuration.database_file, create).unwrap();
        Repository {
            command_line_arguments: clargs.clone(),
            database,
            tags_rc: RefCell::new(crate::model::tags::empty_tags()),
            categories_rc: RefCell::new(crate::model::tags::empty_tags()),
            gallery_rc: Rc::new(RefCell::new(Gallery::new())),
            parent_dirs_rc: RefCell::new(HashMap::new()),
            folder_map_rc: RefCell::new(FolderMap::default()),
        }
    }

    pub fn create_database(configuration: Configuration) -> IOResult<()> {
        println!("creating new database file {}", configuration.database_file);
        match Database::from_connection(&configuration.database_file, true) {
            Ok(database) => match database.rusqlite_create_schema() {
                Ok(_) => Ok(()),
                Err(e) => Err(IOError::other(format!("{}", e))),
            },
            Err(e) => Err(IOError::other(format!("{}", e))),
        }
    }

    pub fn insert_mark(&self, letter: char, file_path: &str) -> IOResult<usize> {
        match self.database.rusqlite_insert_mark(letter, file_path) {
            Ok(n) => Ok(n),
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn retrieve_all_categories(&self) -> IOResult<()> {
        let mut categories = self.categories_rc.borrow_mut();
        self.database.select_categories().and_then(|names| {
            *categories = Tags::from(names);
            Ok(())
        })
    }

    pub fn retrieve_all_folders(&self) -> IOResult<()> {
        let mut folder_map = self.folder_map_rc.borrow_mut();
        self.database.select_folders().and_then(|map| {
            *folder_map = map;
            if folder_map.map().is_empty() {
                let _ = folder_map.insert(1, "%", 0, 0, "");
                self.database.insert_or_update_folders(folder_map.clone());
                Ok(())
            } else {
                Ok(())
            }
        })
    }

    pub fn retrieve_all_labels(&self) -> IOResult<()> {
        let mut tags = self.tags_rc.borrow_mut();
        self.database.select_labels().and_then(|labels| {
            *tags = Tags::from(labels);
            Ok(())
        })
    }

    fn retrieve_all_pictures(
        &self,
        args: &CommandLineArguments,
        predicate_opt: Option<Predicate>,
    ) -> IOResult<usize> {
        self.retrieve_catalog().and_then(|catalog| {
            let mut gallery = self.gallery_rc.borrow_mut();
            RetrieveCriteria::new(args, predicate_opt, Some(catalog.clone())).and_then(
                |retrieve_criteria| {
                    self.database
                        .select_pictures(retrieve_criteria, None)
                        .and_then(|pictures| {
                            let mut new_gallery = Gallery::new_with_pictures(pictures);
                            *gallery = new_gallery.clone();
                            gallery.sort_by(args.order.unwrap_or(Order::Name));
                            Ok(gallery.len())
                        })
                },
            )
        })
    }

    fn retrieve_all_pictures_and_folders(
        &self,
        args: &CommandLineArguments,
        predicate_opt: Option<Predicate>,
        folder_id: FolderId,
    ) -> IOResult<usize> {
        self.retrieve_catalog().and_then(|catalog| {
            let mut gallery = self.gallery_rc.borrow_mut();
            RetrieveCriteria::new(args, predicate_opt, Some(catalog.clone())).and_then(
                |retrieve_criteria| {
                    self.database
                        .select_pictures(retrieve_criteria, Some(folder_id))
                        .and_then(|pictures| {
                            let mut new_gallery = Gallery::new_with_pictures(pictures);
                            new_gallery.set_structured();
                            let folder_map = self.folder_map_rc.borrow();
                            let folders = folder_map.folders_with_parent_id(folder_id);
                            for folder in folders {
                                new_gallery.add_picture(&Picture::for_folder(folder));
                            }
                            new_gallery.sort_by(args.order.unwrap_or(Order::Name));
                            *gallery = new_gallery.clone();
                            gallery.sort_by(args.order.unwrap_or(Order::Name));
                            Ok(gallery.len())
                        })
                },
            )
        })
    }

    fn retrieve_all_parent_dirs(&self) -> IOResult<()> {
        match self.database.select_parent_dirs() {
            Ok(map) => {
                let mut parent_dirs = self.parent_dirs_rc.borrow_mut();
                *parent_dirs = map;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn retrieve_all_picture_file_paths(
        &self,
        added_file_paths: Option<Vec<String>>,
    ) -> IOResult<FolderMap> {
        if added_file_paths.is_some() {
            let last_folder_id = {
                let folder_map = self.folder_map_rc.borrow();
                folder_map.last_folder_id()
            };
            let mut folder_map = self.folder_map_rc.borrow_mut();
            folder_map.add_from_file_paths(&added_file_paths.unwrap());
            Ok(folder_map.clone())
        } else {
            match self.database.select_all_picture_file_paths() {
                Ok(file_paths) => Ok(FolderMap::from_file_paths(&file_paths)),
                Err(e) => Err(IOError::other(e)),
            }
        }
    }

    pub fn amend_folders(&self, folder_counts: HashMap<String, (String, usize)>) -> IOResult<()> {
        let mut new_folders: Vec<String> = Vec::new();
        {
            let mut folders = self.folder_map_rc.borrow_mut();
            for (directory, (first_file_path, count)) in folder_counts.iter() {
                if let Some(folder) = folders.get(&directory) {
                    folders.increase_picture_count(folder.id(), *count)
                } else {
                    folders.add_from_file_path(first_file_path);
                    println!("{}",&directory);
                    new_folders.push(file_path_as_stored(directory));
                }
            }
        }
        let folders = self.folder_map_rc.borrow();
        dbg!(&folders);
        self.database.update_folders(&folders).and_then(|_| {
            for parent_directory in new_folders {
                if let Some(folder) = folders.get(&parent_directory) {
                    match self
                        .database
                        .update_picture_folder_id(&parent_directory, folder.id())
                    {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
                } else {
                    return Err(IOError::other(format!("can't access folder {}", parent_directory)));
                }
            }
            Ok(())
        })
    }

    pub fn amend_all_folders(&self, added_file_paths: Option<Vec<String>>) -> IOResult<usize> {
        match self.retrieve_all_picture_file_paths(added_file_paths.clone()) {
            Ok(folder_map) => {
                let res = if added_file_paths.is_some() {
                    self.database.insert_or_update_folders(folder_map)
                } else {
                    self.database.renew_all_folders(folder_map)
                };
                match res {
                    Ok(n) => match self.database.select_all_cover_filepaths() {
                        Ok(covers) => {
                            let cover_map: BTreeMap<String, String> = covers
                                .into_iter()
                                .filter_map(|file_path| {
                                    parent_directory(&file_path).map(|parent| (parent, file_path))
                                })
                                .collect();
                            let _ = self.retrieve_all_folders();
                            let folder_map = self.folder_map_rc.borrow();
                            for folder in folder_map.map().values() {
                                let directory = folder.file_path();
                                let folder_id = folder.id();
                                match self
                                    .database
                                    .update_picture_folder_id(&directory, folder_id)
                                {
                                    Ok(_) => {}
                                    Err(e) => return Err(e),
                                }
                                if let Some(cover_file_path) = cover_map.get(&directory) {
                                    println!("{}->{}", folder_id, cover_file_path);
                                    match self.database.update_folder_first_file_path_for_id(
                                        folder_id,
                                        cover_file_path,
                                    ) {
                                        Ok(_) => {}
                                        Err(e) => return Err(e),
                                    }
                                }
                            }
                            set_configuration_updated_flag(true);
                            Ok(n)
                        }
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
    pub fn len(&self) -> usize {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            gallery.len()
        } else {
            panic!("can't borrow")
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn order(&self) -> Order {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            gallery.order()
        } else {
            panic!("can't borrow")
        }
    }

    pub fn catalog(&self) -> Catalog {
        if let Ok(catalog) = self.retrieve_catalog() {
            catalog.clone()
        } else {
            panic!("can't borrow")
        }
    }

    pub fn add_category(
        &self,
        new_category_name: &str,
        target_category_name: &str,
    ) -> IOResult<()> {
        if let Ok(mut catalog) = self.retrieve_catalog() {
            match catalog.add_sub_category(new_category_name, target_category_name) {
                Ok(_) => self.save_catalog(&catalog),
                Err(e) => Err(IOError::other(e)),
            }
        } else {
            panic!("can't borrow")
        }
    }

    pub fn move_category(
        &self,
        moving_category_name: &str,
        target_category_name: &str,
    ) -> IOResult<()> {
        if let Ok(mut catalog) = self.retrieve_catalog() {
            match catalog.move_sub_category(moving_category_name, target_category_name) {
                Ok(_) => self.save_catalog(&catalog),
                Err(e) => Err(IOError::other(e)),
            }
        } else {
            panic!("can't borrow")
        }
    }

    pub fn remove_category(&self, category_name: &str) -> IOResult<()> {
        if let Ok(mut catalog) = self.retrieve_catalog() {
            match catalog.remove_category(category_name, false) {
                Ok(_) => self.save_catalog(&catalog),
                Err(e) => Err(IOError::other(e)),
            }
        } else {
            panic!("can't borrow")
        }
    }

    pub fn retrieve_all_marks(&self) -> IOResult<BTreeMap<char, String>> {
        match self.database.rusqlite_select_all_marks() {
            Ok(map) => Ok(map),
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn retrieve_pictures(&self, predicate_opt: Option<Predicate>) -> IOResult<usize> {
        match &self.command_line_arguments.command {
            Some(Command::File { file_path }) => match self.picture_from_file_path(file_path) {
                Ok(file_gallery) => match self.gallery_rc.try_borrow_mut() {
                    Ok(mut gallery) => {
                        *gallery = file_gallery.clone();
                        Ok(1)
                    }
                    Err(e) => Err(IOError::other(e)),
                },
                Err(e) => Err(e),
            },
            Some(Command::Directory { directory }) => match self.pictures_in_directory(directory) {
                Ok(dir_gallery) => match self.gallery_rc.try_borrow_mut() {
                    Ok(mut gallery) => {
                        *gallery = dir_gallery.clone();
                        Ok(gallery.len())
                    }
                    Err(e) => Err(IOError::other(e)),
                },
                Err(e) => Err(e),
            },
            // any other command involves the picture database
            _ => self.retrieve_all_folders().and_then(|_| {
                let directory: &String = &self
                    .command_line_arguments
                    .directory
                    .clone()
                    .unwrap_or_default();
                let folder_map = self.folder_map_rc.borrow();
                if let Some(folder) = folder_map.get(&directory) {
                    self.retrieve_all_labels().and_then(|_| {
                        self.retrieve_all_parent_dirs().and_then(|_| {
                            if self.command_line_arguments.structured {
                                self.retrieve_all_pictures_and_folders(
                                    &self.command_line_arguments.clone(),
                                    predicate_opt,
                                    folder.id(),
                                )
                            } else {
                                self.retrieve_all_pictures(
                                    &self.command_line_arguments.clone(),
                                    predicate_opt,
                                )
                            }
                        })
                    })
                } else {
                    Err(IOError::other(format!("folder {} not found", directory)))
                }
            }),
        }
    }

    pub fn save_catalog(&self, catalog: &Catalog) -> IOResult<()> {
        match self.database.rusqlite_update_catalog(&catalog.to_sexp()) {
            Ok(_) => Ok(()),
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn save_folders(&self) -> IOResult<usize> {
        let folders = self.folder_map_rc.borrow();
        match self.database.rusqlite_update_folders(&folders) {
            Ok(n) => Ok(n),
            Err(e) => Err(IOError::other(e)),
        }
    }
    pub fn initialize_for_args(
        &self,
        args: &CommandLineArguments,
        predicate_opt: Option<Predicate>,
    ) -> IOResult<usize> {
        self.retrieve_all_pictures(args, predicate_opt)
    }

    pub fn pictures_in_directory(&self, dir: &str) -> IOResult<Gallery> {
        let mut pictures: Vec<Picture> = vec![];
        get_all_picture_file_paths(dir).and_then(|list| {
            for file_path in list {
                match Picture::new_with_file_image_data(&file_path, "") {
                    Ok(picture) => pictures.push(picture),
                    Err(err) => return Err(err),
                }
            }
            Ok(Gallery::new_with_pictures(pictures))
        })
    }

    pub fn collect_data(&self, directory: &str) -> IOResult<()> {
        println!("gallery count before collect:{}\n", self.len());
        let mut folder_counts: HashMap<String, (String, usize)> = HashMap::new();
        let mut added_file_paths: Vec<String> = Vec::new();
        self.pictures_in_directory(directory).and_then(|gallery| {
            println!(
                "pictures in directory {} : {}\n",
                &directory,
                gallery.clone().len()
            );
            let total: usize = gallery.len();
            let mut count: usize = 0;
            for picture in gallery.pictures() {
                if self
                    .database
                    .rusqlite_check_picture_with_file_path(&picture.file_path())
                    .is_err()
                {
                    let file_path = file_path_as_stored(&picture.file_path().clone());
                    added_file_paths.push(file_path.clone());
                    match collect_picture_data(picture) {
                        Ok(picture) => match self.database.insert_picture(&picture) {
                            Ok(_) => {
                                count += 1;
                                println!("{}/{}:{}", count, total, &picture.file_path());
                                if let Some(parent_directory) =
                                    parent_directory(&picture.file_path())
                                {
                                    folder_counts
                                        .entry(parent_directory)
                                        .and_modify(|pair| pair.1 += 1)
                                        .or_insert((file_path.clone(), 1));
                                }
                            }
                            Err(err) => {
                                eprintln!("{}:\n{}", picture.file_path(), err)
                            }
                        },
                        Err(err) => {
                            println!("{}", err)
                        }
                    };
                }
            }
            println!("{} pictures added", count);
            if count > 0 {
                self.amend_folders(folder_counts)
            } else {
                Ok(())
            }
        })
    }

    pub fn picture_from_file_path(&self, file_path: &str) -> IOResult<Gallery> {
        get_picture_file_path(file_path).and_then(|path| {
            Picture::new_with_file_image_data(&path, "")
                .map(|picture| Gallery::new_with_pictures(vec![picture]))
        })
    }

    pub fn picture_at(&self, position: usize) -> Picture {
        if let Ok(gallery) = self.gallery_rc().try_borrow() {
            gallery.picture(position)
        } else {
            panic!("can't borrow gallery")
        }
    }

    pub fn set_command_line_arguments(&mut self, clargs: CommandLineArguments) {
        self.command_line_arguments = clargs
    }
    pub fn set_picture_at(&self, position: usize, picture: &Picture) {
        if let Ok(mut gallery) = self.gallery_rc().try_borrow_mut() {
            gallery.set_picture(position, picture.clone());
            match self.update_picture(picture) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        } else {
            panic!("can't borrow gallery")
        }
    }

    pub fn all_labels(&self) -> Tags {
        let tags = self
            .tags_rc
            .try_borrow()
            .expect("can't borrow repository tags");
        tags.clone()
    }

    pub fn all_categories(&self) -> Tags {
        let tags = self
            .categories_rc
            .try_borrow()
            .expect("can't borrow repository categories");
        tags.clone()
    }

    pub fn add_label(&self, label: &str) {
        let mut tags = self
            .tags_rc
            .try_borrow_mut()
            .expect("can't borrow mutably repository tags");
        tags.insert(label.to_string());
    }

    pub fn gallery_rc(&self) -> Rc<RefCell<Gallery>> {
        self.gallery_rc.clone()
    }

    pub fn gallery(&self) -> Ref<'_, Gallery> {
        self.gallery_rc.borrow()
    }

    pub fn gallery_mut(&self) -> RefMut<'_, Gallery> {
        self.gallery_rc.borrow_mut()
    }
    pub fn parent_dirs(&self) -> HashMap<String, (usize, usize)> {
        self.parent_dirs_rc.borrow().clone()
    }

    pub fn directory_count_at_index(&self, index: usize) -> usize {
        let gallery = self.gallery_rc.borrow();
        let picture = &gallery.pictures()[index];
        parent_directory(&picture.file_path())
            .map(|directory| file_path_as_stored(&directory))
            .map(|directory| {
                let configuration = CONFIGURATION.get().expect("configuration not set");
                if configuration.updated {
                    let folders = self.folder_map_rc.borrow();
                    folders
                        .get(&directory)
                        .map(|folder| folder.picture_count())
                        .unwrap_or_default()
                } else {
                    self.parent_dirs()
                        .get(&directory)
                        .map(|pair| pair.0)
                        .unwrap_or_default()
                }
            })
            .unwrap_or_default()
    }

    pub fn covers(&self) -> usize {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            gallery
                .pictures()
                .iter()
                .map(|p| if p.cover().is_some() { 1 } else { 0 })
                .sum()
        } else {
            panic!("can't borrow")
        }
    }

    pub fn update_picture_scores(&self, scores: HashMap<String, u32>) {
        println!("updating picture scores");
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            scores.into_iter().for_each(|(file_path, score)| {
                if let Some(index) = self.find_index_for_file_path(&file_path) {
                    let mut picture = gallery.pictures()[index].clone();
                    picture.increment_score(score);
                    match self.update_picture(&picture) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }
                }
            })
        }
    }

    pub fn find_index_for_file_path(&self, file_path: &str) -> Option<usize> {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            gallery.find_file_path(file_path)
        } else {
            panic!("can'tc borrow")
        }
    }

    pub fn set_tag_selection_criteria(&self, tag_selection_criteria: TagSelectionCriteria) {
        if let Ok(mut gallery) = self.gallery_rc.try_borrow_mut() {
            gallery.set_tag_selection_criteria(tag_selection_criteria.clone());
        } else {
            panic!("can't borrow mut")
        }
    }

    pub fn decrease_folder_picture_count(&self, folder_id: FolderId, count: usize) -> IOResult<()> {
        let (new_picture_count, parent_id) = {
            let mut folder_map = self.folder_map_rc.borrow_mut();
            if let Some(mut folder) = folder_map.folder(folder_id) {
                let directory = folder.file_path();
                folder.decrease_count(count);
                folder_map.update(&directory, &folder);
                (folder.picture_count(), folder.parent_id())
            } else {
                (0, 0)
            }
        };
        if parent_id > 0 {
            let result = if new_picture_count > 0 {
                self.database
                    .update_folder_picture_count_for_id(folder_id, new_picture_count)
            } else {
                self.database.delete_folder_with_id(folder_id)
            };
            match result {
                Ok(_) => self.decrease_folder_picture_count(parent_id, count),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    pub fn increase_folder_picture_count(&self, folder_id: FolderId, count: usize) -> IOResult<()> {
        let (new_picture_count, parent_id) = {
            let mut folder_map = self.folder_map_rc.borrow_mut();
            if let Some(mut folder) = folder_map.folder(folder_id) {
                let directory = folder.file_path();
                folder.increase_count(count);
                folder_map.update(&directory, &folder);
                (folder.picture_count(), folder.parent_id())
            } else {
                (0, 0)
            }
        };
        if parent_id > 0 {
            match self
                .database
                .update_folder_picture_count_for_id(folder_id, new_picture_count)
            {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        } else {
            Ok(())
        }
    }
    pub fn delete_picture(&self, picture: &Picture) -> IOResult<()> {
        let file_path = picture.file_path();
        if self.command_line_arguments.on_database() {
            self.database
                .delete_picture_with_file_path(&file_path)
                .and_then(|_| match delete_picture_files(&file_path) {
                    Ok(_) => {
                        if let Some(parent_directory) = parent_directory(&file_path) {
                            let folder_opt = {
                                self.folder_map_rc
                                    .borrow()
                                    .get(&file_path_as_stored(&parent_directory))
                            };
                            if let Some(folder) = folder_opt {
                                self.decrease_folder_picture_count(folder.id(), 1)
                            } else {
                                Err(IOError::other("can't access to folder"))
                            }
                        } else {
                            Ok(())
                        }
                    }
                    Err(err) => Err(err),
                })
        } else {
            match delete_picture_files(&file_path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
    }

    pub fn delete_picture_at_index(&self, index: usize) -> IOResult<()> {
        if let Ok(gallery) = self.gallery_rc.try_borrow() {
            let picture = gallery.pictures()[index].clone();
            self.delete_picture(&picture)
        } else {
            panic!("can't borrow mut");
        }
    }

    pub fn list(&self, directory: Option<String>) -> IOResult<()> {
        let result = match directory {
            Some(path) => match self.pictures_in_directory(&path) {
                Ok(gallery) => {
                    gallery.print(false);
                    Ok(())
                }
                Err(e) => Err(e),
            },
            None => match self.gallery_rc.try_borrow() {
                Ok(gallery) => {
                    gallery.print(false);
                    Ok(())
                }
                Err(e) => Err(IOError::other(e)),
            },
        };
        match result {
            Ok(_) => {
                let parent_dirs = self.parent_dirs();
                if !parent_dirs.is_empty() {
                    println!("----- directories:{} -----", parent_dirs.len());
                    let mut dirs: Vec<String> = vec![];
                    for dir in parent_dirs.keys() {
                        dirs.push(dir.to_string());
                    }
                    dirs.sort();
                    for dir in dirs {
                        let counts = parent_dirs.get(&dir).unwrap();
                        let count = counts.0;
                        let covers = counts.1;
                        println!("{}:  {}({})", dir, count, covers)
                    }
                };
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn check(&self) -> IOResult<()> {
        match self.gallery_rc.try_borrow() {
            Ok(gallery) => {
                println!("checking pictures where picture file does not exists");
                for picture in gallery.pictures() {
                    if !file_exists(&picture.file_path()) {
                        println!("{}", picture.file_path());
                    }
                }
                Ok(())
            }
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn clean(&self) -> IOResult<()> {
        match self.gallery_rc.try_borrow() {
            Ok(gallery) => {
                println!("deleting picture data where picture file does not exists");
                let mut count: usize = 0;
                let mut deleted: usize = 0;
                let total = gallery.len();
                for picture in gallery.pictures() {
                    if !file_exists(&picture.file_path()) {
                        match self
                            .database
                            .delete_picture_with_file_path(&picture.file_path())
                        {
                            Ok(_) => {
                                println!("deleted from database: {}", picture.file_path());
                                deleted += 1;
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                    }
                    count += 1;
                    println!("{}/{}…", count, total);
                }
                println!("{} picture records deleted", deleted);
                Ok(())
            }
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn move_pictures(&self, source_dir: &str, target_dir: &str) -> IOResult<()> {
        match self.database.select_all_pictures_with_parent(source_dir) {
            Ok(pictures) => {
                let mut count = 0;
                for picture in &pictures {
                    println!("moving {} to {}", picture.file_path(), target_dir);
                    let operations = move_picture(&picture.file_path(), target_dir);
                    match execute(&self.database, &operations) {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    };
                    count += 1;
                }
                println!(
                    "{} pictures moved from {} to {}",
                    count, source_dir, target_dir
                );
                Ok(())
            }
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn update_folder_first_file_path(
        &self,
        parent_dir: &str,
        file_path: &str,
    ) -> IOResult<usize> {
        let folder = {
            let folders = self.folder_map_rc.borrow();
            match folders.get(parent_dir) {
                Some(folder) => folder,
                None => return Ok(0),
            }
        };
        {
            let mut new_folder = folder.clone();
            new_folder.set_first_file_path(&file_path);
            let mut folder_map = self.folder_map_rc.borrow_mut();
            folder_map.update(parent_dir, &new_folder);
            self.database.update_folder_first_file_path_for_id(
                new_folder.id(),
                &new_folder.first_file_path(),
            )
        }
    }

    pub fn update_former_cover_picture(&self, picture: &Picture) -> IOResult<Vec<String>> {
        if let Some(parent_dir) = parent_directory(&picture.file_path()) {
            let directory = file_path_as_stored(&parent_dir);
            let file_path = file_path_as_stored(&picture.file_path());
            let mut paths: Vec<String> = Vec::new();
            match self.database.select_all_pictures_with_parent(&directory) {
                Ok(pictures) => {
                    for picture in pictures.into_iter() {
                        if picture.is_cover()
                            && file_path_as_stored(&picture.file_path()) != file_path
                        {
                            let mut new_picture = picture.clone();
                            let mut image_data = picture.image_data().expect("image data not set");
                            image_data.set_cover_off();
                            new_picture.set_image_data(image_data);
                            match self.database.update_picture_is_cover(&new_picture) {
                                Ok(_) => {}
                                Err(e) => eprintln!("Error:{}", e),
                            }
                            let path = file_path_as_stored(&picture.file_path());
                            {
                                let mut gallery = self.gallery_rc.borrow_mut();
                                let index_opt = gallery.position_with_stored_file_path(&path);
                                if let Some(index) = index_opt {
                                    let mut picture = gallery.picture(index);
                                    picture.toggle_cover(0);
                                    gallery.set_picture(index, picture);
                                }
                            }
                            paths.push(path);
                        }
                    }
                    Ok(paths)
                }
                Err(e) => Err(IOError::other(e)),
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub fn retrieve_catalog(&self) -> IOResult<Catalog> {
        match self.database.select_catalog() {
            Ok(s_expression) => Catalog::from_s_expression(&s_expression),
            Err(e) => {
                println!("empty catalog; launch   ctlg import to import one");
                self.database.update_catalog("(-)");
                self.retrieve_catalog()
            }
        }
    }

    pub fn modify_pictures_at_indices<F>(&self, indices: &Vec<usize>, mut f: F) -> IOResult<usize>
    where
        F: FnMut(&mut Picture),
    {
        let mut gallery = self.gallery_rc.borrow_mut();
        let mut result = Ok(indices.len());
        for position in indices {
            let mut picture = gallery.picture(*position);
            f(&mut picture);
            gallery.set_picture(*position, picture.clone());
            match self.update_picture(&picture) {
                Ok(_) => {}
                Err(e) => {
                    result = Err(e);
                }
            };
        }
        result
    }

    pub fn set_picture_cover_at_current_position(&self) -> IOResult<Vec<String>> {
        let position = {
            let gallery = self.gallery_rc.borrow();
            gallery.current_picture_index()
        };
        let directory_count = self.directory_count_at_index(position);
        let picture = {
            let mut gallery = self.gallery_rc.borrow_mut();
            let mut picture = gallery.current_picture().clone();
            picture.toggle_cover(directory_count);
            gallery.set_picture(position, picture.clone());
            picture.clone()
        };
        self.update_picture(&picture)
            .and_then(|_| self.update_former_cover_picture(&picture))
    }

    pub fn set_picture_cover_at_position(&self, position: usize) -> IOResult<()> {
        let count = self.directory_count_at_index(position);
        let mut gallery = self.gallery_rc.borrow_mut();
        let mut picture = gallery.current_picture().clone();
        picture.toggle_cover(count);
        gallery.set_picture(position, picture.clone());
        self.update_picture(&picture)
    }

    pub fn update_picture(&self, picture: &Picture) -> IOResult<()> {
        if self.command_line_arguments.on_database() {
            if picture.is_cover() {
                if let Some(parent_dir) = parent_directory(&picture.file_path()) {
                    let _ = self.update_folder_first_file_path(
                        &file_path_as_stored(&parent_dir),
                        &file_path_as_stored(&picture.file_path()),
                    );
                }
            }
            match self.database.update_picture(picture) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Err(IOError::other("can't modify picture: not on  database"))
        }
    }

    pub fn rename_picture(&self, picture: &Picture, target_name: &str) -> IOResult<usize> {
        let operations = rename_picture(&picture.file_path(), target_name);
        if operations.is_empty() {
            println!(
                "no operation for rename of {} to {}",
                picture.file_path(),
                target_name
            );
            Ok(0)
        } else {
            let count = operations.len();
            match execute(&self.database, &operations) {
                Ok(_) => Ok(count),
                Err(err) => Err(err),
            }
        }
    }
    pub fn rename_picture_at_index(&self, index: usize, target_name: &str) -> IOResult<usize> {
        match self.gallery_rc().try_borrow() {
            Ok(gallery) => {
                let picture = gallery.picture(index);
                self.rename_picture(&picture, target_name)
            }
            Err(e) => Err(IOError::other(e)),
        }
    }
    pub fn update_picture_parent_id(
        &self,
        picture: &Picture,
        parent_id: FolderId,
    ) -> IOResult<usize> {
        Ok(0)
    }
    pub fn move_picture_to_target(&self, picture: &Picture, target_dir: &str) -> IOResult<usize> {
        let operations = move_picture(&picture.file_path(), target_dir);
        if operations.is_empty() {
            println!(
                "no operation for move of {} to {}",
                picture.file_path(),
                target_dir
            );
            Ok(0)
        } else {
            let count = operations.len();
            match execute(&self.database, &operations) {
                Ok(_) => {
                    if let Some(parent_directory) = &parent_directory(&picture.file_path()) {
                        let source_directory = file_path_as_stored(&parent_directory);
                        let folder_opt = {
                            self.folder_map_rc
                                .borrow()
                                .get(&file_path_as_stored(&source_directory))
                        };
                        if let Some(folder) = folder_opt {
                            self.decrease_folder_picture_count(folder.id(), 1)
                                .and_then(|_| Ok(count))
                        } else {
                            Err(IOError::other("can't access to folder"))
                        };
                        let target_directory = file_path_as_stored(target_dir);
                        let folder_opt = {
                            self.folder_map_rc
                                .borrow()
                                .get(&file_path_as_stored(&target_directory))
                        };
                        if let Some(folder) = folder_opt {
                            self.update_picture_parent_id(&picture, folder.id())
                                .and_then(|_| {
                                    self.increase_folder_picture_count(folder.id(), 1)
                                        .and_then(|_| Ok(count))
                                })
                        } else {
                            Err(IOError::other("can't access to folder"))
                        }
                    } else {
                        Ok(count)
                    }
                }
                Err(err) => Err(err),
            }
        }
    }

    pub fn copy_picture_at_index_to_temp_dir(&self, index: usize) -> IOResult<()> {
        match self.gallery_rc().try_borrow() {
            Ok(gallery) => {
                let picture = gallery.picture(index);
                let temp_dir = &CONFIGURATION.get().expect("configuration not set").temp_dir;
                println!("copying {} to {}", &picture.file_path(), temp_dir);
                copy_picture_file_to_directory(&picture.file_path(), temp_dir)
            }
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn extract_file_names(
        &self,
        indexes: &Vec<usize>,
        extraction_file_path: &str,
    ) -> IOResult<String> {
        let mut lines: Vec<String> = vec![];
        match self.gallery_rc().try_borrow() {
            Ok(gallery) => {
                for index in indexes {
                    let picture = &gallery.picture(*index);
                    lines.push(picture.file_path());
                }
                let file = File::create(extraction_file_path)?;
                let message = format!(
                    "copied {} file names to {}",
                    lines.len(),
                    extraction_file_path
                );
                let mut writer = BufWriter::new(file);
                for line in lines {
                    writer.write_all(line.as_bytes())?;
                    writer.write_all(b"\n")?;
                }
                writer.flush()?;
                Ok(message)
            }
            Err(e) => Err(IOError::other(e)),
        }
    }

    pub fn extract_all_file_names(&self, extraction_file: Option<String>) -> IOResult<()> {
        let extract_file = if let Some(file_name) = extraction_file {
            file_name
        } else {
            timestamp_filename("selection", "txt")
        };
        let mut lines: Vec<String> = vec![];
        match self.gallery_rc().try_borrow() {
            Ok(gallery) => {
                for index in 0..self.len() {
                    let picture = &gallery.picture(index);
                    lines.push(picture.file_path());
                }
                let path: PathBuf = PathBuf::from(&extract_file);
                println!("copying {} file names to {}", lines.len(), path.display());
                let file = File::create(path)?;
                let mut writer = BufWriter::new(file);
                for line in lines {
                    writer.write_all(line.as_bytes())?;
                    writer.write_all(b"\n")?;
                }
                writer.flush()?;
                Ok(())
            }
            Err(e) => Err(IOError::other(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::configuration::tests::my_cfg;
    use crate::file::database::tests::my_args;
    use crate::file::paths::test::current_directory;
    use crate::model::order::Order;
    use crate::test_data::SINGLE_DOT;
    use crate::test_data::TEST_DATA_DIR;
    use crate::test_data::WHITE_SQUARE;
    use serial_test::serial;

    #[test]
    #[serial]
    fn given_a_db_once_initialized_it_provides_the_set_of_all_labels() {
        let args = my_args().expect("can't access to test args");
        let repository = Repository::new(args, false);
        repository
            .retrieve_pictures(None)
            .expect("can't initialize");
        assert!(repository.all_labels().contains("a_rather_long_tag"));
        assert!(repository.all_labels().contains("white_square"));
    }

    #[test]
    #[serial]
    fn after_adding_a_label_the_set_includes_this_label() {
        let args = my_args().expect("can't access to test args");
        let repository = Repository::new(args, false);
        repository
            .retrieve_pictures(None)
            .expect("can't initialize");
        assert!(!repository.all_labels().contains("a-new-label"));
        repository.add_label("a-new-label");
        assert!(repository.all_labels().contains("a-new-label"));
    }

    #[test]
    #[serial]
    fn given_initial_args_it_provides_the_gallery_of_all_picture_matching_the_args() {
        let mut args = my_args().expect("can't access to test args");
        args.order = Some(Order::Size);
        let repository = Repository::new(args.clone(), false);
        repository
            .retrieve_pictures(None)
            .expect("can't initialize repository");
        let gallery_rc = repository.gallery_rc();
        let gallery = gallery_rc
            .try_borrow()
            .expect("can't borrow repository gallery");
        assert_eq!(4, gallery.len());
        dbg!(&args.order);
        assert!(gallery.picture(0).file_size() <= gallery.picture(1).file_size());
        assert!(gallery.picture(1).file_size() <= gallery.picture(2).file_size());
        assert!(gallery.picture(2).file_size() <= gallery.picture(3).file_size());
    }
    #[test]
    #[serial]
    fn given_a_dir_it_provides_the_gallery_of_pictures_with_only_size_and_modified_time() {
        let mut args = my_args().expect("can't access to test args");
        args.order = Some(Order::Size);
        let repository = Repository::new(args, false);
        assert!(repository.retrieve_pictures(None).is_ok());
        let result = repository.pictures_in_directory("testdata");
        assert!(result.is_ok());
        let gallery = result.unwrap();
        assert_eq!(4, gallery.len());
    }
    #[test]
    #[serial]
    fn given_a_file_path_it_provides_the_picture_with_only_size_and_modified_time() {
        let mut args = my_args().expect("can't access to test args");
        args.order = Some(Order::Size);
        let repository = Repository::new(args, false);
        assert!(repository.retrieve_pictures(None).is_ok());
        let result = repository.picture_from_file_path(&format!("testdata/{}", WHITE_SQUARE));
        assert!(result.is_ok());
        let gallery = result.unwrap();
        assert_eq!(1, gallery.len());
        assert!(gallery.pictures()[0].file_size() > Some(0));
    }
    #[test]
    #[serial]
    fn given_a_restriction_in_initial_args_it_provides_only_the_matching_pictures() {
        let mut args = my_args().expect("can't access to test args");
        args.restrict = Some("foo,bar".to_string());
        let repository = Repository::new(args.clone(), false);
        assert!(repository.retrieve_pictures(None).is_ok());
        let gallery_rc = repository.gallery_rc();
        let gallery = gallery_rc
            .try_borrow()
            .expect("can't borrow repository gallery");
        assert_eq!(2, gallery.len()); // only 2 pics have both bar and foo tags, see sql/update_test_data.sql 

        args.restrict = None;
        args.label = Some("dot".to_string());
        let repository = Repository::new(args.clone(), false);
        assert!(repository.retrieve_pictures(None).is_ok());
        let gallery_rc = repository.gallery_rc();
        let gallery = gallery_rc
            .try_borrow()
            .expect("can't borrow repository gallery");
        assert_eq!(1, gallery.len()); // only 1 pic has label "dot"
        args.label = None;
        args.covers = true;
        let repository = Repository::new(args.clone(), false);
        assert!(repository.retrieve_pictures(None).is_ok());
        let gallery_rc = repository.gallery_rc();
        let gallery = gallery_rc
            .try_borrow()
            .expect("can't borrow repository gallery");
        assert_eq!(1, gallery.len()); // only 1 pic is cover
        assert!(gallery.pictures()[0].file_path().contains(SINGLE_DOT));
    }
    #[test]
    #[serial]
    fn a_picture_that_is_a_cover_has_the_len_of_its_parent_dir() {
        let args = my_args().expect("can't access to test args");
        let repository = Repository::new(args.clone(), false);
        assert!(repository.retrieve_pictures(None).is_ok());
        let gallery_rc = repository.gallery_rc();
        let gallery = gallery_rc
            .try_borrow()
            .expect("can't borrow repository gallery");
        let cover_picture = gallery.pictures()[1].clone();
        assert!(cover_picture.file_path().contains(SINGLE_DOT));
        assert!(cover_picture.cover().is_some());
        let count = cover_picture.cover().unwrap();
        assert_eq!(3, count); // because NINE_COLORS is in a subdir
    }
    #[test]
    #[serial]
    fn provides_the_list_of_all_parent_dirs() {
        let args = my_args().expect("can't access to test args");
        let repository = Repository::new(args.clone(), false);
        assert!(repository.retrieve_pictures(None).is_ok());
        let map = repository.parent_dirs();
        dbg!(&map);
        let directory = file_path_as_stored(&format!("{}/{}", current_directory(), TEST_DATA_DIR));
        dbg!(&directory);
        let counts: (usize, usize) = *map.get(&directory).expect("can't access parent dir count");
        assert_eq!((3, 1), counts);
    }
    #[test]
    #[serial]
    fn can_tell_if_selection_has_covers() {
        let args = my_args().expect("can't access to test args");
        let repository = Repository::new(args.clone(), false);
        assert!(repository.retrieve_pictures(None).is_ok());
        assert_eq!(1, repository.covers());
    }
}
