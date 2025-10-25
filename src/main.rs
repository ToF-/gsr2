use crate::cli::args::Args;
use crate::cli::command::Command;
use crate::env::configuration::get_configuration;
use crate::env::default_values::APPLICATION_ID;
use crate::file::database::Database;
use crate::file::paths::file_exists;
use crate::gui::controller::Controller;
use crate::gui::controller::RcController;
use crate::gui::view::application::make_application;
use crate::gui::view::main_window::MainWindow;
use gtk::glib::clone;
use gtk::prelude::ApplicationExt;
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

mod cli;
mod env;
mod file;
mod gui;
mod model;
mod test_data;

fn main() {
    let config = match get_configuration() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };
    match Args::parse_and_check(None, &config) {
        Ok(cli) => {
            if let Some(Command::File { ref file_path }) = cli.command {
                println!("viewing file {}", file_path);
            } else if let Some(Command::Dir { ref directory }) = cli.command {
                println!("viewing files in directory {}", directory);
            } else if let Some(Command::Initialize) = cli.command {
                if !file_exists(&config.database_file) {
                    println!("creating new database file {}", config.database_file);
                    match Database::from_connection(&config.database_file, true) {
                        Ok(database) => match database.rusqlite_create_schema() {
                            Ok(_) => {
                                exit(0);
                            }
                            Err(err) => {
                                eprintln!("{}", err);
                                exit(1)
                            }
                        },
                        Err(err) => {
                            eprintln!("{}", err);
                            exit(1)
                        }
                    }
                } else {
                    eprintln!("{} already exists", &config.database_file);
                    exit(1);
                }
            } else if cli.command.is_none() {
                println!("viewing file from the database");
            }
            let controller_result = Controller::new(cli.clone());
            let controller_rc: RcController = match controller_result {
                Ok(controller) => Rc::new(RefCell::new(controller)),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            };
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                match controller.load_picture_data() {
                    Ok(0) => exit(0),
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                    Ok(_) => {}
                }
            };
            let application: gtk::Application = make_application(APPLICATION_ID);
            application.connect_activate(clone!(
                #[strong]
                cli,
                #[strong]
                controller_rc,
                move |application: &gtk::Application| {
                    MainWindow::activate(application, &cli, &controller_rc)
                }
            ));
            MainWindow::run_application(application);
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
