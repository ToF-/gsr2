use crate::command::Command;
use crate::command_line_interface::CommandLineInterface;
use crate::gui::controller::Controller;
use std::process::exit;

mod command;
mod command_line_interface;
mod control;
mod database;
mod default_values;
mod direction;
mod display;
mod editor;
mod environment;
mod file_system;
mod gallery;
mod gen_image;
mod gui;
mod image_data;
mod order;
mod paths;
mod picture;

fn main() {
    match CommandLineInterface::parse_and_check(None) {
        Ok(cli) => {
            if let Some(Command::File { ref file_path }) = cli.command {
                println!("viewing file {}", file_path);
            } else if let Some(Command::Dir { ref directory }) = cli.command {
                println!("viewing files in directory {}", directory);
            } else if cli.command.is_none() {
                println!("viewing file from the database");
            }
            let result = Controller::new(cli.clone())
                .and_then(|controller| Controller::build_and_run_app(controller));
            match result {
                Ok(_) => {
                    exit(0);
                }
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
    };
}
