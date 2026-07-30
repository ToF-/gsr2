mod imp;

use gtk::glib::subclass::prelude::*;
use gtk::prelude::ObjectExt;

gtk::glib::wrapper! {
    pub struct EntryController(ObjectSubclass<imp::EntryController>);
}

impl EntryController {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn enter(&self, key_name: &str) {
        self.emit_by_name::<()>("entered", &[&key_name]);
    }

    pub fn entry(&self) -> String {
        self.imp().entry.borrow().clone()
    }

    pub fn set_entry(&self, text: &str) {
        *self.imp().entry.borrow_mut() = text.to_string()
    }
    pub fn connect_entered<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&Self, &str) + 'static,
    {
        self.connect_local("entered", false, move |values| {
            let obj = values[0].get::<EntryController>().unwrap();
            let text = values[1].get::<String>().unwrap();
            f(&obj, &text);
            None
        })
    }
}
