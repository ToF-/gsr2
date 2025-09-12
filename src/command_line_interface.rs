use clap::{Parser};

#[derive(Parser, Clone, Debug)]
/// Gallery Show
pub struct CommandLineInterface {
     /// Directory to search
    pub directory: Option<String>,
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

}

