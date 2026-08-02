use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ViewOption {
    Single,
    Grid2x2,
    Grid3x3,
    Grid4x4,
    Grid5x5,
    Thumbnails,
    Covers,
    FilePath,
    FileDate,
    FileSize,
}

impl FromStr for ViewOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Single" => Ok(ViewOption::Single),
            "Grid2x2" => Ok(ViewOption::Grid2x2),
            "Grid3x3" => Ok(ViewOption::Grid3x3),
            "Grid4x4" => Ok(ViewOption::Grid4x4),
            "Grid5x5" => Ok(ViewOption::Grid5x5),
            "Thumbnails" => Ok(ViewOption::Thumbnails),
            "Covers" => Ok(ViewOption::Covers),
            "FilePath" => Ok(ViewOption::FilePath),
            "FileDate" => Ok(ViewOption::FileDate),
            "FileSize" => Ok(ViewOption::FileSize),
            _ => Err(format!("unknown view option: {s}")),
        }
    }
}

impl Display for ViewOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                ViewOption::Single => "single",
                ViewOption::Grid2x2 => "grid 2x2",
                ViewOption::Grid3x3 => "grid 3x3",
                ViewOption::Grid4x4 => "grid 4x4",
                ViewOption::Grid5x5 => "grid 5x5",
                ViewOption::Thumbnails => "thumbnails",
                ViewOption::Covers => "covers",
                ViewOption::FilePath => "file path",
                ViewOption::FileDate => "file date",
                ViewOption::FileSize => "file size",
            }
        )
    }
}
