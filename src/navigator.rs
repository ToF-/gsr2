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

    pub fn move_next(&mut self) {
        self.position += 1
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
}
