use clap::Subcommand;

#[derive(Subcommand, Clone, Debug, PartialEq)]
/// Command
pub enum Command {
    /// <FILE_NAME> view the individual picture FILE_NAME
    File {
        #[arg(value_name = "FILE_NAME")]
        file_name: String,
    },
    /// <DIRECTORY> view the pictures in director DIRECTORY
    Dir {
        #[arg(value_name = "DIRECTORY")]
        directory: String,
    },
}
