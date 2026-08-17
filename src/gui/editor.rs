use crate::gui::action::Action;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::editor::editor_mode::EditorMode;
use crate::gui::editor::editor_rules::EditorRules;
use crate::gui::editor::editor_status::EditorStatus;
use crate::gui::entry_kind::EntryKind;
use crate::gui::mode::Mode;
use crate::model::tags::Tags;
use std::sync::Arc;

pub mod change_label_editor;
pub mod display_information_editor;
pub mod editor_mode;
pub mod editor_rules;
pub mod editor_status;
pub mod entry_editor;
pub mod entry_editor_status;
pub mod legacy_editor;
pub mod validator;

pub struct Editor {
    prompt: String,
    completion_dispenser_opt: Option<CompletionDispenser>,
    editor_mode: EditorMode,
    accepter: Arc<dyn Fn(String, char) -> bool>,
    converter: Arc<dyn Fn(String, char) -> String>,
    launcher: Arc<dyn Fn(String) -> Action>,
}

impl Editor {
    pub fn prompt(&self) -> String {
        self.prompt.clone()
    }
    pub fn editor_mode(&self) -> EditorMode {
        self.editor_mode.clone()
    }
}
impl EditorRules for Editor {
    fn new<
        A: Fn(String, char) -> bool + 'static,
        C: Fn(String, char) -> String + 'static,
        L: Fn(String) -> Action + 'static,
    >(
        prompt: &str,
        completion_tags_opt: Option<Tags>,
        editor_mode: EditorMode,
        accepter: A,
        converter: C,
        launcher: L,
    ) -> Self {
        Self {
            prompt: prompt.to_string(),
            completion_dispenser_opt: match completion_tags_opt {
                None => None,
                Some(tags) => Some(CompletionDispenser::new_with(tags)),
            },
            editor_mode: editor_mode.clone(),
            accepter: Arc::new(accepter),
            converter: Arc::new(converter),
            launcher: Arc::new(launcher),
        }
    }

    fn edit(&self, initial_input: &str, key: gtk::gdk::Key) -> EditorStatus {
        match key.name() {
            None => EditorStatus::no_change(initial_input),
            Some(key_name) => {
                match default_controls().get(&(key_name.to_string(), Mode::Editing)) {
                    Some(Control::CancelEdition) => {
                        EditorStatus::new("", None, Some(Action::Cancel))
                    }
                    Some(Control::ConfirmEdition) => {
                        let launch = self.launcher.clone();
                        let action = launch(initial_input.to_string());
                        EditorStatus::new(initial_input, None, Some(action))
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
                    Some(_) | None => match self.editor_mode {
                        EditorMode::Information => {
                            EditorStatus::new(initial_input, None, Some(Action::Dismiss))
                        }
                        EditorMode::Menu => {
                            let accept = self.accepter.clone();
                            if let Some(ch) = key.to_unicode()
                                && accept(initial_input.to_string(), ch)
                            {
                                let convert = self.converter.clone();
                                let launch = self.launcher.clone();
                                let input = convert(initial_input.to_string(), ch);
                                let action = launch(input);
                                EditorStatus::new("", None, Some(action))
                            } else {
                                EditorStatus::no_change(initial_input)
                            }
                        }
                        EditorMode::Entry => {
                            let input = initial_input.to_string();
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
    }
}
