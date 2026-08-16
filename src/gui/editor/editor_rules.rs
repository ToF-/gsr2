use crate::gui::editor::EditorStatus;
use crate::gui::action::Action;

use crate::model::tags::Tags;
use crate::gui::entry_kind::EntryKind;

pub trait EditorRules {
    fn new<A: Fn(String, char) -> bool +'static,
           C: Fn(String, char) -> String +'static,>
               (entry_kind: EntryKind,
                prompt: &str,
                completion_tags_opt: Option<Tags>,
                action_result: Action,
                accepter: A,
                converter: C) -> Self;

    fn edit(&self, input: &str, key: gtk::gdk::Key) -> EditorStatus ;
}

