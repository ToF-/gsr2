use crate::gui::view::components::main_window::MainWindow;
use crate::gui::view::View;
use gtk::prelude::ApplicationExt;
use crate::gui::view::components::application::make_application;
use crate::env::default_values::APPLICATION_ID;
use crate::cli::args::Args;
use crate::cli::command::Command;
use crate::gui::controller::Controller;
use crate::gui::controller::RcController;
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use gtk::glib::clone;

mod cli;
mod env;
mod file;
mod gui;
mod model;

fn main() {
    match Args::parse_and_check(None) {
        Ok(cli) => {
            if let Some(Command::File { ref file_path }) = cli.command {
                println!("viewing file {}", file_path);
            } else if let Some(Command::Dir { ref directory }) = cli.command {
                println!("viewing files in directory {}", directory);
            } else if cli.command.is_none() {
                println!("viewing file from the database");
            }
            let controller_result = Controller::new(cli.clone());
            let controller_rc: RcController;
            match controller_result {
                Ok(controller) => {
                    controller_rc = Rc::new(RefCell::new(controller));
                }
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
            let application: gtk::Application = make_application(APPLICATION_ID);
            application.connect_activate(clone!(
                    #[strong] cli, 
                    #[strong] controller_rc,
                    move |application: &gtk::Application| {
                        MainWindow::activate(application, &cli, &controller_rc)
                    }));
            MainWindow::run_application(application);
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
