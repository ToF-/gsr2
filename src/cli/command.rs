use clap::Subcommand;
use std::io::Result as IOResult;
use std::path::PathBuf;

use crate::cli::command_line_arguments::CommandLineArguments;
use crate::cli::status::Status;
use crate::env::configuration::Configuration;
use crate::file::paths::{check_collectable, file_exists};
use crate::file::picture_file::create_missing_thumbnails;
use crate::model::gallery::Gallery;
use crate::model::repository::Repository;
use std::io::Error as IOError;

#[derive(Subcommand, Clone, Debug, PartialEq)]
/// Command
pub enum Command {
    /// <FILE_PATH> display the picture
    File {
        #[arg(value_name = "FILE_PATH")]
        file_path: String,
    },

    /// <DIRECTORY> display pictures in directory
    Directory {
        #[arg(value_name = "DIRECTORY")]
        directory: String,
    },

    /// <DIRECTORY> collect data from the pictures in directory
    Collect {
        #[arg(value_name = "DIRECTORY")]
        directory: String,
    },

    /// <FILE_NAME> extract names matching the selection to FILE_NAME
    Extract {
        #[arg(value_name = "FILE_NAME")]
        extract_name: String,
    },
    /// initialize a new database set in the config file is not existing
    Initialize,

    /// [DIRECTORY] list the picture file names in the directory or database
    List { directory: Option<String> },

    /// check picture files for pictures in the database
    Check,

    /// remove database entries for wich picture files don't exit
    Clean,

    /// <SOURCE_DIR> <TARGET_DIR> move picture files and data from source to target directory
    Move { source: String, target: String },

    /// <N> create missing thumbnails for grid with N x N pictures per page (N in range [2..10]) and
    /// then quit
    Thumbnails {
        #[arg(value_name = "N", value_parser(clap::value_parser!(u8).range(2..=10)))]
        pictures_per_row: u8,
    },
}

pub fn execute_command(
    command_line_arguments: CommandLineArguments,
    repository: Repository,
    config: Configuration,
) -> IOResult<Status> {
    let mut gallery = Gallery::new();
    match command_line_arguments.command {
        Some(Command::Collect { directory }) => {
            println!("collecting data for picture files in the database…");
            let path: PathBuf = PathBuf::from(directory);
            match check_collectable(&path) {
                Ok(_) => match repository.collect_data() {
                    Ok(_) => Ok(Status::Done),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        }
        Some(Command::Thumbnails { pictures_per_row }) => {
            match repository.gallery_rc().try_borrow() {
                Ok(gallery) => {
                    create_missing_thumbnails(&gallery, pictures_per_row as usize);
                    Ok(Status::Done)
                }
                Err(e) => Err(IOError::other(e)),
            }
        }
        Some(Command::List { directory }) => {
            let result = match directory {
                Some(path) => match repository.pictures_in_directory(&path) {
                    Ok(gallery) => {
                        print_names(&gallery);
                        Ok(())
                    },
                    Err(e) => Err(e),
                },
                None => match repository.gallery_rc().try_borrow() {
                    Ok(gallery) => {
                        gallery.print(false);
                        Ok(())
                    }
                    Err(e) => Err(IOError::other(e)),
                },
            };
            match result {
                Ok(_) => {
                    let parent_dirs = repository.parent_dirs();
                    if ! parent_dirs.is_empty() {
                        let mut dirs: Vec<String> = vec![];
                        for dir in parent_dirs.keys() {
                            dirs.push(dir.to_string());
                        };
                        dirs.sort();
                        for dir in dirs {
                            let counts = parent_dirs.get(&dir).unwrap();
                            let count = counts.0;
                            let covers = counts.1;
                            println!("{}:  {}({})", dir, count, covers)
                        }
                    };
                    Ok(Status::Done)
                },
                Err(e) => Err(IOError::other(e)),
            }
        },
        Some(Command::Extract {
            extract_name: extraction_file,
        }) => match repository.extract_all_file_names(Some(extraction_file)) {
            Ok(_) => Ok(Status::Done),
            Err(err) => Err(err),
        },
        Some(Command::Check) => match repository.check() {
            Ok(_) => Ok(Status::Done),
            Err(err) => Err(err),
        },
        Some(Command::Clean) => match repository.clean() {
            Ok(_) => Ok(Status::Done),
            Err(err) => Err(err),
        },
        Some(Command::Move { source, target }) => {
            match repository.move_pictures(&source, &target) {
                Ok(_) => Ok(Status::Exit),
                Err(err) => Err(err),
            }
        }
        Some(Command::Initialize) => {
            let config = Configuration::from_env()?;
            println!("initializing database");
            if !file_exists(&config.database_file) {
                match Repository::create_database(config) {
                    Ok(_) => Ok(Status::Done),
                    Err(e) => Err(IOError::other(e)),
                }
            } else {
                Err(IOError::other(format!(
                    "{} already exists",
                    &config.database_file
                )))
            }
        }
        Some(Command::File { file_path }) => match gallery.load_from_file_path(&file_path) {
            Err(e) => Err(e),
            Ok(_) => Ok(Status::Ready(0)),
        },
        Some(Command::Directory { directory }) => match gallery.load_from_directory(&directory) {
            Err(e) => Err(e),
            Ok(0) => {
                println!("no pictures for this selection");
                Ok(Status::Exit)
            }
            Ok(count) => {
                println!("directory length: {} pictures", count);
                Ok(Status::Ready(0))
            }
        },
        None => match repository.gallery_rc().try_borrow_mut() {
            Ok(gallery) => {
                if gallery.is_empty() {
                    println!("no pictures for this selection");
                    Ok(Status::Exit)
                } else if command_line_arguments.names {
                    print_names(&gallery);
                    Ok(Status::Exit)
                } else if command_line_arguments.folders {
                    print_folders(&gallery);
                    Ok(Status::Exit)
                } else if command_line_arguments.tags {
                    print_tags(&gallery);
                    Ok(Status::Exit)
                } else {
                    println!("gallery length: {} pictures", &gallery.len());
                    if let Some(initial_position) = command_line_arguments.index {
                        Ok(Status::Ready(initial_position))
                    } else if let Some(file_path) = config.current_picture
                        && let Some(initial_position) = gallery.find_file_path(&file_path)
                    {
                        Ok(Status::Ready(initial_position))
                    } else {
                        Ok(Status::Ready(0))
                    }
                }
            }
            Err(e) => Err(IOError::other(e)),
        },
    }
}

fn print_names(gallery: &Gallery) {
    for picture in gallery.pictures() {
        println!("{}", picture.file_path())
    }
}
fn print_folders(gallery: &Gallery) {
    for (folder, count) in gallery.folders() {
        println!("{:6}  {}", count, folder)
    }
}

fn print_tags(gallery: &Gallery) {
    let mut current_dir: String = String::new();
    for (parent_dir, tag, count) in gallery.tags() {
        if current_dir != parent_dir {
            print!("\n{} {}:{}", parent_dir, tag, count);
            current_dir = parent_dir;
        } else {
            print!(" {}:{}", tag, count);
        }
    }
    println!();
}
