use crate::gui::editor::entry_editor_status::EntryEditorStatus;use crate::model::tags::tags_from_str;
use crate::model::tags::Tags;
use crate::gui::action::Action;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::entry_kind::EntryKind;
use crate::gui::validator::Validator;

#[derive(Debug, Clone)]
pub struct EntryEditor {
    entry_kind: EntryKind,
    validator: Validator,
    completion_dispenser_opt: Option<CompletionDispenser>,
}

impl EntryEditor {
    pub fn new(entry_kind: EntryKind,
        validator: Validator,
        completion_dispenser_opt: Option<CompletionDispenser>) -> Self {
        Self {
            entry_kind,
            validator,
            completion_dispenser_opt,
        }
    }

    pub fn edit(&self, input: &str, key_name: &str) -> EntryEditorStatus {
        let mut input = input.to_string();
        input.push_str(key_name);
        EntryEditorStatus::new(
            &input,
            None,
            None,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn editor_for_entry_kind_label() {
        let entry_editor = EntryEditor::new(
            EntryKind::Label,
            Validator::new(EntryKind::Label),
            Some(CompletionDispenser::new_with(tags_from_str("foo,bar,barnaby,lab"))),
            );
        let status = entry_editor.edit("f", "o");
        assert_eq!("fo", &status.input());
        let status = entry_editor.edit("b", "a");
        assert_eq!("ba", &status.input());
    }
}

