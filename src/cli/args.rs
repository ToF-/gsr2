use crate::model::color_range::ColorRange;
use regex::Regex;
use crate::cli::command::Command;
use crate::env::configuration::Configuration;
use crate::env::default_values::{DEFAULT_HEIGHT, DEFAULT_SLIDESHOW_DELAY, DEFAULT_WIDTH};
use crate::env::dimension::{dimension, slideshow_delay};
use crate::file::paths::check_collectable;
use crate::file::paths::{check_path, check_picture_file};
use crate::model::order::Order;
use clap::Parser;
use std::io::{Error, Result};
use std::path::PathBuf;

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
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// display only pictures which are selected as cover
    #[arg(short, long)]
    pub cover: bool,

    /// display all pictures, not only cover pictures
    #[arg(short, long)]
    pub all: bool,

    /// select only picture data for pictures present on <DIRECTORY>
    #[arg(short, long, value_name = "DIRECTORY")]
    pub directory: Option<String>,

    /// select only pictures that are in the extraction list <LIST>
    #[arg(short, long, value_name = "LIST")]
    pub extraction: Option<String>,

    /// filter pictures having a percentage of pixel in the given range
    #[arg(short, long, value_name = "COLOR_RANGE")]
    pub filter: Option<String>,

    /// display N x N pictures per page (N in range [1..10[)
    #[arg(short, long, value_name = "N",value_parser(clap::value_parser!(u8).range(1..=9)))]
    pub grid: Option<u8>,

    /// window height
    #[arg(long, value_name = "N")]
    pub height: Option<i32>,

    /// start display from picture number <N>
    #[arg(short, long, value_name = "N")]
    pub index: Option<usize>,

    /// display only pictures with label = "LABEL"
    #[arg(short, long, value_name = "LABEL")]
    pub label: Option<String>,

    /// move selected picture to <DIRECTORY> on confirmation
    #[arg(long, value_name = "DIRECTORY")]
    pub r#move: Option<String>,

    /// display pictures in order
    #[arg(short, long, value_name="ORDER", ignore_case(true))]
    pub order: Option<Order>,

    /// display only pictures where file name matches PATTERN
    #[arg(short, long, value_name="PATTERN")]
    pub pattern: Option<String>,

    /// select only picture having at all their tags in TAGS (e.g "foo,bar")
    #[arg(short, long, value_name = "TAGS", conflicts_with = "select")]
    pub restrict: Option<String>,

    /// select only picture having at least one tag in TAGS (e.g "foo,bar")
    #[arg(short, long, value_name = "TAGS", conflicts_with = "restrict")]
    pub select: Option<String>,

    /// slideshow mode, displaying next picture every N seconds
    #[arg(long, value_name = "N")]
    pub slideshow: Option<i32>,

    /// display 10 x 10 thumbnail pictures per page
    #[arg(short, long, default_value_t = false, conflicts_with("grid"))]
    pub thumbnails: bool,

    /// window width
    #[arg(long, value_name = "N")]
    pub width: Option<i32>,
}

