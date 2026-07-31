use crate::gui::view::entry_view::EntryView;
use crate::gui::input_validate::InputValidate;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::sync::OnceLock;

pub struct EntryController {
    pub entry: std::cell::RefCell<String>,
    pub validator: InputValidate,
    pub view: EntryView,
}

impl Default for EntryController {
    fn default() -> Self {
        Self {
        entry: std::cell::RefCell::new(String::new()),
        validator: InputValidate::new(),
        view: EntryView::new(),
        }
    }
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
