use crate::Args;
use crate::Configuration;
use crate::IOError;
use crate::Status;
use crate::file::paths::check_collectable;
use crate::file::picture_file::create_missing_thumbnails;
use crate::file_exists;
use crate::model::gallery::Gallery;
use crate::model::repository::Repository;
use clap::Subcommand;
use std::io::Result as IOResult;
use std::path::PathBuf;

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
    args: Args,
    repository: Repository,
    config: Configuration,
) -> IOResult<Status> {
    let mut gallery = Gallery::new();
    match args.command {
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
        Some(Command::List { directory }) => match repository.list(directory) {
            Ok(_) => Ok(Status::Done),
            Err(err) => Err(err),
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
                println!("{} pictures", count);
                Ok(Status::Ready(0))
            }
        },
        None => match repository.gallery_rc().try_borrow_mut() {
            Ok(gallery) => {
                if gallery.is_empty() {
                    println!("no pictures for this selection");
                    Ok(Status::Exit)
                } else {
                    println!("{} pictures", &gallery.len());
                    if let Some(initial_position) = args.index {
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
