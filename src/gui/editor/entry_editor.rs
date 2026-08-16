use crate::gui::action::Action;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::editor::Control;
use crate::gui::editor::Controls;
use crate::gui::editor::default_controls;
use crate::gui::editor::entry_editor_status::EntryEditorStatus;
use crate::gui::entry_kind::EntryKind;
use crate::gui::mode::Mode;
use crate::gui::validator::Validator;
use crate::model::tags::Tags;
use crate::model::tags::tags_from_str;
use gtk::gdk::Key;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct EntryEditor {
    entry_kind: EntryKind,
    controls: Controls,
    validator: Validator,
    completion_dispenser_opt: Option<CompletionDispenser>,
}

impl EntryEditor {
    pub fn new(
        entry_kind: EntryKind,
        validator: Validator,
        completion_dispenser_opt: Option<CompletionDispenser>,
    ) -> Self {
        Self {
            entry_kind,
            controls: default_controls(),
            validator,
            completion_dispenser_opt,
        }
    }

    fn no_change(&self, input: &str) -> EntryEditorStatus {
        EntryEditorStatus::new(input, None, None)
    }

    fn candidates(&self, input: &str, candidates: Vec<String>) -> EntryEditorStatus {
        EntryEditorStatus::new(
            input,
            Some("[ ".to_owned() + &candidates.iter().join(" ") + " ]"),
            None,
        )
    }

    fn complete(&self, input: &str) -> EntryEditorStatus {
        if let Some(completion_dispenser) = self.completion_dispenser_opt.as_ref() {
            let candidates = completion_dispenser.candidates(input);
            match candidates.len() {
                0 => self.no_change(input),
                1 => EntryEditorStatus::new(&candidates[0], None, None),
                _ => self.candidates(input, candidates),
            }
        } else {
            self.no_change(input)
        }
    }

    fn append_char_from_key(&self, input: &str, key: Key) -> EntryEditorStatus {
        if let Some(ch) = key.to_unicode() {
            let mut input = input.to_string();
            input.push(ch);
            EntryEditorStatus::new(&input, None, None)
        } else {
            self.no_change(input)
        }
    }

    pub fn edit(&self, input: &str, key: Key) -> EntryEditorStatus {
        match key.name() {
            None => EntryEditorStatus::new(input, None, None),
            Some(key_name) => match self.controls.get(&(key_name.to_string(), Mode::Editing)) {
                Some(Control::Complete) => self.complete(input),
                Some(_) | None => self.append_char_from_key(input, key),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn editor_for_entry_kind_label_simple_key_strokes() {
        let entry_editor = EntryEditor::new(
            EntryKind::Label,
            Validator::new(EntryKind::Label),
            Some(CompletionDispenser::new_with(tags_from_str(
                "foo,bar,barnaby,lab",
            ))),
        );
        let status = entry_editor.edit("f", Key::from_name("o").unwrap());
        assert_eq!("fo", &status.input());
        let status = entry_editor.edit("b", Key::from_name("a").unwrap());
        assert_eq!("ba", &status.input());
    }

    #[test]
    fn editor_for_entry_kind_label_tab_key() {
        let entry_editor = EntryEditor::new(
            EntryKind::Label,
            Validator::new(EntryKind::Label),
            Some(CompletionDispenser::new_with(tags_from_str(
                "foo,bar,barnaby,lab",
            ))),
        );
        let status = entry_editor.edit("f", Key::from_name("Tab").unwrap());
        assert_eq!("f", &status.input());
        assert_eq!(None, status.candidate_list_tip());

        let status = entry_editor.edit("fo", Key::from_name("Tab").unwrap());
        assert_eq!("foo", &status.input());
        assert_eq!(None, status.candidate_list_tip());

        let status = entry_editor.edit("ba", Key::from_name("Tab").unwrap());
        assert_eq!("ba", &status.input());
        assert_eq!(
            Some("[ bar barnaby ]".to_string()),
            status.candidate_list_tip()
        );

        let status = entry_editor.edit("bar", Key::from_name("Tab").unwrap());
        assert_eq!("bar", &status.input());
        assert_eq!(
            Some("[ bar barnaby ]".to_string()),
            status.candidate_list_tip()
        );
    }
}
