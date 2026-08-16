use crate::gui::action::Action;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::editor::editor_rules::EditorRules;
use crate::gui::editor::editor_status::EditorStatus;
use crate::gui::entry_kind::EntryKind;
use crate::gui::mode::Mode;
use crate::model::tags::Tags;
use std::sync::Arc;

pub mod change_label_editor;
pub mod editor_rules;
pub mod editor_status;
pub mod entry_editor;
pub mod entry_editor_status;
pub mod legacy_editor;
pub mod validator;

pub struct Editor {
    completion_dispenser_opt: Option<CompletionDispenser>,
    accepter: Arc<dyn Fn(String, char) -> bool>,
    converter: Arc<dyn Fn(String, char) -> String>,
}

impl EditorRules for Editor {
    fn new<A: Fn(String, char) -> bool + 'static, C: Fn(String, char) -> String + 'static>(
        entry_kind: EntryKind,
        prompt: &str,
        completion_tags_opt: Option<Tags>,
        action_result: Action,
        accepter: A,
        converter: C,
    ) -> Self {
        Self {
            completion_dispenser_opt: match completion_tags_opt {
                None => None,
                Some(tags) => Some(CompletionDispenser::new_with(tags)),
            },
            accepter: Arc::new(accepter),
            converter: Arc::new(converter),
        }
    }

    fn edit(&self, initial_input: &str, key: gtk::gdk::Key) -> EditorStatus {
        match key.name() {
            None => EditorStatus::no_change(initial_input),
            Some(key_name) => match default_controls().get(&(key_name.to_string(), Mode::Editing)) {
                Some(Control::CancelEdition) => {
                    EditorStatus::new("", None, Some(Action::Cancel))
                }
                Some(Control::Complete) => {
                    if let Some(completion_dispenser) = self.completion_dispenser_opt.as_ref() {
                        let candidates = completion_dispenser.candidates(initial_input);
                        match candidates.len() {
                            0 => EditorStatus::no_change(initial_input),
                            1 => EditorStatus::new(&candidates[0], None, None),
                            _ => EditorStatus::candidates(initial_input, candidates),
                        }
                    } else {
                        EditorStatus::no_change(initial_input)
                    }
                }
                Some(_) | None => {
                    let mut input = initial_input.to_string();
                    let accept = self.accepter.clone();
                    if let Some(ch) = key.to_unicode()
                        && accept(initial_input.to_string(), ch)
                    {
                        let convert = self.converter.clone();
                        let input = convert(initial_input.to_string(), ch);
                        EditorStatus::new(&input, None, None)
                    } else {
                        EditorStatus::no_change(initial_input)
                    }
                }
            },
        }
    }
}
