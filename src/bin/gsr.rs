use gsr::cli::command::Command;
use gsr::cli::command::execute_command;
use gsr::cli::command_line_arguments::CommandLineArguments;
use gsr::cli::status::Status;
use gsr::env::configuration::Configuration;
use gsr::env::default_values::APPLICATION_ID;
use gsr::file::database::Database;
use gsr::file::paths::file_exists;
use gsr::gui::controller::Controller;
use gsr::gui::controller::RcController;
use gsr::gui::gsr_application::GsrApplication;
use gsr::gui::main_controller::MainController;
use gsr::gui::view::application::make_gsr_application;
use gsr::gui::view::main_window::MainWindow;
use gtk::glib::clone;
use gtk::prelude::ApplicationExt;
use std::cell::RefCell;
use std::io::Error as IOError;
use std::process::exit;
use std::rc::Rc;

fn main() {
    let config = match Configuration::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };
    let result = CommandLineArguments::parse_and_check(None, &config).and_then(|cli| {
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
            let result = Controller::new(config.clone(), args.clone()).and_then(|controller| {
                let repository = controller.repository();
                let controller_rc: RcController = Rc::new(RefCell::new(controller));

                {
                    let mut controller = controller_rc.borrow_mut();
                }
                let result = execute_command(args.clone(), repository, config.clone());
                if let Ok(Status::Ready(index)) = result {
                    build_and_run_app(args, controller_rc, index);
                    Ok(Status::Done)
                } else {
                    result
                }
            });
            match result {
                Ok(Status::Done) | Ok(Status::Exit) | Ok(Status::Ready(_)) => exit(0),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1)
                }
            }
        }
    });
    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}

fn build_and_run_app(clargs: CommandLineArguments, controller_rc: RcController, position: usize) {
    let main_controller: MainController = MainController::new(Some(controller_rc.clone()));
    let gsr_application: GsrApplication = make_gsr_application(APPLICATION_ID, main_controller);
    // application.insert_action_group("main-controller", Some(main_controller.actions()));
    gsr_application.connect_activation(clargs, position);
    MainWindow::run_application(gsr_application);
}
