use crate::paths::check_path;
use clap::{Parser, Subcommand};
use std::io::Result;

#[derive(Subcommand, Clone, Debug)]
/// Command
pub enum Command {
    /// <FILE_NAME> view the individual picture FILE_NAME
    File {
        #[arg(value_name = "FILE_NAME")]
        file_name: String,
    },
}

#[derive(Parser, Clone, Debug)]
/// Gallery Show
pub struct CommandLineInterface {
    /// Directory to search
    pub directory: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

impl CommandLineInterface {
    pub fn parse_and_check() -> Result<Self> {
        let cli = Self::parse();
        if let Some(ref directory) = cli.directory {
            match check_path(directory) {
                Ok(_) => Ok(cli.clone()),
                Err(e) => Err(e),
            }
        } else {
            Ok(cli.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_line_interface::Command::File;

    #[test]
    fn command_line_interface_with_specified_directory() {
        let args = vec!["gsr", "foo"];
        let cli = CommandLineInterface::parse_from(args);
        assert_eq!(Some(String::from("foo")), cli.directory);
    }

    #[test]
    fn command_line_interface_with_no_specified_directory() {
        let args = vec!["gsr"];
        let cli = CommandLineInterface::parse_from(args);
        assert_eq!(None, cli.directory);
    }

    #[test]
    fn command_line_interface_with_command_file_with_adequate_argument() {
        let args = vec!["gsr", "file", "fool.jpg"];
        let cli = CommandLineInterface::parse_from(args);
        if let Some(File { file_name }) = cli.command {
            assert_eq!(String::from("fool.jpg"), file_name);
        } else {
            assert!(false)
        }
    }
}
