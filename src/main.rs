use crate::command_line_interface::Command::File;
use crate::command_line_interface::CommandLineInterface;
use crate::graphic_user_interface::launch_application;
use std::process::exit;

mod command_line_interface;
mod graphic_user_interface;
mod gen_image;
mod paths;

fn main() {
    match CommandLineInterface::parse_and_check(None) {
        Ok(cli) => {
            match cli.directory {
                Some(ref dir) => println!("directory: {}", dir),
                None => println!("no directory supplied"),
            };
            if let Some(File { ref file_name }) = cli.command {
                println!("viewing file {}", file_name);
                launch_application(cli.clone())
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    exit(0);
}
