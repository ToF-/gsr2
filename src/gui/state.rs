#[derive(Clone, Debug)]
pub struct State {
    pub pictures_per_row: usize,
    pub old_pictures_per_row: usize,
    pub expand_on: bool,
    pub full_size_on: bool,
    pub palette_on: bool,
}

impl State {
    pub fn new(pictures_per_row: usize) -> Self {
        State {
            pictures_per_row,
            old_pictures_per_row: 1,
            expand_on: false,
            full_size_on: false,
            palette_on: false,
        }
    }

    pub fn pictures_per_row(&self) -> usize {
        self.pictures_per_row
    }

    pub fn single_view(&self) -> bool {
        self.pictures_per_row == 1
    }

    pub fn full_size_on(&self) -> bool {
        self.full_size_on
    }

    pub fn toggle_single_view(&mut self) {
        if self.old_pictures_per_row != self.pictures_per_row {
            std::mem::swap(&mut self.pictures_per_row, &mut self.old_pictures_per_row);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_toggle_single_view_cells_per_row_change() {
        let mut my_state = State::new(5);
        assert!(!my_state.single_view());
        my_state.toggle_single_view();
        assert!(my_state.single_view());
        my_state.toggle_single_view();
        assert!(!my_state.single_view());
    }
    #[test]
    fn state_with_one_picture_per_row_cannot_toggle_single_view() {
        let mut my_state = State::new(1);
        assert!(my_state.single_view());
        my_state.toggle_single_view();
        assert!(my_state.single_view());
    }
}
