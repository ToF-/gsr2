use gtk::{Align, Orientation};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GsrPictureCellBox(ObjectSubclass<imp::GsrPictureCellBox>)
        @extends gtk::Widget, gtk::Box,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::Orientable,
            gtk::ConstraintTarget;
}

impl GsrPictureCellBox {
    pub fn new(col: i32, row: i32) -> Self {
        let obj: Self = glib::Object::new();
        obj.initialize(col, row);
        obj
    }

}
impl GsrPictureCellBox {
    pub fn initialize(&self, col: i32, row: i32) {
        self.set_orientation(Orientation::Vertical);
        self.set_spacing(0);
        self.set_valign(Align::Center);
        self.set_halign(Align::Center);
        self.set_hexpand(true);
        self.set_vexpand(true);

        self.set_col(col);
        self.set_row(row);
    }
}
