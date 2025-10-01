use gtk::gdk::{Key, ModifierType};

#[derive(Debug, Clone)]
pub enum Event {
    KeyPressed {
        key: Key,
        key_code: u32,
        modifier_type: ModifierType,
    },
    NextSlideDelay,
    PaneClicked {
        button: usize,
        pane_number: usize,
    },
    PictureClicked {
        button: u32,
        col: i32,
        row: i32,
    },
}
