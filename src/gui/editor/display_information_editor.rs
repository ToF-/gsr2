use crate::env::default_values::INFORMATION_SYMBOL;
use crate::env::default_values::SPACE_REPLACEMENT_CHAR_FOR_TAGS;
use crate::gui::editor::Action;
use crate::gui::editor::Editor;
use crate::gui::editor::EditorMode;
use crate::gui::editor::EditorRules;
use crate::gui::entry_kind::EntryKind;
use crate::model::label::Label;
use crate::model::tags::Tags;

pub fn display_information_editor() -> Editor {
    Editor::new(
        INFORMATION_SYMBOL,
        None,
        EditorMode::Information,
        |s, c| true,
        |s, _| s,
        |_| Action::Dismiss,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tags::tags_from_str;
    use gtk::gdk::Key;

    #[test]
    fn given_any_key_does_not_change_the_input() {
        let editor = display_information_editor();
        let status = editor.edit("foo", Key::from_name("b").unwrap());
        assert_eq!("foo", &status.input());
    }
    #[test]
    fn given_any_key_lauche_the_dismiss_action() {
        let editor = display_information_editor();
        let status = editor.edit("fib", Key::from_name("a").unwrap());
        assert_eq!(Some(Action::Dismiss), status.result_action());
    }
}
