pub struct Settings {
    current_pictures_per_row: i32,
    last_pictures_per_row: i32,
    palette_on: bool,
    expand_on: bool,
    full_size_on: bool,
    blinking_on: bool,
    covers_only: bool,
    file_date_on: bool,
    file_path_on: bool,
    file_size_on: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            current_pictures_per_row: 10,
            last_pictures_per_row: 1,
            palette_on: false,
            expand_on: false,
            blinking_on: true,
            full_size_on: false,
            covers_only: false,
            file_date_on: false,
            file_path_on: false,
            file_size_on: false,
        }
    }
}

impl Settings {
    pub fn pictures_per_row(&self) -> i32 {
        self.current_pictures_per_row
    }

    pub fn single_view(&self) -> bool {
        self.pictures_per_row() == 1
    }

    pub fn thumbnail_view(&self) -> bool {
        self.pictures_per_row() == 10
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

    pub fn blinking_on(&self) -> bool {
        self.blinking_on
    }

    pub fn covers_only(&self) -> bool {
        self.covers_only
    }

    pub fn file_date_on(&self) -> bool {
        self.file_date_on
    }

    pub fn file_path_on(&self) -> bool {
        self.file_path_on
    }

    pub fn file_size_on(&self) -> bool {
        self.file_size_on
    }

    pub fn toggle_pictures_per_row(&mut self, new: i32) -> i32 {
        if new != self.current_pictures_per_row {
            self.last_pictures_per_row = self.current_pictures_per_row;
            self.current_pictures_per_row = new
        } else {
            std::mem::swap(
                &mut self.current_pictures_per_row,
                &mut self.last_pictures_per_row,
            )
        };
        if self.current_pictures_per_row != 1 {
            self.full_size_on = false;
            self.expand_on = false;
        }
        self.current_pictures_per_row
    }

    pub fn toggle_palette(&mut self) {
        self.palette_on = !self.palette_on
    }

    pub fn toggle_expand(&mut self) {}

    pub fn toggle_full_size(&mut self) -> bool {
        if self.pictures_per_row() == 1 {
            self.full_size_on = !self.full_size_on;
        }
        self.full_size_on
    }

    pub fn toggle_blinking(&mut self) -> bool {
        self.blinking_on = !self.blinking_on;
        self.blinking_on
    }

    pub fn toggle_covers_only(&mut self) -> bool {
        self.covers_only = !self.covers_only;
        self.covers_only
    }

    pub fn toggle_file_date(&mut self) -> bool {
        self.file_date_on = !self.file_date_on;
        self.file_date_on
    }

    pub fn toggle_file_path(&mut self) -> bool {
        self.file_path_on = !self.file_path_on;
        self.file_path_on
    }

    pub fn toggle_file_size(&mut self) -> bool {
        self.file_size_on = !self.file_size_on;
        self.file_size_on
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
        assert_eq!(10, settings.toggle_pictures_per_row(1));
        assert!(!settings.full_size_on());
    }

    #[test]
    fn setting_and_toggling_pictures_per_row() {
        let mut settings = Settings::default();
        assert_eq!(10, settings.pictures_per_row());
        assert_eq!(2, settings.toggle_pictures_per_row(2));
        assert_eq!(2, settings.pictures_per_row());
        assert_eq!(10, settings.toggle_pictures_per_row(2));
        assert_eq!(10, settings.pictures_per_row());
        assert_eq!(1, settings.toggle_pictures_per_row(1));
        assert_eq!(1, settings.pictures_per_row());
        assert_eq!(10, settings.toggle_pictures_per_row(1));
        assert_eq!(10, settings.pictures_per_row());
        assert_eq!(2, settings.toggle_pictures_per_row(2));
        assert_eq!(2, settings.pictures_per_row());
        assert_eq!(1, settings.toggle_pictures_per_row(1));
        assert_eq!(1, settings.pictures_per_row());
        assert_eq!(2, settings.toggle_pictures_per_row(1));
        assert_eq!(2, settings.pictures_per_row());
    }

    #[test]
    fn toggling_blinking_on() {
        let mut settings = Settings::default();
        assert!(settings.blinking_on());
        assert!(!settings.toggle_blinking());
        assert!(!settings.blinking_on());
    }

    #[test]
    fn toggling_covers_only() {
        let mut settings = Settings::default();
        assert!(settings.toggle_covers_only());
        assert!(settings.covers_only());
        assert!(!settings.toggle_covers_only());
        assert!(!settings.covers_only());
    }

    #[test]
    fn toggling_file_date_on() {
        let mut settings = Settings::default();
        assert!(settings.toggle_file_date());
        assert!(settings.file_date_on());
    }

    #[test]
    fn toggling_file_path_on() {
        let mut settings = Settings::default();
        assert!(settings.toggle_file_path());
        assert!(settings.file_path_on());
    }

    #[test]
    fn toggling_file_size_on() {
        let mut settings = Settings::default();
        assert!(settings.toggle_file_size());
        assert!(settings.file_size_on());
    }
}
