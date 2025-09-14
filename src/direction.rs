#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn from_key_name(key_name: &str) -> Option<Direction> {
    match key_name {
        "Left" => Some(Direction::Left),
        "Right" => Some(Direction::Right),
        "Up" => Some(Direction::Up),
        "Down" => Some(Direction::Down),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_key_name_to_a_direction() {
        assert_eq!(Some(Direction::Left), from_key_name("Left"))
    }
}
