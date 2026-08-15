use crate::gui::action::Action;

#[derive(Debug, Clone)]
pub struct EntryEditorStatus {
    input: String,
    candidate_list_tip: Option<String>,
    result_action: Option<Action>,
}

impl EntryEditorStatus {
    pub fn new(input: &str, candidate_list_tip: Option<String>, result_action: Option<Action>) -> Self {
        Self {
            input: input.to_string(),
            candidate_list_tip,
            result_action,
        }
    }
    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn candidate_list_tip(&self) -> Option<String> {
        self.candidate_list_tip.clone()
    }

    pub fn result_action(&self) -> Option<Action> {
        self.result_action()
    }
}
