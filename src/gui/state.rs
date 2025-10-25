use crate::env::default_values::{FOCUS_SYMBOL_1, FOCUS_SYMBOL_2};
use crate::gui::mode::Mode;

#[derive(Clone, Debug)]
pub struct State {
    pictures_per_row: usize,
    old_pictures_per_row: usize,
    single_view: bool,
    expand_on: bool,
    full_size_on: bool,
    slideshow_on: bool,
    display_date_on: bool,
    display_size_on: bool,
    palette_on: bool,
    mode: Mode,
    focus_symbol: char,
    change_focus_symbol_on: bool,
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
            display_date_on: false,
            display_size_on: false,
            palette_on: false,
            mode: Mode::View,
            focus_symbol: FOCUS_SYMBOL_1,
            change_focus_symbol_on: true,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }

    pub fn focus_symbol(&self) -> char {
        self.focus_symbol
    }

    pub fn change_focus_symbol_on(&self) -> bool {
        self.change_focus_symbol_on
    }

    pub fn toggle_change_focus_symbol(&mut self) {
        self.change_focus_symbol_on = !self.change_focus_symbol_on
    }

    pub fn toggle_focus_symbol(&mut self) {
        if self.focus_symbol == FOCUS_SYMBOL_1 {
            self.focus_symbol = FOCUS_SYMBOL_2
        } else {
            self.focus_symbol = FOCUS_SYMBOL_1
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }
    pub fn slideshow_on(&self) -> bool {
        self.slideshow_on
    }

    pub fn set_slideshow_off(&mut self) {
        self.slideshow_on = false;
    }

    pub fn palette_on(&self) -> bool {
        self.palette_on
    }

    pub fn toggle_palette(&mut self) {
        self.palette_on = !self.palette_on
    }

    pub fn pictures_per_row(&self) -> usize {
        self.pictures_per_row
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

    pub fn acknowledge_grid_size_change(&mut self) {
        self.old_pictures_per_row = self.pictures_per_row
    }

    pub fn change_grid_size(&mut self, pictures_per_row: usize) {
        if pictures_per_row != self.pictures_per_row {
            self.old_pictures_per_row = self.pictures_per_row;
            self.pictures_per_row = pictures_per_row
        };
        self.single_view = false
    }

    pub fn display_date_on(&self) -> bool {
        self.display_date_on
    }

    pub fn toggle_display_date(&mut self) {
        self.display_date_on = !self.display_date_on
    }

    pub fn display_size_on(&self) -> bool {
        self.display_size_on
    }

    pub fn toggle_display_size(&mut self) {
        self.display_size_on = !self.display_size_on
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
