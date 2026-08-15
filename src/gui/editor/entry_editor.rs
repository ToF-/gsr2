use crate::gui::action::Action;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::entry_kind::EntryKind;
use crate::gui::validator::Validator;

#[derive(Debug, Clone)]
pub struct EntryEditorStatus {
    input: String,
    candidate_list_tip: Option<String>,
    result_action: Option<Action>,
}
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
}

