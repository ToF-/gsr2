use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::validator::Validator;
use crate::gui::view::entry_view::EntryView;
use gtk::glib::subclass::prelude::*;
use gtk::prelude::ObjectExt;
use std::cell::RefCell;
use std::rc::Rc;

mod imp;
pub type RcEntryController = Rc<RefCell<EntryController>>;

gtk::glib::wrapper! {
    pub struct EntryController(ObjectSubclass<imp::EntryController>);
}

impl EntryController {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn new_with(entry_view_rc: RefCell<EntryView>, validator: Validator, completion_dispenser: CompletionDispenser) -> Self {
        let obj = Self::new();
        obj.imp().initialize(entry_view_rc, validator, completion_dispenser);
        obj
    }

    pub fn set_prompt(&self) {
        self.imp().set_prompt()
    }

    pub fn set_prompt_with_candidates(&self, candidates: Vec<String>) {
        self.imp().set_prompt_with_candidates(candidates);
    }
    pub fn view(&self) -> Option<EntryView> {
        self.imp().view()
    }

    pub fn entry(&self) -> String {
        self.imp().entry.borrow().clone()
    }

    pub fn candidates(&self) -> Option<Vec<String>> {
        self.imp().candidates()
    }
    pub fn set_entry(&self, text: &str) {
        self.imp().set_entry(text)
    }
    
    pub fn validate_char(&self, ch: char) -> Option<String> {
        let entry = self.entry();
        if let Some(entry) =  self.imp().validate_entry(&entry, ch) {
            self.set_entry(&entry);
            Some(entry)
        } else {
            None
        }
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

