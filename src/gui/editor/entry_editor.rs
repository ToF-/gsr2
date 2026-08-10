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
use std::rc::Rc;
use std::sync::OnceLock;

pub type RcEntryEditor = Rc<RefCell<EntryEditor>>;
#[derive(Debug, Clone)]
pub struct EntryEditor {
    pub entry_rc: RefCell<String>,
    pub prompt_rc: RefCell<String>,
    pub validator_rc: RefCell<Validator>,
    pub completion_dispenser_rc: RefCell<CompletionDispenser>,
    pub view_opt_rc: RefCell<Option<EntryView>>,
}

impl EntryEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with(
        entry_view_rc: RefCell<EntryView>,
        validator: Validator,
        completion_dispenser: CompletionDispenser,
    ) -> Self {
        let obj = Self::default();
        obj.initialize(entry_view_rc, validator, completion_dispenser);
        obj
    }

    pub fn initialize(
        &self,
        entry_view_rc: RefCell<EntryView>,
        validator: Validator,
        completion_dispenser: CompletionDispenser,
    ) {
        if let Ok(entry_view) = entry_view_rc.try_borrow() {
            *self.view_opt_rc.borrow_mut() = Some(entry_view.clone());
            *self.prompt_rc.borrow_mut() = entry_view.prompt();
            *self.validator_rc.borrow_mut() = validator;
            *self.completion_dispenser_rc.borrow_mut() = completion_dispenser;
        } else {
            panic!("can't borrow");
        }
    }

    pub fn view(&self) -> Option<EntryView> {
        self.view_opt_rc.borrow().clone()
    }

    pub fn entry(&self) -> String {
        self.entry_rc.borrow().clone()
    }

    fn edit_return(&self) {}

    pub fn key_pressed(&self, key_name: &str) {}
    fn edit_backspace(&self) {
        if self.entry().len() > 0 {
            let mut entry = self.entry();
            entry.pop();
            self.set_entry(&entry);
            self.set_prompt();
        }
    }

    fn edit_tab(&self) {
        if let Some(candidates) = self.candidates() {
            if candidates.len() == 1 {
                self.set_entry(&candidates[0]);
                self.set_prompt();
            } else {
                self.set_prompt_with_candidates(candidates)
            }
        }
    }
    fn edit_escape(&self) {
        self.set_entry("");
    }

    fn edit_key(&self, key_name: &str) {
        if let Some(key) = gtk::gdk::Key::from_name(key_name)
            && let Some(ch) = key.to_unicode()
            && let Some(entry) = self.validate_char(ch)
        {
            self.set_entry(&entry);
            self.set_prompt()
        }
    }
    pub fn edit_entry(&self, key_name: &str) {
        if key_name == "Escape" {
            self.edit_escape()
        } else if key_name == "Return" {
            self.edit_return()
        } else if key_name == "BackSpace" {
            self.edit_backspace()
        } else if key_name == "Tab" {
            self.edit_tab()
        } else {
            self.edit_key(key_name)
        }
    }
    pub fn candidates(&self) -> Option<Vec<String>> {
        let entry = self.entry_rc.borrow();
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
        *self.entry_rc.borrow_mut() = s.to_string();
        let view_opt = self.view_opt_rc.borrow();
        if let Some(view) = view_opt.as_ref() {
            view.set_input(s)
        }
    }

    pub fn set_prompt(&self) {
        let view_opt = self.view_opt_rc.borrow();
        if let Some(view) = view_opt.as_ref() {
            let s = self.prompt_rc.borrow().to_string();
            view.set_prompt(&s)
        }
    }

    pub fn validate_char(&self, ch: char) -> Option<String> {
        let entry = self.entry();
        if let Some(entry) = self.validate_entry(&entry, ch) {
            self.set_entry(&entry);
            Some(entry)
        } else {
            None
        }
    }
    pub fn set_prompt_with_candidates(&self, candidates: Vec<String>) {
        let view_opt = self.view_opt_rc.borrow();
        if let Some(view) = view_opt.as_ref() {
            let s = self.prompt_rc.borrow().to_string();
            view.set_prompt(&(s + " [ " + &candidates.iter().join(" ") + " ] "));
        }
    }
}

impl Default for EntryEditor {
    fn default() -> Self {
        Self {
            entry_rc: RefCell::new(String::new()),
            prompt_rc: RefCell::new(String::new()),
            validator_rc: Validator::new(EntryKind::Information).into(),
            completion_dispenser_rc: CompletionDispenser::new_with(empty_tags()).into(),
            view_opt_rc: RefCell::new(None),
        }
    }
}
