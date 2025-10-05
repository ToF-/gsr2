use crate::cli::args::Args;
use crate::cli::command::Command;
use crate::gui::controller::Controller;
use crate::gui::controller::RcController;
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

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
            { 
                match controller_rc.try_borrow_mut() {
                    Ok(mut controller) => controller.create_view(&cli, &controller_rc),
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                }
            }
            match controller_rc.try_borrow() {
                Ok(controller) => controller.run_application(),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
