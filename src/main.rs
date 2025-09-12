use std::process::exit;
use crate::command_line_interface::CommandLineInterface;

mod command_line_interface;
mod paths;

fn main() {
    match CommandLineInterface::parse_and_check() {
        Ok(cli) => match cli.directory {
            Some(dir) => println!("directory: {}", dir),
            None => println!("no directory supplied"),
        },
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    };
    exit(0);
}
