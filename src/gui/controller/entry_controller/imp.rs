use crate::gui::input_validate::InputValidate;
use crate::gui::view::entry_view::EntryView;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::cell::RefCell;
use std::sync::OnceLock;

pub struct EntryController {
    pub entry: RefCell<String>,
    pub prompt: RefCell<String>,
    pub validator: InputValidate,
    pub view_opt_rc: RefCell<Option<EntryView>>,
}

impl EntryController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&self, entry_view_rc: RefCell<EntryView>) {
        if let Ok(entry_view) = entry_view_rc.try_borrow() {
            *self.view_opt_rc.borrow_mut() = Some(entry_view.clone())
        } else {
            panic!("can't borrow");
        }
    }

    pub fn view(&self) -> Option<EntryView> {
        self.view_opt_rc.borrow().clone()
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
            entry: RefCell::new(String::new()),
            prompt: RefCell::new(String::new()),
            validator: InputValidate::new(),
            view_opt_rc: RefCell::new(None),
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
                    .param_types([glib::Type::STRING])
                    .build(),
                Signal::builder("closed").build(),
            ]
        })
    }
}
