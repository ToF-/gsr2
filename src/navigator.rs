use crate::direction::Direction;

#[derive(Debug, Clone)]
pub struct Navigator {
    limit: usize,
    cells_per_row: usize,
    position: usize,
    page_start: usize,
    page_changed: bool,
}

impl Navigator {
    pub fn new(limit: usize, cells_per_row: usize) -> Self {
        Navigator {
            limit,
            cells_per_row,
            position: 0,
            page_start: 0,
            page_changed: false,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn page_start(&self) -> usize {
        self.page_start
    }

    pub fn page_changed(&self) -> bool {
        self.page_changed
    }

    pub fn position_from_coords(&self, row: usize, col: usize) -> Option<usize> {
        let position_from_coords = self.page_start + row * self.cells_per_row + col;
        if position_from_coords < self.limit {
            Some(position_from_coords)
        } else {
            None
        }
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
        };
        self.update_page_start();
    }

    fn page_size(&self) -> usize {
        self.cells_per_row * self.cells_per_row
    }

    fn update_page_start(&mut self) {
        let old_page_start: usize = self.page_start;
        self.page_start = (self.position / self.page_size()) * self.page_size();
        self.page_changed = !(old_page_start == self.page_start)
    }
}
#[cfg(test)]

mod tests {
    use super::*;
    use crate::default_values::ONE_CELL_PER_ROW;

    #[test]
    fn navigator_cannot_move_past_gallery_limit() {
        let mut navigator = Navigator::new(3, ONE_CELL_PER_ROW);
        assert_eq!(0, navigator.position());
        assert!(navigator.can_move(Direction::Right));
        navigator.move_towards(Direction::Right);
        assert_eq!(1, navigator.position());
        navigator.move_towards(Direction::Right);
        assert!(!navigator.can_move(Direction::Right));
    }

    #[test]
    fn navigator_cannot_move_before_first_position() {
        let mut navigator = Navigator::new(3, ONE_CELL_PER_ROW);
        assert!(!navigator.can_move(Direction::Left));
        navigator.move_towards(Direction::Right);
        assert!(navigator.can_move(Direction::Left));
        navigator.move_towards(Direction::Left);
        assert_eq!(0, navigator.position());
    }

    #[test]
    fn navigator_can_move_to_first_and_last_position() {
        let mut navigator = Navigator::new(3, ONE_CELL_PER_ROW);
        navigator.move_towards(Direction::Last);
        assert_eq!(2, navigator.position());
        navigator.move_towards(Direction::First);
        assert_eq!(0, navigator.position());
    }

    #[test]
    fn given_coords_can_tell_position_trivial_case() {
        let navigator = Navigator::new(3, ONE_CELL_PER_ROW);
        assert_eq!(Some(0), navigator.position_from_coords(0, 0));
    }

    #[test]
    fn given_coords_can_tell_position_with_several_cells_per_row_on_first_page() {
        assert_eq!(Some(0), Navigator::new(10, 2).position_from_coords(0, 0));
        assert_eq!(Some(1), Navigator::new(10, 2).position_from_coords(0, 1));
        assert_eq!(Some(2), Navigator::new(10, 2).position_from_coords(1, 0));
        assert_eq!(Some(3), Navigator::new(10, 2).position_from_coords(1, 1));
    }
    #[test]
    fn given_illegal_coors_position_is_none() {
        assert_eq!(None, Navigator::new(1, 1).position_from_coords(0, 1));
        assert_eq!(None, Navigator::new(10, 4).position_from_coords(3, 3));
    }
    #[test]
    fn after_page_change_position_from_coords_changes() {
        let mut navigator = Navigator::new(10, 2);
        assert_eq!(Some(3), navigator.position_from_coords(1, 1));
        for _ in 1..=4 {
            navigator.move_towards(Direction::Right)
        }
        assert_eq!(4, navigator.position());
        assert_eq!(Some(4), navigator.position_from_coords(0, 0));
        assert_eq!(Some(7), navigator.position_from_coords(1, 1));
    }
    #[test]
    fn after_page_change_page_change_is_detected() {
        let mut navigator = Navigator::new(10, 2);
        assert!(!navigator.page_changed());
        assert_eq!(Some(3), navigator.position_from_coords(1, 1));
        for _ in 1..=4 {
            navigator.move_towards(Direction::Right);
        }
        assert!(navigator.page_changed());
    }
}
