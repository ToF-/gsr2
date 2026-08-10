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
use gsr::gui::main_controller::MainController;
use gsr::gui::objects::gsr_application::GsrApplication;
use gsr::gui::objects::gsr_application::make_gsr_application;
use gsr::gui::view::main_view::MainView;
use std::cell::RefCell;
use std::io::Error as IOError;
use std::io::Result;
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
    let result = CommandLineArguments::parse_and_check(None, &config).and_then(|clargs| {
        if let Some(Command::Initialize) = clargs.clone().command {
            initialize_database(&config)
        } else {
            run_application(&config, &clargs)
        }
    });
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    }
}

fn build_and_run_app(clargs: &CommandLineArguments, controller_rc: RcController, position: usize) {
    let main_controller: MainController = MainController::new(Some(controller_rc.clone()));
    let gsr_application: GsrApplication =
        make_gsr_application(APPLICATION_ID, main_controller.clone(), clargs.clone(), position);
    MainView::run_application(gsr_application);
}

fn initialize_database(config: &Configuration) -> Result<Status> {
    let database_file = config.database_file.clone();
    if !file_exists(&database_file) {
        println!("creating new database file {}", &database_file);
        match Database::from_connection(&database_file, true) {
            Ok(database) => match database.rusqlite_create_schema() {
                Ok(_) => Ok(Status::Done),
                Err(err) => Err(IOError::other(err)),
            },
            Err(err) => Err(err),
        }
    } else {
        Err(IOError::other(format!("{} already exists", &database_file)))
    }
}

fn run_application(config: &Configuration, clargs: &CommandLineArguments) -> Result<Status> {
    let result = Controller::new(config.clone(), clargs.clone()).and_then(|controller| {
        let repository = controller.repository();
        let controller_rc: RcController = Rc::new(RefCell::new(controller));
        let result = execute_command(clargs.clone(), repository, config.clone());
        if let Ok(Status::Ready(index)) = result {
            build_and_run_app(clargs, controller_rc, index);
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
