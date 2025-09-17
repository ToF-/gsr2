use crate::command::Command;
use crate::command_line_interface::CommandLineInterface;
use crate::graphic_user_interface::build_and_run_application;
use std::process::exit;

mod application_state;
mod command;
mod command_line_interface;
mod control;
mod default_values;
mod direction;
mod display;
mod file_system;
mod gallery;
mod gen_image;
mod graphic_user_interface;
mod image_data;
mod navigator;
mod paths;
mod picture;

fn main() {
    match CommandLineInterface::parse_and_check(None) {
        Ok(cli) => {
            if let Some(Command::File { ref file_path }) = cli.command {
                println!("viewing file {}", file_path);
                build_and_run_application(cli.clone())
            } else if let Some(Command::Dir { ref directory }) = cli.command {
                println!("viewing files in directory {}", directory);
                build_and_run_application(cli.clone())
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    exit(0);
}
