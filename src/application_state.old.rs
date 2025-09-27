use crate::control::{Control, Controls, default_controls};
use crate::database::Database;
use crate::direction::Direction;
use crate::editor::Editor;
use crate::environment::database_connection;
use crate::gallery::Gallery;
use crate::navigator::Navigator;
use crate::picture::Picture;
use std::io::Result;

#[derive(Debug)]
pub struct ApplicationState {
    gallery: Gallery,
    navigator: Navigator,
    controls: Controls,
    database: Database,
    editor: Editor,
    pictures_per_row: usize,
    old_pictures_per_row: usize,
    expand_on: bool,
    full_size_on: bool,
    palette_on: bool,
}

impl ApplicationState {
    pub fn new() -> Result<Self> {
        match database_connection() {
            Err(err) => Err(err),
            Ok(connection_string) => match Database::from_connection(&connection_string) {
                Err(err) => Err(err),
                Ok(database) => Ok(ApplicationState {
                    gallery: Gallery::new(),
                    navigator: Navigator::new(0, 1),
                    controls: default_controls(),
                    database,
                    editor: Editor::new(),
                    pictures_per_row: 1,
                    old_pictures_per_row: 1,
                    expand_on: false,
                    full_size_on: false,
                    palette_on: false,
                }),
            },
        }
    }

    pub fn gallery(&self) -> &Gallery {
        &self.gallery
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn editor(&self) -> &Editor {
        &self.editor
    }

    pub fn current_picture(&self) -> Picture {
        self.gallery.picture(self.navigator.position())
    }

    pub fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    pub fn set_editor(&mut self, editor: Editor) {
        self.editor = editor
    }
    pub fn move_towards(&mut self, direction: Direction) {
        self.navigator.move_towards(direction)
    }

    pub fn can_move(&self, direction: Direction) -> bool {
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

    pub fn pictures_per_row(&self) -> usize {
        self.pictures_per_row
    }

    pub fn thumbnails_on(&self) -> bool {
        self.pictures_per_row == 10
    }

    pub fn set_gallery(&mut self, gallery: Gallery, pictures_per_row: usize) {
        self.gallery = gallery;
        self.load_image_data();
        self.navigator = Navigator::new(self.gallery.len(), pictures_per_row);
        self.pictures_per_row = pictures_per_row
    }

    pub fn toggle_single_view(&mut self) {
        if self.pictures_per_row != self.old_pictures_per_row {
            let current_position = self.navigator.position();
            std::mem::swap(&mut self.pictures_per_row, &mut self.old_pictures_per_row);
            self.navigator = Navigator::new(self.gallery.len(), self.pictures_per_row);
            self.navigator.move_towards(Direction::Index {
                value: current_position,
            });
            self.navigator.set_page_changed();
        }
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

    fn load_image_data(&mut self) {
        match self.database.rusqlite_retrieve_all_pictures() {
            Ok(map) => {
                let mut new_pictures: Vec<Picture> = vec![];
                for picture in self.gallery.pictures().iter() {
                    let new_picture = match map.get(&picture.file_path()) {
                        Some(image_data) => {
                            Picture::new_with_image_data(&picture.file_path(), &image_data.label())
                        }
                        None => Picture::new(&picture.file_path()),
                    };
                    new_pictures.push(new_picture)
                }
                let new_gallery = Gallery::new_with_pictures(new_pictures);
                self.gallery = new_gallery;
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn my_app_state() -> ApplicationState {
        ApplicationState::new().expect("cannot create application state")
    }

    #[test]
    fn after_palette_toggle_palette_on_is_inverted() {
        let mut state = my_app_state();
        state.toggle_palette();
        assert_eq!(true, state.palette_on());
    }

    #[test]
    fn after_expand_toggle_expand_on_is_inverted() {
        let mut state = my_app_state();
        state.toggle_expand();
        assert_eq!(true, state.expand_on());
        state.toggle_expand();
        assert_eq!(false, state.expand_on());
    }
    #[test]
    fn after_full_size_toggle_full_size_on_is_inverted() {
        let mut state = my_app_state();
        state.toggle_full_size();
        assert_eq!(true, state.full_size_on());
        state.toggle_full_size();
        assert_eq!(false, state.full_size_on());
    }

    #[test]
    fn get_the_control_matching_a_keyname() {
        let mut state = my_app_state();
        assert_eq!(Some(Control::ToggleFullSize), state.get_control("f"));
    }
    #[test]
    fn setting_the_gallery_and_pictures_per_row_hence_page_size() {
        let mut state = my_app_state();
        state.set_gallery(Gallery::new(), 5);
        assert_eq!(5, state.pictures_per_row);
        assert_eq!(25, state.navigator().page_size());
    }
    #[test]
    fn thumbnails_on_tells_if_10_pictures_per_row() {
        let mut state = ApplicationState::new().expect("cannot create application state");
        state.set_gallery(Gallery::new(), 10);
        assert_eq!(true, state.thumbnails_on())
    }
}
