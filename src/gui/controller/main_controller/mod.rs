mod imp;

use gtk::glib::subclass::prelude::*;
use gtk::prelude::ObjectExt;
use std::cell::RefCell;
use std::rc::Rc;

pub type RcMainController = Rc<RefCell<MainController>>;

gtk::glib::wrapper! {
    pub struct MainController(ObjectSubclass<imp::MainController>);
}

impl MainController {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn entered(&self, key: &str) {
        self.emit_by_name::<()>("entered", &[&key]);
    }
}
