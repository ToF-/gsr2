use crate::gui::key_input::key_input_mode::KeyInputMode;
use crate::gui::key_input::key_input_rules::KeyInputRules;
use crate::gui::key_input::key_input_status::KeyInputStatus;

use crate::gui::action::Action;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::control::{Control, default_controls};
use crate::gui::mode::Mode;
use crate::model::tags::Tags;
use std::sync::Arc;

pub mod entry;
pub mod information;
pub mod key_input_mode;
pub mod key_input_rules;
pub mod key_input_status;
pub mod menu;

#[derive(Clone)]
pub struct KeyInput {
    prompt: String,
    completion_dispenser_opt: Option<CompletionDispenser>,
    key_input_mode: KeyInputMode,
    accepter: Arc<dyn Fn(String, char) -> bool>,
    converter: Arc<dyn Fn(String, char) -> String>,
    launcher: Arc<dyn Fn(String) -> Action>,
}

impl KeyInput {
    pub fn prompt(&self) -> String {
        self.prompt.clone()
    }
    pub fn key_input_mode(&self) -> KeyInputMode {
        self.key_input_mode.clone()
    }
}
impl KeyInputRules for KeyInput {
    fn new<
        A: Fn(String, char) -> bool + 'static,
        C: Fn(String, char) -> String + 'static,
        L: Fn(String) -> Action + 'static,
    >(
        prompt: &str,
        completion_tags_opt: Option<Tags>,
        key_input_mode: KeyInputMode,
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
            key_input_mode: key_input_mode.clone(),
            accepter: Arc::new(accepter),
            converter: Arc::new(converter),
            launcher: Arc::new(launcher),
        }
    }

    fn edit(&self, initial_input: &str, key: gtk::gdk::Key) -> KeyInputStatus {
        match key.name() {
            None => KeyInputStatus::no_change(initial_input),
            Some(key_name) => {
                match default_controls().get(&(key_name.to_string(), Mode::Editing)) {
                    Some(Control::CancelEdition) => {
                        println!("cancelling…");
                        KeyInputStatus::new("", None, Some(Action::Cancel))
                    }
                    Some(Control::ConfirmEdition) => {
                        let launch = self.launcher.clone();
                        let action = launch(initial_input.to_string());
                        KeyInputStatus::new(initial_input, None, Some(action))
                    }
                    Some(Control::DeleteChar) => {
                        let mut input = initial_input.to_string();
                        let _ = input.pop();
                        KeyInputStatus::new(&input, None, None)
                    }
                    Some(Control::Complete) => {
                        if let Some(completion_dispenser) = self.completion_dispenser_opt.as_ref() {
                            let candidates = completion_dispenser.candidates(initial_input);
                            match candidates.len() {
                                0 => KeyInputStatus::no_change(initial_input),
                                1 => KeyInputStatus::new(&candidates[0], None, None),
                                _ => KeyInputStatus::candidates(initial_input, candidates),
                            }
                        } else {
                            KeyInputStatus::no_change(initial_input)
                        }
                    }
                    Some(_) | None => match self.key_input_mode {
                        KeyInputMode::Information => {
                            KeyInputStatus::new(initial_input, None, Some(Action::Dismiss))
                        }
                        KeyInputMode::Menu => {
                            let accept = self.accepter.clone();
                            if let Some(ch) = key.to_unicode()
                                && accept(initial_input.to_string(), ch)
                            {
                                let convert = self.converter.clone();
                                let launch = self.launcher.clone();
                                let input = convert(initial_input.to_string(), ch);
                                let action = launch(input);
                                KeyInputStatus::new("", None, Some(action))
                            } else {
                                KeyInputStatus::no_change(initial_input)
                            }
                        }
                        KeyInputMode::Entry => {
                            let accept = self.accepter.clone();
                            if let Some(ch) = key.to_unicode()
                                && accept(initial_input.to_string(), ch)
                            {
                                let convert = self.converter.clone();
                                let input = convert(initial_input.to_string(), ch);
                                KeyInputStatus::new(&input, None, None)
                            } else {
                                KeyInputStatus::no_change(initial_input)
                            }
                        }
                    },
                }
            }
        }
    }
}
