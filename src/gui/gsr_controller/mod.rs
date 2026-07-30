mod imp;

use gtk::glib::subclass::prelude::*;
use gtk::prelude::ObjectExt;

gtk::glib::wrapper! {
    pub struct GsrController(ObjectSubclass<imp::GsrController>);
}

impl GsrController {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn entered(&self, key: &str) {
        self.emit_by_name::<()>("entered", &[&key]);
    }
}
