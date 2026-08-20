use gsr::cli::command::Command;
use gsr::cli::command::execute_command;
use gsr::cli::command_line_arguments::CommandLineArguments;
use gsr::cli::status::Status;
use gsr::env::configuration::Configuration;
use gsr::file::database::Database;
use gsr::gui::controller::Controller;
use gsr::gui::objects::gsr_application::GsrApplication;
use gtk::gio;
use gtk::prelude::ApplicationExtManual;
use std::io::Error as IOError;
use std::io::Result;
use std::process::exit;

pub fn error_exit(error: &IOError) {
    eprintln!("{}", error);
    exit(1)
}

fn main() {
    gio::resources_register_include!("gsr.gresource").expect("Failed to register resources");

    let config_result = Configuration::from_env();
    if config_result.is_err() {
        error_exit(config_result.as_ref().err().unwrap());
    }
    let config = config_result.unwrap();
    let app_result = CommandLineArguments::parse_and_check(None, &config).and_then(|clargs| {
        if let Some(Command::Initialize) = clargs.clone().command {
            Database::initialize(&config)
        } else {
            run_application(&config, &clargs)
        }
    });
    if app_result.is_err() {
        error_exit(app_result.as_ref().err().unwrap());
    }
}

fn run_application(config: &Configuration, clargs: &CommandLineArguments) -> Result<Status> {
    let result = Controller::new(config.clone(), clargs.clone()).and_then(|controller| {
        let repository = controller.repository();
        let result = execute_command(clargs.clone(), repository.clone(), config.clone());
        if let Ok(Status::Ready(initial_position)) = result {
            let gallery = repository.gallery_rc().borrow();
            println!("{} picture(s)", gallery.len());
            build_and_run_app(clargs, controller);
            Ok(Status::Done)
        } else {
            result
        }
    });
    if result.is_err() {
        error_exit(result.as_ref().err().unwrap());
    }
    exit(0)
}

fn build_and_run_app(clargs: &CommandLineArguments, controller: Controller) {
    let gsr_application = GsrApplication::default();
    gsr_application.set_state(clargs.clone(), controller);
    let no_args: Vec<String> = vec![];
    gsr_application.run_with_args(&no_args);
}
