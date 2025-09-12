use crate::command_line_interface::CommandLineInterface;
use clap::Parser;

mod command_line_interface;

fn main() {
    let cli = CommandLineInterface::parse_and_check().unwrap();
    match cli.directory {
        Some(dir) => println!("directory: {}", dir),
        None => println!("no directory supplied"),
    }
}
