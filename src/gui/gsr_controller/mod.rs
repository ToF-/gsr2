mod imp;

use gtk::prelude::ObjectExt;
use gtk::glib::subclass::prelude::*;

gtk::glib::wrapper! {
    pub struct MyController(ObjectSubclass<imp::MyController>);
}

impl MyController {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn finish(&self, text: &str) {
        self.emit_by_name::<()>("finished", &[&text]);
    }
}
