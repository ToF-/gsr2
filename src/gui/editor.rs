use crate::MainWindow;
use crate::env::default_values::MAX_LABEL_LENGTH;
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::entry_kind::EntryKind;
use crate::gui::mode::Mode;
use crate::gui::view::entry_window::EntryWindow;
use crate::model::order::Order;
use crate::model::tags::{Tags, empty};
use gdk::Key;
use gtk::{self, gdk};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Editor {
    prompt: String,
    editing: bool,
    input: String,
    controls: Controls,
    entry_kind: EntryKind,
    entry_window_opt: Option<EntryWindow>,
    choice: Tags,
}

#[allow(dead_code)]
impl Editor {
    pub fn new() -> Editor {
        Editor {
            prompt: "".to_string(),
            editing: false,
            controls: default_controls(),
            input: String::from(""),
            entry_kind: EntryKind::Label,
            entry_window_opt: None,
            choice: empty(),
        }
    }

    pub fn begin(
        &mut self,
        main_window: &MainWindow,
        entry_kind: EntryKind,
        choice_opt: Option<Tags>,
    ) {
        let prompt: &str = match entry_kind {
            EntryKind::Label => "Enter a label",
            EntryKind::AddTag => "Enter a new tag to add",
            EntryKind::RemoveTag => "Enter a tag to remove",
            EntryKind::Number => "Enter a number",
            EntryKind::Order => {
                "Enter a sorting criteria: c)olors d)ate l)abel n)ame p)alette co)ver r)andom s)ize v)alue "
            }
            EntryKind::DeleteConfirmation => "Delete these pictures?",
            EntryKind::MoveConfirmation => "Move these pictures?",
            EntryKind::MoveToLabelConfirmation(ref target) => &format!("Move these pictures to {} ?", target),
            EntryKind::Find => "Enter a part of the picture file name",
            EntryKind::FindLabel => "Enter a part of the picture label",
            EntryKind::Information => "Current picture",
            EntryKind::Help => "Keyboard shortcuts",
            EntryKind::SetSelection => "Enter tags to define the selection",
            EntryKind::SetRestriction => "Enter tags to define the restriction",
        };
        self.prompt = prompt.to_string();
        self.begin_input(entry_kind, choice_opt);
        self.entry_window_opt = Some(main_window.popup_entry_window(&self.prompt, &self.input));
    }

    pub fn begin_input(&mut self, kind: EntryKind, choice_opt: Option<Tags>) {
        self.entry_kind = kind;
        if let Some(choice) = choice_opt {
            self.choice = choice.clone()
        };
        self.editing = true;
        self.input = String::from("");
    }

