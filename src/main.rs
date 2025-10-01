use crate::cli::command::Command;
use crate::cli::args::Args;
use crate::gui::controller::Controller;
use std::process::exit;

mod cli;
mod database;
mod default_values;
mod dimension;
mod display;
mod editor;
mod environment;
mod file;
mod gen_image;
mod gui;
mod model;
mod paths;

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
