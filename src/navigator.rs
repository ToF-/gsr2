use crate::direction::Direction;

#[derive(Debug, Clone)]
pub struct Navigator {
    limit: usize,
    position: usize,
}

impl Navigator {
    pub fn new(limit: usize) -> Self {
        Navigator { limit, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        match direction {
            Direction::First => true,
            Direction::Last => true,
            Direction::Left => self.position > 0,
            Direction::Right => self.position < self.limit - 1,
        }
    }

    pub fn move_towards(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.position += 1,
            Direction::Left => self.position -= 1,
            Direction::Last => self.position = self.limit - 1,
            Direction::First => self.position = 0,
            _ => {}
        }
    }
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn navigator_cannot_move_past_gallery_limit() {
        let mut navigator = Navigator::new(3);
        assert_eq!(0, navigator.position());
        assert!(navigator.can_move(Direction::Right));
        navigator.move_towards(Direction::Right);
        assert_eq!(1, navigator.position());
        navigator.move_towards(Direction::Right);
        assert!(!navigator.can_move(Direction::Right));
    }

    #[test]
    fn navigator_cannot_move_before_first_position() {
        let mut navigator = Navigator::new(3);
        assert!(!navigator.can_move(Direction::Left));
        navigator.move_towards(Direction::Right);
        assert!(navigator.can_move(Direction::Left));
        navigator.move_towards(Direction::Left);
        assert_eq!(0, navigator.position());
    }

    #[test]
    fn navigator_can_move_to_first_and_last_position() {
        let mut navigator = Navigator::new(3);
        navigator.move_towards(Direction::Last);
        assert_eq!(2, navigator.position());
        navigator.move_towards(Direction::First);
        assert_eq!(0, navigator.position());
    }
}
