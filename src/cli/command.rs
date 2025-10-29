use clap::Subcommand;

#[derive(Subcommand, Clone, Debug, PartialEq)]
/// Command
pub enum Command {
    /// <FILE_PATH> display the picture
    File {
        #[arg(value_name = "FILE_PATH")]
        file_path: String,
    },

    /// <DIRECTORY> display pictures in directory
    Directory {
        #[arg(value_name = "DIRECTORY")]
        directory: String,
    },

    /// <DIRECTORY> collect data from the pictures in directory
    Collect {
        #[arg(value_name = "DIRECTORY")]
        directory: String,
    },

    /// initialize a new database set in the config file is not existing
    Initialize,

    /// [DIRECTORY] list the picture file names in the directory or database
    List { directory: Option<String> },

    /// check picture files for pictures in the database
    Check,

    /// remove database entries for wich picture files don't exit
    Clean,

    /// <SOURCE_DIR> <TARGET_DIR> move picture files and data from source to target directory
    Move { source: String, target: String },

    /// <N> create missing thumbnails for grid with N x N pictures per page (N in range [2..10]) and
    /// then quit
    Thumbnails {
        #[arg(value_name = "N", value_parser(clap::value_parser!(u8).range(2..=10)))]
        pictures_per_row: u8,
    },


}
