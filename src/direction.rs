#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    PageStart,
    PageEnd,
    First,
    Last,
    Index { value: usize },
}
