use std::sync::Arc;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::editor::editor_status::EditorStatus;
use crate::gui::action::Action;
use crate::env::default_values::MAX_LABEL_LENGTH;
use crate::env::default_values::MAX_LABELS_LENGTH;
use crate::env::default_values::MAX_NAME_LENGTH;
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;
use crate::gui::mode::Mode;
use crate::gui::view::main_view::MainView;
use crate::model::order::Order;
use crate::model::tags::{Tags, empty_tags};
use gdk::Key;
use gtk::{self, gdk};
use itertools::Itertools;

pub mod editor_rules;
pub mod legacy_editor;
pub mod change_label_editor;
pub mod entry_editor;
pub mod entry_editor_status;
pub mod editor_status;
pub mod validator;

trait EditorRules {
    fn new<A: Fn(String, char) -> bool +'static,
           C: Fn(String, char) -> String +'static,>
               (entry_kind: EntryKind,
                completion_tags_opt: Option<Tags>,
                action_result: Action,
                accepter: A,
                converter: C) -> Self;

    fn edit(&self, input: &str, key: gtk::gdk::Key) -> EditorStatus ;
}

pub struct Editor {
    completion_dispenser_opt: Option<CompletionDispenser>,
    accepter: Arc<dyn Fn(String, char) -> bool>,
    converter: Arc<dyn Fn(String, char) -> String>,
}

impl EditorRules for Editor {
    fn new<A: Fn(String, char) -> bool +'static,
           C: Fn(String, char) -> String +'static,>
               (entry_kind: EntryKind,
                completion_tags_opt: Option<Tags>,
                action_result: Action,
                accepter: A,
                converter: C) -> Self {
                   Self {
                   completion_dispenser_opt: match completion_tags_opt {
                       None => None,
                       Some(tags) => Some(CompletionDispenser::new_with(tags)),
                   },
                   accepter: Arc::new(accepter),
                   converter: Arc::new(converter),
                   }
           }

    fn edit(&self, input: &str, key: gtk::gdk::Key) -> EditorStatus {
        EditorStatus::no_change(input)
    }

}
