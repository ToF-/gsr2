use crate::MainWindow;
use crate::env::default_values::MAX_LABEL_LENGTH;
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::entry_kind::EntryKind;
use crate::gui::mode::Mode;
use crate::gui::view::entry_window::EntryWindow;
use gdk::Key;
use gtk::{self, gdk};

#[derive(Clone, Debug)]
pub struct Editor {
    editing: bool,
    input: String,
    controls: Controls,
    entry_kind: EntryKind,
    entry_window_opt: Option<EntryWindow>,
}

#[allow(dead_code)]
impl Editor {
    pub fn new() -> Editor {
        Editor {
            editing: false,
            controls: default_controls(),
            input: String::from(""),
            entry_kind: EntryKind::Label,
            entry_window_opt: None,
        }
    }

    pub fn begin(&mut self, main_window: &MainWindow, entry_kind: EntryKind) {
        let prompt: &str = match entry_kind {
            EntryKind::Label => "Enter a label",
            EntryKind::Number => "Enter a number",
            EntryKind::DeleteConfirmation => "Delete these pictures?",
        };
        self.entry_kind = entry_kind;
        self.editing = true;
        self.input = String::from("");
        self.entry_window_opt = Some(main_window.popup_entry_window(&prompt, &self.input));
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn entry_kind(&self) -> EntryKind {
        self.entry_kind.clone()
    }

    pub fn process(&mut self, key: Key) {
        match key.name() {
            None => {}
            Some(key_name) => match self.controls.get(&(key_name.to_string(), Mode::Editing)) {
                Some(Control::CancelEdition) => self.cancel(),
                Some(Control::ConfirmEdition) => self.enter(),
                Some(Control::DeleteChar) => self.delete(),
                Some(_) | None => self.append_from_key(key),
            },
        }
    }

    pub fn append_from_key(&mut self, key: Key) {
        if let Some(ch) = key.to_unicode() {
            self.append(ch);
        }
    }
    pub fn cancel(&mut self) {
        self.input = String::from("");
        self.editing = false;
        self.entry_window_opt.clone().unwrap().close();
    }

    pub fn enter(&mut self) {
        self.entry_window_opt.clone().unwrap().close();
        self.editing = false
    }

    pub fn begin_input(&mut self, kind: EntryKind) {
        self.editing = true;
        self.input = String::from("");
        self.entry_kind = kind
    }

    pub fn confirm_input(&mut self) -> String {
        if self.editing {
            self.editing = false;
            self.input.clone()
        } else {
            String::from("")
        }
    }

    pub fn cancel_input(&mut self) {
        if self.editing {
            self.editing = false;
            self.input = String::from("")
        }
    }
    pub fn append(&mut self, ch: char) {
        let ch_is_ok = match self.entry_kind {
            EntryKind::Number => ch.is_ascii_digit(),
            EntryKind::DeleteConfirmation => matches!(ch, 'e' | 'n' | 'o' | 's' | 'y'),
            EntryKind::Label => matches!(ch,
                'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' '),
        };
        if ch_is_ok && self.input.len() < MAX_LABEL_LENGTH {
            self.input.push(Self::convert_char(ch));
            self.refresh_view();
        }
    }

    fn convert_char(ch: char) -> char {
        match ch {
            ' ' => '-',
            other if other.is_ascii() => other.to_lowercase().next().unwrap(),
            other => other,
        }
    }

    pub fn delete(&mut self) {
        self.input.pop();
        self.refresh_view();
    }

    fn refresh_view(&self) {
        self.entry_window_opt
            .clone()
            .map(|window| window.set_text(&self.input));
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initially_not_editing() {
        let editor = Editor::new();
        assert!(!editor.editing());
        assert_eq!(EntryKind::Label, editor.entry_kind());
    }

    #[test]
    fn after_begin_input_edting_is_true() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        assert!(editor.editing());
        assert_eq!(String::from(""), editor.input());
        assert_eq!(EntryKind::Label, editor.entry_kind());
    }

    #[test]
    fn appending_a_char_changes_the_input() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        editor.append('a');
        assert_eq!(String::from("a"), editor.input.clone());
        editor.append('b');
        assert_eq!(String::from("ab"), editor.input.clone());
        editor.append('0');
        assert_eq!(String::from("ab0"), editor.input.clone());
        editor.append('9');
        assert_eq!(String::from("ab09"), editor.input.clone());
        editor.append('-');
        assert_eq!(String::from("ab09-"), editor.input.clone());
        editor.append('_');
        assert_eq!(String::from("ab09-_"), editor.input.clone());
    }
    #[test]
    fn cannot_append_forbidden_chars() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        editor.append('"');
        editor.append('@');
        editor.append('^');
        assert_eq!(String::from(""), editor.input());
    }

    #[test]
    fn treat_space_as_dash() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        editor.append('a');
        editor.append(' ');
        editor.append('b');
        assert_eq!(String::from("a-b"), editor.input());
    }
    #[test]
    fn treat_uppercase_as_lowercase() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        editor.append('A');
        editor.append('B');
        editor.append('Z');
        assert_eq!(String::from("abz"), editor.input());
    }
    #[test]
    fn deleting_a_char_changes_the_input() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        editor.append('a');
        editor.append('b');
        editor.append('c');
        editor.delete();
        assert_eq!(String::from("ab"), editor.input.clone());
    }

    #[test]
    fn confirming_return_input_and_set_editing_to_false() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        editor.append('a');
        editor.append('b');
        editor.append('c');
        assert_eq!(String::from("abc"), editor.confirm_input());
        assert!(!editor.editing());
    }
    #[test]
    fn cancelling_set_editing_to_false() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        editor.append('a');
        editor.append('b');
        editor.append('c');
        editor.cancel_input();
        assert!(!editor.editing());
    }
    #[test]
    fn label_length_is_limited_to_max_label_length() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label);
        for _ in 0..40 {
            editor.append('a')
        }
        assert_eq!(MAX_LABEL_LENGTH, editor.input.clone().len());
    }
}
