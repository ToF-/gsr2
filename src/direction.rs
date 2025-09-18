#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    First,
    Last,
    Index { value: usize },
}
