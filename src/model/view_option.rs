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
    FullSize = 10,
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
            10 => ViewOption::FullSize,
            _ => todo!(),
        }
    }
}
