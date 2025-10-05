#[derive(Clone, Debug)]
pub struct State {
    pub pictures_per_row: usize,
    old_pictures_per_row: usize,
    pub single_view: bool,
    pub expand_on: bool,
    pub full_size_on: bool,
    slideshow_on: bool,
}

impl State {
    pub fn new(pictures_per_row: usize, slideshow_on: bool) -> Self {
        State {
            pictures_per_row,
            old_pictures_per_row: 1,
            single_view: pictures_per_row == 1,
            expand_on: false,
            full_size_on: false,
            slideshow_on,
        }
    }

    pub fn slideshow_on(&self) -> bool {
        self.slideshow_on
    }

    pub fn set_slideshow_off(&mut self) {
        self.slideshow_on = false;
    }

    pub fn pictures_per_row(&self) -> usize {
        self.pictures_per_row
    }

    pub fn old_pictures_per_row(&self) -> usize {
        self.old_pictures_per_row
    }

    pub fn single_view(&self) -> bool {
        self.single_view
    }

    pub fn expand_on(&self) -> bool {
        self.expand_on
    }

    pub fn full_size_on(&self) -> bool {
        self.full_size_on
    }

    pub fn toggle_single_view(&mut self) {
        self.single_view = !self.single_view
    }

    pub fn toggle_slideshow(&mut self) {
        self.slideshow_on = !self.slideshow_on
    }
    pub fn toggle_expand(&mut self) {
        self.expand_on = !self.expand_on
    }

    pub fn toggle_full_size(&mut self) {
        self.full_size_on = !self.full_size_on
    }

    pub fn dimension_changed(&self) -> bool {
        self.pictures_per_row != self.old_pictures_per_row
    }

    pub fn acknowledge_dimension(&mut self) {
        self.old_pictures_per_row = self.pictures_per_row
    }

    pub fn change_grid_size(&mut self, pictures_per_row: usize) {
        if pictures_per_row != self.pictures_per_row {
            self.old_pictures_per_row = self.pictures_per_row;
            self.pictures_per_row = pictures_per_row
        };
        self.single_view = false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_toggle_single_view_cells_per_row_change() {
        let mut my_state = State::new(5, false);
        assert!(!my_state.single_view());
        my_state.toggle_single_view();
        assert!(my_state.single_view());
        my_state.toggle_single_view();
        assert!(!my_state.single_view());
    }
}
