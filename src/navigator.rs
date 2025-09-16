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

    pub fn can_move_next(&self) -> bool {
        self.position < self.limit - 1
    }

    pub fn can_move_prev(&self) -> bool {
        self.position > 0
    }

    pub fn move_next(&mut self) {
        self.position += 1
    }

    pub fn move_prev(&mut self) {
        self.position -= 1
    }

    pub fn move_first(&mut self) {
        self.position = 0
    }

    pub fn move_last(&mut self) {
        self.position = self.limit - 1
    }
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn navigator_cannot_move_past_gallery_limit() {
        let mut navigator = Navigator::new(3);
        assert_eq!(0, navigator.position());
        assert!(navigator.can_move_next());
        navigator.move_next();
        assert_eq!(1, navigator.position());
        navigator.move_next();
        assert!(!navigator.can_move_next());
    }

    #[test]
    fn navigator_cannot_move_before_first_position() {
        let mut navigator = Navigator::new(3);
        assert!(!navigator.can_move_prev());
        navigator.move_next();
        assert!(navigator.can_move_prev());
        navigator.move_prev();
        assert_eq!(0, navigator.position());
    }

    #[test]
    fn navigator_can_move_to_first_and_last_position() {
        let mut navigator = Navigator::new(3);
        navigator.move_last();
        assert_eq!(2, navigator.position());
        navigator.move_first();
        assert_eq!(0, navigator.position());
    }
}
