mod imp;

use crate::gui::view::entry_view::EntryView;
use gtk::glib::subclass::prelude::*;
use gtk::prelude::ObjectExt;
use std::cell::RefCell;
use std::rc::Rc;

pub type RcEntryController = Rc<RefCell<EntryController>>;

gtk::glib::wrapper! {
    pub struct EntryController(ObjectSubclass<imp::EntryController>);
}

impl EntryController {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn key_pressed(&self, key: u32) {
        self.emit_by_name::<()>("entered", &[&key]);
    }

    pub fn close(&self) {
        self.emit_by_name::<()>("closed", &[]);
    }

    pub fn entry(&self) -> String {
        self.imp().entry.borrow().clone()
    }

    pub fn set_entry(&self, text: &str) {
        *self.imp().entry.borrow_mut() = text.to_string()
    }

    pub fn connect_key_pressed<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&Self, u32) + 'static,
    {
        self.connect_local("entered", false, move |values| {
            let obj = values[0].get::<EntryController>().unwrap();
            let key = values[1].get::<u32>().unwrap();
            f(&obj, key);
            None
        })
    }

    pub fn connect_closed<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&EntryController) + 'static,
    {
        self.connect_local("closed", false, move |values| {
            let obj = values[0].get::<EntryController>().unwrap();
            f(&obj);
            None
        })
    }
}
