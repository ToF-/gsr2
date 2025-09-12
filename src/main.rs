use crate::command_line_interface::CommandLineInterface;

mod command_line_interface;
mod paths;

fn main() {
    let cli = CommandLineInterface::parse_and_check().unwrap();
    match cli.directory {
        Some(dir) => println!("directory: {}", dir),
        None => println!("no directory supplied"),
    }
}
