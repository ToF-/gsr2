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
    pub fn parse_and_check(args_opt: Option<Vec<&str>>) -> Result<Self> {
        let cli:Self = match args_opt {
            Some(args) => Self::parse_from(args),
            None => Self::parse(),
        };
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
    use crate::gen_image::{gen_single_dot, SINGLE_DOT};

    #[test]
    fn command_line_interface_with_specified_directory() {
        let args = vec!["gsr", "testdata"];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        assert_eq!(Some(String::from("testdata")), cli.directory);
    }

    #[test]
    fn command_line_interface_with_no_specified_directory() {
        let args = vec!["gsr"];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        assert_eq!(None, cli.directory);
    }

    #[test]
    fn command_line_interface_with_command_file_with_adequate_argument() {
        gen_single_dot();
        let args = vec!["gsr", "file", SINGLE_DOT];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        if let Some(File { file_name }) = cli.command {
            assert_eq!(String::from(SINGLE_DOT), file_name);
        } else {
            assert!(false)
        }
    }

    fn command_line_interface_with_command_file_with_non_existing_file() {
        let args = vec!["gsr", "file", "not_existing.png"];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        if let Some(File { file_name }) = cli.command {
            assert_eq!(String::from(SINGLE_DOT), file_name);
        } else {
            assert!(false)
        }
    }

}
