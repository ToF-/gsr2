use crate::gui::direction::Direction;

#[derive(Debug, Clone)]
pub struct Navigator {
    limit: usize,
    pictures_per_row: usize,
    position: usize,
    old_position: usize,
    page_start: usize,
    page_end: usize,
    page_changed: bool,
}

impl Navigator {
    pub fn new(limit: usize, pictures_per_row: usize) -> Self {
        let mut result = Navigator {
            limit,
            pictures_per_row,
            position: 0,
            old_position: 0,
            page_start: 0,
            page_end: 0,
            page_changed: false,
        };
        result.update_page_limits();
        result
    }

    pub fn position(&self) -> usize {
        self.position
    }

    #[allow(dead_code)]
    pub fn old_position(&self) -> usize {
        self.old_position
    }

    pub fn page_start(&self) -> usize {
        self.page_start
    }

    pub fn page_end(&self) -> usize {
        self.page_end
    }

    pub fn page_size(&self) -> usize {
        self.pictures_per_row * self.pictures_per_row
    }

    pub fn current_page(&self) -> usize {
        self.position() / self.page_size() + 1
    }

    pub fn total_pages(&self) -> usize {
        self.limit / self.page_size() + 1
    }

    pub fn next_page_start(&self) -> usize {
        self.page_start + self.page_size()
    }

    pub fn prev_page_start(&self) -> usize {
        if self.page_start >= self.page_size() {
            self.page_start - self.page_size()
        } else {
            0
        }
    }

    pub fn has_moved(&self) -> bool {
        self.page_changed || (self.old_position != self.position)
    }

    pub fn page_changed(&self) -> bool {
        self.page_changed
    }

    pub fn set_page_changed(&mut self) {
        self.page_changed = true
    }

    pub fn set_pictures_per_row(&mut self, pictures_per_row: usize) {
        self.pictures_per_row = pictures_per_row;
        self.update_page_limits();
    }

    pub fn position_from_coords(&self, row: usize, col: usize) -> Option<usize> {
        let position_from_coords = self.page_start + row * self.pictures_per_row + col;
        if position_from_coords < self.limit {
            Some(position_from_coords)
        } else {
            None
        }
    }

    pub fn coords_from_position(&self, position: usize) -> Option<(usize, usize)> {
        if (self.page_start()..=self.page_end()).contains(&position) {
            let row = (position - self.page_start()) / self.pictures_per_row;
            let col = (position - self.page_start()) % self.pictures_per_row;
            Some((row, col))
        } else {
            None
        }
    }

    pub fn can_move(&mut self, direction: Direction) -> bool {
        let can_move = match direction {
            Direction::First => true,
            Direction::Last => true,
            Direction::Left => self.position > 0,
            Direction::Right => self.position < self.limit - 1,
            Direction::Index { value } => value < self.limit,
            Direction::Down => self.position + self.pictures_per_row < self.limit,
            Direction::Up => self.position >= self.pictures_per_row,
            Direction::PageStart => true,
            Direction::PageEnd => true,
        };
        if !can_move {
            self.old_position = self.position;
            self.page_changed = false;
        }
        can_move
    }

    pub fn can_move_next_page(&mut self) -> bool {
        self.can_move(Direction::Index {
            value: self.next_page_start(),
        })
    }

    pub fn can_move_prev_page(&mut self) -> bool {
        self.can_move(Direction::Index {
            value: self.prev_page_start(),
        })
    }

    pub fn move_towards(&mut self, direction: Direction) {
        self.old_position = self.position;
        match direction {
            Direction::Right => self.position += 1,
            Direction::Left => self.position -= 1,
            Direction::Last => self.position = self.limit - 1,
            Direction::First => self.position = 0,
            Direction::Index { value } => self.position = value,
            Direction::Down => self.position += self.pictures_per_row,
            Direction::Up => self.position = self.position.saturating_sub(self.pictures_per_row),
            Direction::PageStart => self.position = self.page_start,
            Direction::PageEnd => self.position = self.page_end,
        };
        self.update_page_limits();
    }

    pub fn move_next_page(&mut self) {
        self.move_towards(Direction::Index {
            value: self.next_page_start(),
        })
    }

    pub fn move_prev_page(&mut self) {
        self.move_towards(Direction::Index {
            value: self.prev_page_start(),
        })
    }

    pub fn update_page_limits(&mut self) {
        if self.limit > 0 {
            let old_page_start: usize = self.page_start;
            self.page_start = (self.position / self.page_size()) * self.page_size();
            self.page_end = (self.page_start + self.page_size() - 1).min(self.limit - 1);
            self.page_changed = old_page_start != self.page_start;
        }
    }
}
#[cfg(test)]

mod tests {
    use super::*;
    use crate::env::default_values::ONE_PICTURE_PER_ROW;

    #[test]
    fn navigator_cannot_move_past_gallery_limit() {
        let mut navigator = Navigator::new(3, ONE_PICTURE_PER_ROW);
        assert_eq!(0, navigator.position());
        assert!(navigator.can_move(Direction::Right));
        navigator.move_towards(Direction::Right);
        assert_eq!(1, navigator.position());
        navigator.move_towards(Direction::Right);
        assert!(!navigator.can_move(Direction::Right));
    }

    #[test]
    fn navigator_cannot_move_before_first_position() {
        let mut navigator = Navigator::new(3, ONE_PICTURE_PER_ROW);
        assert!(!navigator.can_move(Direction::Left));
        navigator.move_towards(Direction::Right);
        assert!(navigator.can_move(Direction::Left));
        navigator.move_towards(Direction::Left);
        assert_eq!(0, navigator.position());
    }

