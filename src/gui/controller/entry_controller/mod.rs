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

    pub fn new_with(entry_view_rc: RefCell<EntryView>) -> Self {
        let obj = Self::new();
        obj.imp().initialize(entry_view_rc);
        obj
    }

    pub fn view(&self) -> Option<EntryView> {
        self.imp().view()
    }

    pub fn entry(&self) -> String {
        self.imp().entry.borrow().clone()
    }

    pub fn set_entry(&self, text: &str) {
        *self.imp().entry.borrow_mut() = text.to_string()
    }

    // different entry controllers do different things when receiving a key that was pressed, so it's a closure
    pub fn key_pressed(&self, key_name: &str) {
        self.emit_by_name::<()>("key-pressed", &[&key_name]);
    }

    pub fn connect_key_pressed<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&Self, &str) + 'static,
    {
        self.connect_local("key-pressed", false, move |values| {
            let obj = values[0].get::<EntryController>().unwrap();
            let key = values[1].get::<&str>().unwrap();
            f(&obj, key);
            None
        })
    }

    // different entry controllers do different things when closing, so it's a closure
    pub fn close(&self) {
        self.emit_by_name::<()>("closed", &[]);
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
