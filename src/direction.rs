#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    First,
    Last,
    Index { value: usize },
}
