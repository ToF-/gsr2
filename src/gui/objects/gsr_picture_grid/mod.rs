use crate::model::picture::Picture;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GsrPictureGrid(ObjectSubclass<imp::GsrPictureGrid>)
        @extends gtk::Widget, gtk::Grid,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::Orientable,
            gtk::ConstraintTarget;
}

impl GsrPictureGrid {
    pub fn new(pictures_per_row: i32, focus_at_coords: (i32, i32), palette_on: bool,) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().initialize(pictures_per_row, focus_at_coords, palette_on);
        obj
    }

    pub fn set_picture_at(col: i32, row: i32, picture: &Picture, picture_index: usize) {
    }

    pub fn set_focus_at(col: i32, row: i32) {
    }

    pub fn set_palette_on() {
    }

}
