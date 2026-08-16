use crate::gui::action::Action;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct EditorStatus {
    input: String,
    candidate_list_tip: Option<String>,
    result_action: Option<Action>,
}

impl EditorStatus {
    pub fn new(
        input: &str,
        candidate_list_tip: Option<String>,
        result_action: Option<Action>,
    ) -> Self {
        Self {
            input: input.to_string(),
            candidate_list_tip,
            result_action,
        }
    }

    pub fn no_change(input: &str) -> Self {
        Self {
            input: input.to_string(),
            candidate_list_tip: None,
            result_action: None,
        }
    }

    pub fn candidates(input: &str, candidates: Vec<String>) -> Self {
        Self {
            input: input.to_string(),
            candidate_list_tip: Some("[ ".to_owned() + &candidates.iter().join(" ") + " ]"),
            result_action: None,
        }
    }
    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn candidate_list_tip(&self) -> Option<String> {
        self.candidate_list_tip.clone()
    }

    pub fn result_action(&self) -> Option<Action> {
        self.result_action.clone()
    }
}
