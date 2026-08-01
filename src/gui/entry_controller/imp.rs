use crate::gui::input_validate::InputValidate;
use crate::gui::view::entry_view::EntryView;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::sync::OnceLock;

pub struct EntryController {
    pub entry: std::cell::RefCell<String>,
    pub validator: InputValidate,
    pub view: EntryView,
}

impl EntryController {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn validate_input(&self, s: &str) -> Option<String> {
        self.validator().validate(s)
    }

    pub fn validator(&self) -> InputValidate {
        self.validator.clone()
    }
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
                Signal::builder("key-pressed")
                    .param_types([glib::Type::U32])
                    .build(),
                Signal::builder("closed").build(),
            ]
        })
    }
}
