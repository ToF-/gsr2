use crate::gui::action::Action;
use crate::gui::key_input::KeyInputMode;
use crate::gui::key_input::KeyInputStatus;

use crate::gui::entry_kind::EntryKind;
use crate::model::tags::Tags;

pub trait KeyInputRules {
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
    ) -> Self;

    fn edit(&self, input: &str, key: gtk::gdk::Key) -> KeyInputStatus;
}
