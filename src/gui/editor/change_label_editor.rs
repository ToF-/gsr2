
use crate::gui::editor::Action;
use crate::gui::editor::Editor;
use crate::gui::editor::EditorRules;
use crate::gui::entry_kind::EntryKind;
use crate::model::label::Label;
use crate::model::tags::Tags;

pub fn change_label_editor(completion_tags: Tags) -> Editor {
    Editor::new(
        EntryKind::Label,
        "Enter a label",
        Some(completion_tags),
        Action::Label(Label::from("foo")),
        |_, ch| matches!(ch, 'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' '),
        |s, ch| {
            let mut input = s;
            if ch.is_ascii() {
                input.push(ch.to_lowercase().next().unwrap())
            } else {
                input.push(ch)
            }
            input
        },
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tags::tags_from_str;
    use gtk::gdk::Key;

    #[test]
    fn given_a_simple_key_append_that_to_the_input() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartelby,law"));
        let status = editor.edit("fi", Key::from_name("b").unwrap());
        assert_eq!("fib", &status.input());
    }
    #[test]
    fn given_an_illegal_key_input_does_not_change() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartelby,law"));
        let status = editor.edit("fi", Key::from_name("numbersign").unwrap());
        assert_eq!("fi", &status.input());
    }
    #[test]
    fn given_an_uppercase_key_input_is_converted_to_lowercase() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartelby,law"));
        let status = editor.edit("fi", Key::from_name("B").unwrap());
        assert_eq!("fib", &status.input());
    }
}
