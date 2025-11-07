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
