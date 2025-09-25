use crate::command::Command;
use crate::command_line_interface::CommandLineInterface;
use std::process::exit;
use crate::gui::controller::Controller;

mod application_state;
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
mod navigator;
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
            match Controller::new(cli.clone()) {
                Ok(controller) => controller.build_and_run_app(),
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
    exit(0);
}
