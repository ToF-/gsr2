use crate::cli::status::Status;
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
            let controller_result = Controller::new(cli.clone());
            let controller_rc: RcController = match controller_result {
                Ok(controller) => Rc::new(RefCell::new(controller)),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            };
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                match controller.execute_command() {
                    Ok(Status::Exit) => exit(0),
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                    Ok(_) => {}
                }
            };
            build_and_run_app(cli, controller_rc);
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
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
