use clap::{Parser};
use std::io::Result;

#[derive(Parser, Clone, Debug)]
/// Gallery Show
pub struct CommandLineInterface {
     /// Directory to search
    pub directory: Option<String>,
}

impl CommandLineInterface {
    pub fn parse_and_check() -> Result<Self> {
        let cli = Self::parse();
        Ok(cli.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_interface_with_specified_directory() {
        let args = vec!["gsr","foo"];
        let cli = CommandLineInterface::parse_from(args);
        assert_eq!(Some(String::from("foo")), cli.directory);
    }

    #[test]
    fn command_line_interface_with_no_specified_directory() {
        let args = vec!["gsr"];
        let cli = CommandLineInterface::parse_from(args);
        assert_eq!(None, cli.directory);
    }

}