    pub fn editing(&self) -> bool {
        self.editing
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
        self.refresh_view();
    }

    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
        self.refresh_prompt(prompt);
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
                Some(Control::Complete) => self.complete(),
                Some(_) | None => self.append_from_key(key),
            },
        }
    }

    pub fn append_from_key(&mut self, key: Key) {
        if let Some(ch) = key.to_unicode() {
            self.append(ch);
            self.refresh_prompt(&self.prompt)
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

    pub fn complete(&mut self) {
        let candidates = self.candidates();
        match candidates.len() {
            0 => self.refresh_prompt(&self.prompt),
            1 => {
                let words: Vec<&str> = self.input.split(',').collect();
                let (_, firsts) = words.split_last().unwrap();
                let candidate = candidates[0].clone();
                self.input = match firsts.len() {
                    0 => candidate,
                    1 => format!("{},{}", firsts[0], candidate),
                    _ => format!("{},{}", firsts.join(","), candidate),
                };
                self.refresh_prompt(&self.prompt);
                self.refresh_view()
            }
            _ => self.refresh_prompt(
                &(self.prompt.clone() + " [ " + &candidates.iter().join(" ") + " ] "),
            ),
        }
    }

    pub fn candidates(&self) -> Vec<String> {
        let mut words = self.input.split(',');
        if let Some(last) = words.next_back() {
            if !self.choice.is_empty() && last.len() >= 2 {
                let mut result: Vec<String> = vec![];
                for candidate in self.choice.clone() {
                    if candidate.starts_with(last) {
                        result.push(candidate)
                    }
                }
                result.sort();
                result
            } else {
                vec![]
            }
        } else {
            vec![]
        }
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
        if self.entry_kind == EntryKind::Information {
            return;
        };
        let ch_is_ok = match self.entry_kind {
            EntryKind::Number => ch.is_ascii_digit(),
            EntryKind::DeleteConfirmation | EntryKind::MoveConfirmation | EntryKind::MoveToLabelConfirmation(_) => {
                matches!(ch, 'e' | 'n' | 'o' | 's' | 'y')
            }
            | EntryKind::Find
            | EntryKind::FindLabel => {
                matches!(ch,
                    'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | '^' | '$' | '.' | '*' | '/' | '{' | '}' | '[' | ']' | '(' | ')' | '\\' )
            }
            EntryKind::Label
            | EntryKind::AddTag
            | EntryKind::RemoveTag => {
                matches!(ch,
                'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ')
            }
            EntryKind::SetSelection | EntryKind::SetRestriction => matches!(ch,
                'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | ',' ),
            EntryKind::Order => matches!(ch, 'c' | 'd' | 'p' | 'l' | 'n' | 'o' | 'r' | 's' | 'v'),
            EntryKind::Information | EntryKind::Help => false,
        };
        if ch_is_ok && self.input.len() < MAX_LABEL_LENGTH {
            self.convert_char(ch);
            self.refresh_view();
            self.refresh_prompt(&self.prompt);
        }
    }

    fn convert_char(&mut self, ch: char) {
        match ch {
            ' ' if self.entry_kind == EntryKind::SetSelection => self.input.push(','),
            ' ' if self.entry_kind == EntryKind::SetRestriction => self.input.push(','),
            ' ' => self.input.push('-'),
            c if self.entry_kind == EntryKind::Order => {
                let order: Order = match c {
                    'c' => Order::ColorCount,
                    'd' => Order::Date,
                    'l' => Order::Label,
                    'n' => Order::Name,
                    'o' => Order::Cover,
                    'p' => Order::Palette,
                    'r' => Order::Random,
                    's' => Order::Size,
                    'v' => Order::Value,
                    _ => todo!(),
                };
                self.input = format!("{}", order);
            }
            other if other.is_ascii() => self.input.push(other.to_lowercase().next().unwrap()),
            other => self.input.push(other),
        }
    }

    pub fn delete(&mut self) {
        if self.entry_kind == EntryKind::Information {
            return;
        };
        if self.entry_kind == EntryKind::Order {
            self.input = String::from("");
        } else {
            let _ = self.input.pop();
        }
        self.refresh_prompt(&self.prompt);
        self.refresh_view();
    }

    fn refresh_prompt(&self, prompt: &str) {
        if let Some(entry_window) = &self.entry_window_opt {
            entry_window.set_prompt(prompt)
        }
    }

    fn refresh_view(&self) {
        if let Some(window) = &self.entry_window_opt {
            window.set_text(&self.input);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tags::tags_from_str;

    #[test]
    fn initially_not_editing() {
        let editor = Editor::new();
        assert!(!editor.editing());
        assert_eq!(EntryKind::Label, editor.entry_kind());
    }

    #[test]
    fn after_begin_input_edting_is_true() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
        assert!(editor.editing());
        assert_eq!(String::from(""), editor.input());
        assert_eq!(EntryKind::Label, editor.entry_kind());
    }

    #[test]
    fn appending_a_char_changes_the_input() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
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
        editor.begin_input(EntryKind::Label, None);
        editor.append('"');
        editor.append('@');
        editor.append('^');
        assert_eq!(String::from(""), editor.input());
    }

    #[test]
    fn treat_space_as_dash() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
        editor.append('a');
        editor.append(' ');
        editor.append('b');
        assert_eq!(String::from("a-b"), editor.input());
    }
    #[test]
    fn treat_uppercase_as_lowercase() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
        editor.append('A');
        editor.append('B');
        editor.append('Z');
        assert_eq!(String::from("abz"), editor.input());
    }
    #[test]
    fn deleting_a_char_changes_the_input() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
        editor.append('a');
        editor.append('b');
        editor.append('c');
        editor.delete();
        assert_eq!(String::from("ab"), editor.input.clone());
    }

    #[test]
    fn confirming_return_input_and_set_editing_to_false() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
        editor.append('a');
        editor.append('b');
        editor.append('c');
        assert_eq!(String::from("abc"), editor.confirm_input());
        assert!(!editor.editing());
    }
    #[test]
    fn cancelling_set_editing_to_false() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
        editor.append('a');
        editor.append('b');
        editor.append('c');
        editor.cancel_input();
        assert!(!editor.editing());
    }
    #[test]
    fn label_length_is_limited_to_max_label_length() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, None);
        for _ in 0..40 {
            editor.append('a')
        }
        assert_eq!(MAX_LABEL_LENGTH, editor.input.clone().len());
    }

    #[test]
    fn no_candidates_when_input_is_empty() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, Some(tags_from_str("bar,foo,qux,zoo")));
        assert!(editor.candidates().is_empty())
    }
    #[test]
    fn no_candidates_when_input_is_one_char() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, Some(tags_from_str("bar,foo,qux,zoo")));
        editor.append('b');
        assert!(editor.candidates().is_empty())
    }
    #[test]
    fn possibles_candidates_when_input_is_two_chars_prefixing_a_choice() {
        let mut editor = Editor::new();
        editor.begin_input(
            EntryKind::Label,
            Some(tags_from_str("bar,foo,qux,zone,zoo")),
        );
        editor.append('b');
        editor.append('a');
        assert_eq!(vec!["bar".to_string()], editor.candidates());
        editor.delete();
        editor.delete();
        editor.append('f');
        editor.append('o');
        assert_eq!(vec!["foo".to_string()], editor.candidates());
        editor.delete();
        editor.delete();
        editor.append('z');
        editor.append('o');
        assert_eq!(
            vec!["zone".to_string(), "zoo".to_string()],
            editor.candidates()
        );
    }
    #[test]
    fn no_candidates_when_no_prefix() {
        let mut editor = Editor::new();
        editor.begin_input(EntryKind::Label, Some(tags_from_str("bar,foo,qux,zoo")));
        editor.append('a');
        editor.append('b');
        assert!(editor.candidates().is_empty())
    }
    #[test]
    fn possibles_candidates_when_input_is_two_chars_prefixing_a_choice_after_a_first_selection_item()
     {
        let mut editor = Editor::new();
        editor.begin_input(
            EntryKind::SetSelection,
            Some(tags_from_str("bar,foo,qux,zone,zoo")),
        );
        editor.append('b');
        editor.append('a');
        assert_eq!("ba", editor.input);
        assert_eq!(vec!["bar".to_string()], editor.candidates());
        editor.complete();
        assert_eq!("bar", editor.input);
        editor.append(' ');
        editor.append('f');
        editor.append('o');
        assert_eq!("bar,fo", editor.input);
        assert_eq!(vec!["foo".to_string()], editor.candidates());
    }
}
