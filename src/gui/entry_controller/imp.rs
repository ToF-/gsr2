use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::sync::OnceLock;

#[derive(Default)]
pub struct EntryController {
    pub entry: std::cell::RefCell<String>,
}

#[gtk::glib::object_subclass]
impl ObjectSubclass for EntryController {
    const NAME: &'static str = "EntryController";
    type Type = super::EntryController;
    type ParentType = gtk::glib::Object;
}

impl ObjectImpl for EntryController {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("entered")
                    .param_types([gtk::glib::types::Type::STRING])
                    .build(),
            ]
        })
    }
}
