
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::entry_kind::EntryKind;
use crate::gui::validator::Validator;
use crate::gui::view::entry_view::EntryView;
use crate::model::tags::empty_tags;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::sync::OnceLock;

pub struct EntryEditor {
    pub entry: RefCell<String>,
    pub prompt: RefCell<String>,
    pub validator_rc: RefCell<Validator>,
    pub completion_dispenser_rc: RefCell<CompletionDispenser>,
    pub view_opt_rc: RefCell<Option<EntryView>>,
}

impl EntryEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(
        &self,
        entry_view_rc: RefCell<EntryView>,
        validator: Validator,
        completion_dispenser: CompletionDispenser,
    ) {
        if let Ok(entry_view) = entry_view_rc.try_borrow() {
            *self.view_opt_rc.borrow_mut() = Some(entry_view.clone());
            *self.prompt.borrow_mut() = entry_view.prompt();
            *self.validator_rc.borrow_mut() = validator;
            *self.completion_dispenser_rc.borrow_mut() = completion_dispenser;
        } else {
            panic!("can't borrow");
        }
    }

    pub fn view(&self) -> Option<EntryView> {
        self.view_opt_rc.borrow().clone()
    }

    pub fn candidates(&self) -> Option<Vec<String>> {
        let entry = self.entry.borrow();
        let completion_dispenser = self.completion_dispenser_rc.borrow();
        let candidates = completion_dispenser.candidates(&entry);
        if candidates.is_empty() {
            None
        } else {
            Some(candidates)
        }
    }

    pub fn validate_entry(&self, s: &str, ch: char) -> Option<String> {
        self.validator_rc.borrow().validate_entry(s, ch)
    }

    pub fn set_entry(&self, s: &str) {
        *self.entry.borrow_mut() = s.to_string();
        let view_opt = self.view_opt_rc.borrow();
        if let Some(view) = view_opt.as_ref() {
            view.set_input(s)
        }
    }

    pub fn set_prompt(&self) {
        let view_opt = self.view_opt_rc.borrow();
        if let Some(view) = view_opt.as_ref() {
            let s = self.prompt.borrow().to_string();
            view.set_prompt(&s)
        }
    }

    pub fn set_prompt_with_candidates(&self, candidates: Vec<String>) {
        let view_opt = self.view_opt_rc.borrow();
        if let Some(view) = view_opt.as_ref() {
            let s = self.prompt.borrow().to_string();
            view.set_prompt(&(s + " [ " + &candidates.iter().join(" ") + " ] "));
        }
    }
}

impl Default for EntryEditor {
    fn default() -> Self {
        Self {
            entry: RefCell::new(String::new()),
            prompt: RefCell::new(String::new()),
            validator_rc: Validator::new(EntryKind::Information).into(),
            completion_dispenser_rc: CompletionDispenser::new_with(empty_tags()).into(),
            view_opt_rc: RefCell::new(None),
        }
    }
}
#[gtk::glib::object_subclass]
impl ObjectSubclass for EntryEditor {
    const NAME: &'static str = "EntryEditor";
    type Type = super::EntryEditor;
    type ParentType = gtk::glib::Object;
}

impl ObjectImpl for EntryEditor {
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
