use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;

#[repr(i32)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ViewOption {
    Single = 0,
    Grid2x2 = 1,
    Grid3x3 = 2,
    Grid4x4 = 3,
    Grid5x5 = 4,
    Thumbnails = 5,
    Covers = 6,
    FilePath = 7,
    FileDate = 8,
    FileSize = 9,
}

impl From<i32> for ViewOption {
    fn from(n: i32) -> Self {
        match n {
            0 => ViewOption::Single,
            1 => ViewOption::Grid2x2,
            2 => ViewOption::Grid3x3,
            3 => ViewOption::Grid4x4,
            4 => ViewOption::Grid5x5,
            5 => ViewOption::Thumbnails,
            6 => ViewOption::Covers,
            7 => ViewOption::FilePath,
            8 => ViewOption::FileDate,
            9 => ViewOption::FileSize,
            _ => todo!(),
        }
    }
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
                ViewOption::Single => "Single",
                ViewOption::Grid2x2 => "Grid2x2",
                ViewOption::Grid3x3 => "Grid3x3",
                ViewOption::Grid4x4 => "Grid4x4",
                ViewOption::Grid5x5 => "Grid5x5",
                ViewOption::Thumbnails => "Thumbnails",
                ViewOption::Covers => "Covers",
                ViewOption::FilePath => "FilePath",
                ViewOption::FileDate => "FileDate",
                ViewOption::FileSize => "FileSize",
            }
        )
    }
}