impl Args {
    pub fn parse_and_check(args_opt: Option<Vec<&str>>, config: &Configuration) -> Result<Self> {
        let mut args: Self = match args_opt {
            Some(args) => Self::parse_from(args),
            None => Self::parse(),
        };

        args.width = dimension(args.width.unwrap_or(config.width), "width", DEFAULT_WIDTH);
        args.height = dimension(
            args.height.unwrap_or(config.height),
            "height",
            DEFAULT_HEIGHT,
        );
        if let Some(pattern) = args.clone().pattern {
            match Regex::new(&pattern) {
                Ok(_) => {},
                Err(e) => return Err(Error::other(e)),
            }
        };
        args.slideshow =
            slideshow_delay(args.slideshow, "slideshow delay", DEFAULT_SLIDESHOW_DELAY);
        if let Some(Command::File { ref file_path }) = args.command {
            match check_picture_file(file_path) {
                Ok(_) => {
                    if args.grid.is_some() {
                        return Err(Error::other("option --grid not allowed for File command"));
                    } else if args.thumbnails {
                        return Err(Error::other(
                            "option --thumbnails not allowed with file command",
                        ));
                    } else {
                        if args.r#move.is_some() {
                            return Err(Error::other(
                                "option --move not allowed with file command",
                            ));
                        };
                        return Ok(args.clone());
                    }
                }

                Err(e) => return Err(e),
            }
        }
        if let Some(ref color_range_spec) = args.filter {
            match ColorRange::from_string(color_range_spec) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{} ??", color_range_spec);
                    return Err(Error::other(e))
                },
            }
        };
        if let Some(ref target_dir) = args.r#move {
            let target_path = PathBuf::from(target_dir);
            match check_collectable(&target_path) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
            if args.command.is_some() {
                return Err(Error::other("option --move not allowed with this command"));
            }
        }
        if let Some(Command::Collect { ref directory }) = args.command {
            let path = PathBuf::from(directory);
            match check_collectable(&path) {
                Ok(_) => return Ok(args.clone()),
                Err(e) => return Err(e),
            }
        }
        if let Some(Command::Directory { ref directory }) = args.command {
            match check_path(directory) {
                Ok(_) => return Ok(args.clone()),
                Err(e) => return Err(e),
            }
        }
        if let Some(Command::Move {
            ref source,
            ref target,
        }) = args.command
        {
            let target_path = PathBuf::from(target);
            let source_path = PathBuf::from(source);
            match check_collectable(&source_path) {
                Ok(_) => match check_collectable(&target_path) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            }
        }
        Ok(args.clone())
    }

    pub fn slideshow(&self) -> Option<i32> {
        self.slideshow
    }

    pub fn pictures_per_row(&self) -> i32 {
        if let Some(grid) = self.grid {
            grid.into()
        } else if self.thumbnails {
            10
        } else {
            1
        }
    }

    pub fn on_database(&self) -> bool {
        !matches!(self.command, Some(Command::File { file_path: _ }) | Some(Command::Directory { directory: _ }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Command::Directory;
    use crate::Command::File;
    use crate::cli::command::Command;
    use crate::Configuration;
    use crate::test_data::*;
    use std::io::ErrorKind;

    fn config() -> Configuration {
        Configuration::from_env().unwrap()
    }
    #[test]
    fn command_line_interface_with_command_file_with_adequate_argument() {
        let single_dot = single_dot_file_path();
        let args = vec!["gsr", "file", &single_dot];
        let args = Args::parse_and_check(Some(args), &config()).unwrap();
        if let Some(Command::File { ref file_path }) = args.command {
            assert_eq!(&single_dot_file_path(), file_path);
        } else {
            assert!(false)
        }
        assert!(!args.clone().on_database())
    }

    #[test]
    fn command_line_interface_with_command_file_with_non_existing_file() {
        let args = vec!["gsr", "file", "not_existing.png"];
        let args = Args::parse_and_check(Some(args), &config());
        assert!(args.is_err());
        let err = args.expect_err("can't extract error");
        assert_eq!(ErrorKind::NotFound, err.kind());
        assert_eq!("not found: not_existing.png", &err.to_string())
    }

    #[test]
    fn command_line_interface_with_command_file_with_non_file() {
        let args = vec!["gsr", "file", "testdata"];
        let args = Args::parse_and_check(Some(args), &config());
        assert!(args.is_err());
        let err = args.expect_err("can't extract error");
        assert_eq!(ErrorKind::Other, err.kind());
        assert_eq!("testdata is not a file", &err.to_string())
    }
    #[test]
    fn command_line_interface_with_command_file_with_non_jpg_or_png_file() {
        let args = vec!["gsr", "file", "src/file/paths.rs"];
        let args = Args::parse_and_check(Some(args), &config());
        assert!(args.is_err());
        let err = args.expect_err("can't extract error");
        assert_eq!(ErrorKind::Other, err.kind());
        assert_eq!(
            "src/file/paths.rs is not a jpg or png file",
            &err.to_string()
        );
    }

    #[test]
    fn command_line_interface_with_command_directory_and_adequate_argument() {
        let args = vec!["gsr", "dir", "."];
        let args = Args::parse_and_check(Some(args), &config()).unwrap();
        if let Some(Command::Directory { ref directory }) = args.command {
            assert_eq!(".", directory);
        } else {
            assert!(false)
        }
        assert!(!args.clone().on_database())
    }
    #[test]
    fn command_line_interface_dir_command_with_non_existing_specified_directory() {
        let args = vec!["gsr", "dir", "not_existing_dir"];
        let args = Args::parse_and_check(Some(args), &config());
        assert!(args.is_err());
        let err = args.expect_err("can't extract error");
        assert_eq!(ErrorKind::NotFound, err.kind());
        assert_eq!("not found: not_existing_dir", &err.to_string())
    }
    #[test]
    fn command_line_interface_dir_command_with_object_specified_not_directory() {
        let args = vec!["gsr", "dir", "README.md"];
        let args = Args::parse_and_check(Some(args), &config());
        assert!(args.is_err());
        let err = args.expect_err("can't extract error");
        assert_eq!(ErrorKind::NotADirectory, err.kind());
        assert_eq!("README.md is not a directory", &err.to_string())
    }
    #[test]
    fn with_no_grid_or_thumbnail_option_pictures_per_row_is_1() {
        let args = vec!["gsr", "dir", "testdata"];
        let args = Args::parse_and_check(Some(args), &config()).unwrap();
        assert_eq!(1, args.pictures_per_row())
    }
    #[test]
    fn pictures_per_row_is_determined_by_grid_option() {
        let args = vec!["gsr", "--grid", "5", "dir", "testdata"];
        let args = Args::parse_and_check(Some(args), &config()).unwrap();
        assert_eq!(5, args.pictures_per_row())
    }
    #[test]
    fn pictures_per_row_is_determined_by_thumbnails_option() {
        let args = vec!["gsr", "--thumbnails", "dir", "testdata"];
        let args = Args::parse_and_check(Some(args), &config()).unwrap();
        assert_eq!(10, args.pictures_per_row())
    }
    #[test]
    fn command_line_interface_with_no_command_dir_or_file_is_on_database() {
        let args = vec!["gsr", "--grid", "5"];
        let args = Args::parse_and_check(Some(args), &config()).unwrap();
        assert!(args.on_database())
    }
}
