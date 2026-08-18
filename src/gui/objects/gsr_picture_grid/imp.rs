use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::Cell;

pub struct GsrPictureGrid {
    pub focus_at_coords: Cell<Option<(i32, i32)>>,
    pub palette_on: Cell<bool>,
    pub pictures_per_row: Cell<i32>,
}

impl Default for GsrPictureGrid {
    fn default() -> Self {
        Self {
            focus_at_coords: Cell::new(None),
            palette_on: Cell::new(false),
            pictures_per_row: Cell::new(10),
        }
    }
}
impl GsrPictureGrid {
    pub fn initialize(&self, pictures_per_row: i32, focus_at_coords: (i32, i32), palette_on: bool) {
        dbg!(focus_at_coords);
        self.focus_at_coords.set(Some(focus_at_coords));
        self.pictures_per_row.set(pictures_per_row);
        self.palette_on.set(palette_on);
    }
}
#[glib::object_subclass]
impl ObjectSubclass for GsrPictureGrid {
    const NAME: &'static str = "GsrPictureGrid";
    type Type = super::GsrPictureGrid;
    type ParentType = gtk::Grid;
}

impl ObjectImpl for GsrPictureGrid {}

impl WidgetImpl for GsrPictureGrid {}

impl GridImpl for GsrPictureGrid {}
