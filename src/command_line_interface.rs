use crate::command::Command;
use crate::paths::{check_path, check_picture_file};
use clap::Parser;
use std::io::{Error, ErrorKind, Result};

#[derive(Parser, Clone, Debug, PartialEq)]
/// Gallery Show
#[command(
    about("a picture viewer from terminal"),
    author("ToF"),
    version,
    infer_long_args = true,
    infer_subcommands = true,
    help_template(
        "\
{before-help}{name} {version} {about} by {author-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"
    )
)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// display N x N pictures per page (N in range [2..10[)
    #[arg(short, long, value_name = "N",value_parser(clap::value_parser!(u8).range(2..=9)))]
    pub grid: Option<u8>,

    /// display 10 x 10 thumbnail pictures per page
    #[arg(short, long, default_value_t = false, conflicts_with("grid"))]
    pub thumbnails: bool,
}

impl CommandLineInterface {
    pub fn parse_and_check(args_opt: Option<Vec<&str>>) -> Result<Self> {
        let cli: Self = match args_opt {
            Some(args) => Self::parse_from(args),
            None => Self::parse(),
        };
        if let Some(Command::File { ref file_path }) = cli.command {
            match check_picture_file(file_path) {
                Ok(_) => {
                    if let Some(grid) = cli.grid {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "option --grid not allowed for File command",
                        ));
                    } else if cli.thumbnails {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "option --thumbnails not allowed for File command",
                        ));
                    } else {
                        return Ok(cli.clone());
                    }
                }

                Err(e) => return Err(e),
            }
        }
        if let Some(Command::Dir { ref directory }) = cli.command {
            match check_path(directory) {
                Ok(_) => return Ok(cli.clone()),
                Err(e) => return Err(e),
            }
        }
        Ok(cli.clone())
    }

    pub fn cells_per_row(&self) -> i32 {
        if let Some(grid) = self.grid {
            grid.into()
        } else if self.thumbnails {
            10
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_line_interface::Command::{Dir, File};
    use crate::gen_image::{SINGLE_DOT, gen_single_dot};
    use std::io::ErrorKind;

    #[test]
    fn command_line_interface_with_command_file_with_adequate_argument() {
        gen_single_dot();
        let args = vec!["gsr", "file", SINGLE_DOT];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        if let Some(File { file_path }) = cli.command {
            assert_eq!(String::from(SINGLE_DOT), file_path);
        } else {
            assert!(false)
        }
    }

    #[test]
    fn command_line_interface_with_command_file_with_non_existing_file() {
        let args = vec!["gsr", "file", "not_existing.png"];
        let cli = CommandLineInterface::parse_and_check(Some(args));
        assert!(cli.is_err());
        let err = cli.expect_err("can't extract error");
        assert_eq!(ErrorKind::NotFound, err.kind());
        assert_eq!("not found: not_existing.png", &err.to_string())
    }

    #[test]
    fn command_line_interface_with_command_file_with_non_file() {
        let args = vec!["gsr", "file", "testdata"];
        let cli = CommandLineInterface::parse_and_check(Some(args));
        assert!(cli.is_err());
        let err = cli.expect_err("can't extract error");
        assert_eq!(ErrorKind::Other, err.kind());
        assert_eq!("testdata is not a file", &err.to_string())
    }
    #[test]
    fn command_line_interface_with_command_file_with_non_jpg_or_png_file() {
        let args = vec!["gsr", "file", "src/paths.rs"];
        let cli = CommandLineInterface::parse_and_check(Some(args));
        assert!(cli.is_err());
        let err = cli.expect_err("can't extract error");
        assert_eq!(ErrorKind::Other, err.kind());
        assert_eq!("src/paths.rs is not a jpg or png file", &err.to_string())
    }

    #[test]
    fn command_line_interface_with_command_directory_and_adequate_argument() {
        let args = vec!["gsr", "dir", "."];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        if let Some(Dir { directory }) = cli.command {
            assert_eq!(String::from("."), directory);
        } else {
            assert!(false)
        }
    }
    #[test]
    fn command_line_interface_dir_command_with_non_existing_specified_directory() {
        let args = vec!["gsr", "dir", "not_existing_dir"];
        let cli = CommandLineInterface::parse_and_check(Some(args));
        assert!(cli.is_err());
        let err = cli.expect_err("can't extract error");
        assert_eq!(ErrorKind::NotFound, err.kind());
        assert_eq!("not found: not_existing_dir", &err.to_string())
    }
    #[test]
    fn command_line_interface_dir_command_with_object_specified_not_directory() {
        let args = vec!["gsr", "dir", "README.md"];
        let cli = CommandLineInterface::parse_and_check(Some(args));
        assert!(cli.is_err());
        let err = cli.expect_err("can't extract error");
        assert_eq!(ErrorKind::NotADirectory, err.kind());
        assert_eq!("README.md is not a directory", &err.to_string())
    }
    #[test]
    fn with_no_grid_or_thumbnail_option_cells_per_row_is_1() {
        let args = vec!["gsr", "dir", "testdata"];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        assert_eq!(1, cli.cells_per_row())
    }
    #[test]
    fn cells_per_row_is_determined_by_grid_option() {
        let args = vec!["gsr", "--grid", "5", "dir", "testdata"];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        assert_eq!(5, cli.cells_per_row())
    }
    #[test]
    fn cells_per_row_is_determined_by_thumbnails_option() {
        let args = vec!["gsr", "--thumbnails", "dir", "testdata"];
        let cli = CommandLineInterface::parse_and_check(Some(args)).unwrap();
        assert_eq!(10, cli.cells_per_row())
    }
}
