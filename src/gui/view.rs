pub mod legacy_main_view;
pub mod palette_area;
pub mod picture_frame;
pub mod treelist_view;

#[derive(Clone, Debug)]
pub struct View {
    pictures_per_row: i32,
    last_pictures_per_row: i32,
    palette_on: bool,
    full_size: bool,
    focus_at_coords: (i32, i32),
}

impl Default for View {
    fn default() -> Self {
        Self {
            pictures_per_row: 10,
            last_pictures_per_row: 1,
            palette_on: false,
            full_size: false,
            focus_at_coords: (0, 0),
        }
    }
}

impl View {
    pub fn pictures_per_row(&self) -> i32 {
        self.pictures_per_row
    }

    pub fn set_pictures_per_row(&mut self, n: i32) {
        self.last_pictures_per_row = n;
        self.toggle_pictures_per_row();
    }

    pub fn toggle_pictures_per_row(&mut self) {
        std::mem::swap(&mut self.pictures_per_row, &mut self.last_pictures_per_row);
        self.full_size = false;
    }

    pub fn palette_on(&self) -> bool {
        self.palette_on
    }

    pub fn set_palette_on(&mut self, on: bool) {
        self.palette_on = on
    }

    pub fn toggle_palette_on(&mut self) {
        self.palette_on = !self.palette_on
    }

    pub fn full_size(&self) -> bool {
        self.full_size
    }

    pub fn toggle_full_size(&mut self) -> bool {
        if self.single_view() {
            self.full_size = !self.full_size;
            true
        } else {
            false
        }
    }

    pub fn single_view(&self) -> bool {
        self.pictures_per_row == 1
    }

    pub fn thumbnail_view(&self) -> bool {
        self.pictures_per_row == 10
    }

    pub fn focus_at_coords(&self) -> (i32, i32) {
        self.focus_at_coords
    }

    pub fn set_focus_at_coords(&mut self, coords: (i32, i32)) {
        self.focus_at_coords = coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_view_state() {
        let view = View::default();
        assert_eq!(10, view.pictures_per_row());
        assert_eq!(false, view.palette_on());
        assert_eq!(false, view.full_size());
    }

    #[test]
    fn switching_pictures_per_row() {
        let mut view = View::default();
        view.set_pictures_per_row(2);
        assert_eq!(2, view.pictures_per_row());
        view.toggle_pictures_per_row();
        assert_eq!(10, view.pictures_per_row());
        view.toggle_pictures_per_row();
        assert_eq!(2, view.pictures_per_row());
        view.set_pictures_per_row(5);
        assert_eq!(5, view.pictures_per_row());
        view.toggle_pictures_per_row();
        assert_eq!(2, view.pictures_per_row());
    }

    #[test]
    fn switching_palette_on() {
        let mut view = View::default();
        view.toggle_palette_on();
        assert!(view.palette_on());
        view.toggle_palette_on();
        assert!(!view.palette_on());
    }

    #[test]
    fn switching_full_size_only_when_single_view() {
        let mut view = View::default();
        assert!(!view.single_view());
        assert!(view.thumbnail_view());
        assert!(!view.toggle_full_size());
        assert!(!view.full_size());
        view.set_pictures_per_row(1);
        assert!(view.single_view());
        assert!(view.toggle_full_size());
        assert!(view.full_size());
        view.toggle_pictures_per_row();
        assert!(!view.full_size());
    }
}
