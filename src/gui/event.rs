use gtk::gdk::{Key, ModifierType};

pub enum Event {
    KeyPressed { key: Key, n_press: u32, modifier_type: ModifierType },
    PaneClicked { button: u32, number: i32 },
    PictureClicked { button: u32, col: i32, row: i32 },

}


