pub struct Settings {
    pictures_per_row: i32,
    palette_on: bool,
    expand_on: bool,
    full_size_on: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            pictures_per_row: 10,
            palette_on: false,
            expand_on: false,
    full_size_on: false,
        }
    }
}

impl Settings {
    pub fn pictures_per_row(&self) -> i32 {
        self.pictures_per_row
    }

    pub fn single_view(&self) -> bool {
        self.pictures_per_row == 1
    }

    pub fn thumbnail_view(&self) -> bool {
        self.pictures_per_row == 10
    }

    pub fn palette_on(&self) -> bool {
        self.palette_on
    }

    pub fn expand_on(&self) -> bool {
        self.expand_on
    }

    pub fn full_size_on(&self) -> bool {
        self.full_size_on
    }

    pub fn toggle_pictures_per_row(&mut self, new: i32) -> i32 {
        1
    }

    pub fn toggle_palette(&mut self) {
    }

    pub fn toggle_expand(&mut self) {
    }

    pub fn toggle_full_size(&mut self) -> bool {
        false
    }
}
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::default();
        assert_eq!(10, settings.pictures_per_row());
        assert_eq!(false, settings.palette_on());
        assert_eq!(false, settings.expand_on());
        assert_eq!(false, settings.full_size_on());
    }

    #[test]
    fn switching_palette_on() {
        let mut settings = Settings::default();
        settings.toggle_palette();
        assert!(settings.palette_on());
        settings.toggle_palette();
        assert!(!settings.palette_on());
    }

    #[test]
    fn switching_full_size_only_when_single_view() {
        let mut settings = Settings::default();
        assert!(!settings.single_view());
        assert!(settings.thumbnail_view());
        assert!(!settings.toggle_full_size());
        assert!(!settings.full_size_on());
        settings.toggle_pictures_per_row(1);
        assert!(settings.single_view());
        assert!(settings.toggle_full_size());
        assert!(settings.full_size_on());
        settings.toggle_pictures_per_row(1);
        assert!(!settings.full_size_on());
    }

    #[test]
    fn setting_and_toggling_pictures_per_row() {
        let mut settings = Settings::default();
        assert_eq!(10, settings.pictures_per_row());
        settings.toggle_pictures_per_row(2);
        assert_eq!(2, settings.pictures_per_row());
        settings.toggle_pictures_per_row(2);
        assert_eq!(10, settings.pictures_per_row());
        settings.toggle_pictures_per_row(1);
        assert_eq!(1, settings.pictures_per_row());
        settings.toggle_pictures_per_row(1);
        assert_eq!(10, settings.pictures_per_row());
        settings.toggle_pictures_per_row(2);
        assert_eq!(2, settings.pictures_per_row());
        settings.toggle_pictures_per_row(1);
        assert_eq!(1, settings.pictures_per_row());
        settings.toggle_pictures_per_row(1);
        assert_eq!(2, settings.pictures_per_row());
    }
}