    #[test]
    fn navigator_can_move_to_first_and_last_position() {
        let mut navigator = Navigator::new(3, ONE_PICTURE_PER_ROW);
        navigator.move_towards(Direction::Last);
        assert_eq!(2, navigator.position());
        navigator.move_towards(Direction::First);
        assert_eq!(0, navigator.position());
    }

    #[test]
    fn given_coords_can_tell_position_trivial_case() {
        let navigator = Navigator::new(3, ONE_PICTURE_PER_ROW);
        assert_eq!(Some(0), navigator.position_from_coords(0, 0));
    }

    #[test]
    fn given_coords_can_tell_position_with_several_pictures_per_row_on_first_page() {
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
    #[test]
    fn given_a_destination_index_can_move_if_within_limit() {
        let mut navigator = Navigator::new(10, 2);
        assert!(!navigator.can_move(Direction::Index { value: 10 }));
        assert!(navigator.can_move(Direction::Index { value: 7 }));
    }
    #[test]
    fn moving_to_a_specific_index() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Index { value: 7 });
        assert_eq!(7, navigator.position());
    }
    #[test]
    fn next_page_start_is_page_start_plus_page_size_modulo_limit() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Right);
        assert_eq!(1, navigator.position());
        assert_eq!(4, navigator.next_page_start());
        assert!(navigator.can_move_next_page());
        navigator.move_next_page();
        assert!(navigator.can_move_next_page());
        navigator.move_next_page();
        assert_eq!(8, navigator.page_start());
        assert_eq!(12, navigator.next_page_start());
    }
    #[test]
    fn prev_page_start_is_page_start_minus_page_size() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Right);
        assert_eq!(1, navigator.position());
        assert_eq!(0, navigator.prev_page_start());
        navigator.move_next_page();
        navigator.move_next_page();
        assert_eq!(8, navigator.page_start());
        assert_eq!(4, navigator.prev_page_start());
        assert!(navigator.can_move_prev_page());
        navigator.move_prev_page();
        assert_eq!(4, navigator.position());
    }
    #[test]
    fn moving_down_moves_to_entry_one_row_further() {
        let mut navigator = Navigator::new(10, 2);
        assert_eq!(0, navigator.position());
        navigator.move_towards(Direction::Down);
        assert_eq!(2, navigator.position());
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::Down);
        assert_eq!(5, navigator.position());
    }
    #[test]
    fn cannot_move_down_if_beyond_limit() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::Down);
        assert!(navigator.can_move(Direction::Down));
        navigator.move_towards(Direction::Down);
        assert!(!navigator.can_move(Direction::Down));
    }
    #[test]
    fn moving_up_moves_to_entry_one_row_above() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::Up);
        assert_eq!(3, navigator.position());
        navigator.move_towards(Direction::Up);
        assert_eq!(1, navigator.position());
    }
    #[test]
    fn cannot_move_up_if_before_limit() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Down);
        assert!(navigator.can_move(Direction::Up));
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::Up);
        assert!(!navigator.can_move(Direction::Up));
    }
    #[test]
    fn moving_to_beginning_of_page() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::PageStart);
        assert_eq!(4, navigator.position());
    }
    #[test]
    fn moving_to_end_of_page() {
        let mut navigator = Navigator::new(10, 2);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::Down);
        navigator.move_towards(Direction::PageEnd);
        assert_eq!(7, navigator.position());
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::PageEnd);
        assert_eq!(9, navigator.position());
    }
    #[test]
    fn position_from_coords_depnds_on_page_start() {
        let mut navigator = Navigator::new(10, 2);
        assert_eq!(Some(0), navigator.position_from_coords(0, 0));
        assert_eq!(Some(1), navigator.position_from_coords(0, 1));
        assert_eq!(Some(3), navigator.position_from_coords(1, 1));
        navigator.move_towards(Direction::Index {
            value: navigator.next_page_start(),
        });
        assert_eq!(Some(7), navigator.position_from_coords(1, 1));
    }
    #[test]
    fn coords_from_position_depends_on_given_position() {
        let mut navigator = Navigator::new(10, 2);
        assert_eq!(None, navigator.coords_from_position(27));
        assert_eq!(Some((0, 0)), navigator.coords_from_position(0));
        assert_eq!(Some((0, 1)), navigator.coords_from_position(1));
        assert_eq!(Some((1, 0)), navigator.coords_from_position(2));
        navigator.move_towards(Direction::Last);
        assert_eq!(9, navigator.position());
        assert_eq!(Some((0, 1)), navigator.coords_from_position(9));
    }
    #[test]
    fn after_moving_old_position_and_new_position_can_differ() {
        let mut navigator = Navigator::new(10, 2);
        assert!(navigator.position() == navigator.old_position());
        assert!(!navigator.has_moved());
        navigator.move_towards(Direction::Right);
        assert!(navigator.has_moved());
        navigator.move_towards(Direction::Last);
        assert!(navigator.has_moved());
        navigator.move_towards(Direction::Last);
        assert!(!navigator.has_moved());
    }
    #[test]
    fn current_page_according_to_position_pictures_per_row() {
        let mut navigator = Navigator::new(10, 2);
        assert_eq!(navigator.current_page(), 1);
        navigator.move_towards(Direction::Index {
            value: navigator.next_page_start(),
        });
        assert_eq!(navigator.current_page(), 2);
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::Right);
        navigator.move_towards(Direction::Right);
        assert_eq!(navigator.current_page(), 3);
    }
    #[test]
    fn total_pages_according_to_len_and_page_size() {
        let mut navigator = Navigator::new(10, 2);
        assert_eq!(navigator.total_pages(), 3);
    }
}
