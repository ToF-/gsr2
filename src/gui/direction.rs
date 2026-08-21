#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Down,
    First,
    Index { value: usize },
    Last,
    Left,
    NextPage,
    PageEnd,
    PageStart,
    PrevPage,
    Right,
    Up,
}

impl From<Direction> for i32 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Down => -1,
            Direction::First => -2,
            Direction::Index { value } => value as i32,
            Direction::Last => -3,
            Direction::Left => -4,
            Direction::NextPage => -5,
            Direction::PageEnd => -6,
            Direction::PageStart => -7,
            Direction::PrevPage => -8,
            Direction::Right => -9,
            Direction::Up => -10,
        }
    }
}

impl From<i32> for Direction {
    fn from(n: i32) -> Self {
        match n {
            -1 => Direction::Down,
            -2 => Direction::First,
            -3 => Direction::Last,
            -4 => Direction::Left,
            -5 => Direction::NextPage,
            -6 => Direction::PageEnd,
            -7 => Direction::PageStart,
            -8 => Direction::PrevPage,
            -9 => Direction::Right,
            -10 => Direction::Up,
            value if value >= 0 => Direction::Index {
                value: value as usize,
            },
            _ => todo!(),
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "Down" => Direction::Down,
            "First" => Direction::First,
            "Last" => Direction::Last,
            "Left" => Direction::Left,
            "NextPage" => Direction::NextPage,
            "PageEnd" => Direction::PageEnd,
            "PageStart" => Direction::PageStart,
            "PrevPage" => Direction::PrevPage,
            "Right" => Direction::Right,
            "Up" => Direction::Up,
            _ => todo!(),
        }
    }
}
