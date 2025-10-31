use crate::cli::args::Args;
use crate::cli::command::Command;
use crate::cli::status::Status;
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
use std::io::Error as IOError;
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
    let result = Args::parse_and_check(None, &config).and_then(|cli| {
        let args = cli.clone();
        if let Some(Command::Initialize) = args.clone().command {
            if !file_exists(&config.database_file) {
                println!("creating new database file {}", config.database_file);
                match Database::from_connection(&config.database_file, true) {
                    Ok(database) => match database.rusqlite_create_schema() {
                        Ok(_) => Ok(Status::Done),
                        Err(e) => Err(IOError::other(e)),
                    },
                    Err(e) => Err(e),
                }
            } else {
                Err(IOError::other(format!(
                    "{} already exists",
                    &config.database_file
                )))
            }
        } else {
            Controller::new(config.clone(), args.clone()).and_then(|controller| {
                let controller_rc: RcController = Rc::new(RefCell::new(controller));
                let result = match controller_rc.try_borrow_mut() {
                    Ok(mut controller) => match controller.execute_command() {
                        Err(e) => Err(IOError::other(e)),
                        other => other,
                    },
                    Err(e) => Err(IOError::other(e)),
                };
                if let Ok(Status::Ready) = result {
                    build_and_run_app(args, controller_rc);
                    Ok(Status::Done)
                } else {
                    result
                }
            })
        }
    });
    match result {
        Ok(Status::Done) | Ok(Status::Exit) | Ok(Status::Ready) => exit(0),
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}

fn build_and_run_app(args: Args, controller_rc: RcController) {
    let application: gtk::Application = make_application(APPLICATION_ID);
    application.connect_activate(clone!(
        #[strong]
        args,
        #[strong]
        controller_rc,
        move |application: &gtk::Application| {
            MainWindow::activate(application, &args, &controller_rc)
        }
    ));
    MainWindow::run_application(application);
}
