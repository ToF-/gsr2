use std::sync::Arc;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::editor::EditorStatus;
use crate::gui::editor::Action;
use crate::model::tags::Tags;
use crate::gui::entry_kind::EntryKind;
use crate::gui::editor::EditorT;

pub struct ChangeLabelEditor {
    completion_dispenser_opt: Option<CompletionDispenser>,
    accepter: Arc<dyn Fn(String, char) -> bool>,
    converter: Arc<dyn Fn(String, char) -> String>,
}

impl EditorT for ChangeLabelEditor {
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

