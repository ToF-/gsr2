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


