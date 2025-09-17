use crate::control::{Control, Controls, default_controls};
use crate::direction::Direction;
use crate::gallery::Gallery;
use crate::navigator::Navigator;
use crate::picture::Picture;

#[derive(Debug)]
pub struct ApplicationState {
    gallery: Gallery,
    navigator: Navigator,
    controls: Controls,
    pictures_per_row: usize,
    expand_on: bool,
    full_size_on: bool,
    palette_on: bool,
}

impl ApplicationState {
    pub fn new(palette_on: bool) -> Self {
        ApplicationState {
            gallery: Gallery::new(),
            navigator: Navigator::new(0),
            controls: default_controls(),
            pictures_per_row: 1,
            expand_on: false,
            full_size_on: false,
            palette_on,
        }
    }

    pub fn gallery(&self) -> &Gallery {
        &self.gallery
    }

    pub fn current_picture(&self) -> Picture {
        self.gallery.picture(self.navigator.position())
    }

    pub fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    pub fn move_towards(&mut self, direction: Direction) {
        self.navigator.move_towards(direction)
    }

    pub fn can_move(&mut self, direction: Direction) -> bool {
        self.navigator.can_move(direction)
    }

    pub fn expand_on(&self) -> bool {
        self.expand_on
    }

    pub fn full_size_on(&self) -> bool {
        self.full_size_on
    }

    pub fn palette_on(&self) -> bool {
        self.palette_on
    }

    pub fn set_gallery(&mut self, gallery: Gallery) {
        self.gallery = gallery;
        self.navigator = Navigator::new(self.gallery.len())
    }

    pub fn toggle_expand(&mut self) {
        self.expand_on = !self.expand_on
    }

    pub fn toggle_full_size(&mut self) {
        self.full_size_on = !self.full_size_on
    }

    pub fn toggle_palette(&mut self) {
        self.palette_on = !self.palette_on
    }

    pub fn get_control(&self, key_name: &str) -> Option<Control> {
        self.controls.get(key_name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn after_palette_toggle_palette_on_is_inverted() {
        let mut state = ApplicationState::new(false);
        state.toggle_palette();
        assert_eq!(true, state.palette_on());
    }

    #[test]
    fn after_expand_toggle_expand_on_is_inverted() {
        let mut state = ApplicationState::new(false);
        state.toggle_expand();
        assert_eq!(true, state.expand_on());
        state.toggle_expand();
        assert_eq!(false, state.expand_on());
    }
    #[test]
    fn after_full_size_toggle_full_size_on_is_inverted() {
        let mut state = ApplicationState::new(false);
        state.toggle_full_size();
        assert_eq!(true, state.full_size_on());
        state.toggle_full_size();
        assert_eq!(false, state.full_size_on());
    }

    #[test]
    fn get_the_control_matching_a_keyname() {
        let state = ApplicationState::new(false);
        assert_eq!(Some(Control::ToggleFullSize), state.get_control("f"));
    }
}
