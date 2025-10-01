use clap::Subcommand;

#[derive(Subcommand, Clone, Debug, PartialEq)]
/// Command
pub enum Command {
    /// <FILE_PATH> view the individual picture file_path
    File {
        #[arg(value_name = "FILE_PATH")]
        file_path: String,
    },
    /// <DIRECTORY> view the pictures in directory
    Dir {
        #[arg(value_name = "DIRECTORY")]
        directory: String,
    },
}
