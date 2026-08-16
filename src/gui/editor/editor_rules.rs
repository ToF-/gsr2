use crate::gui::action::Action;
use crate::gui::editor::EditorStatus;

use crate::gui::entry_kind::EntryKind;
use crate::model::tags::Tags;

pub trait EditorRules {
    fn new<
        A: Fn(String, char) -> bool + 'static,
        C: Fn(String, char) -> String + 'static,
        L: Fn(String) -> Action + 'static,
    >(
        prompt: &str,
        completion_tags_opt: Option<Tags>,
        editable: bool,
        accepter: A,
        converter: C,
        launcher: L,
    ) -> Self;

    fn edit(&self, input: &str, key: gtk::gdk::Key) -> EditorStatus;
}
