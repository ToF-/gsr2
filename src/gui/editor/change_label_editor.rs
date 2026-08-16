use crate::env::default_values::SPACE_REPLACEMENT_CHAR_FOR_TAGS;
use crate::gui::editor::Action;
use crate::gui::editor::Editor;
use crate::gui::editor::EditorRules;
use crate::gui::entry_kind::EntryKind;
use crate::model::label::Label;
use crate::model::tags::Tags;

pub fn change_label_editor(completion_tags: Tags) -> Editor {
    Editor::new(
        "Enter a label",
        Some(completion_tags),
        true,
        |_, ch| matches!(ch, 'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' '),
        |s, ch| {
            let mut input = s;
            if ch.is_ascii_uppercase() {
                input.push(ch.to_lowercase().next().unwrap())
            } else if ch.is_ascii_whitespace() {
                input.push(SPACE_REPLACEMENT_CHAR_FOR_TAGS)
            } else {
                input.push(ch)
            }
            input
        },
        |s| Action::Label(s),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tags::tags_from_str;
    use gtk::gdk::Key;

    #[test]
    fn given_a_simple_key_append_that_to_the_input() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("fi", Key::from_name("b").unwrap());
        assert_eq!("fib", &status.input());
    }
    #[test]
    fn given_an_illegal_key_input_does_not_change() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("fi", Key::from_name("numbersign").unwrap());
        assert_eq!("fi", &status.input());
    }
    #[test]
    fn given_an_uppercase_key_input_is_converted_to_lowercase() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("fi", Key::from_name("B").unwrap());
        assert_eq!("fib", &status.input());
    }
    #[test]
    fn given_an_initial_input_that_is_too_short_complete_produce_no_candidates() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("f", Key::from_name("Tab").unwrap());
        assert_eq!(None, status.candidate_list_tip());
    }
    #[test]
    fn given_an_initial_input_that_completes_with_one_candidates_input_is_set_to_that() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("fo", Key::from_name("Tab").unwrap());
        assert_eq!(None, status.candidate_list_tip());
        assert_eq!("foo", &status.input());
    }
    #[test]
    fn given_an_initial_input_that_completes_with_two_candidates_these_candidates_are_tipped() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("ba", Key::from_name("Tab").unwrap());
        assert_eq!(
            Some("[ bar bartleby ]".to_string()),
            status.candidate_list_tip()
        );
        assert_eq!("ba", &status.input());
    }
    #[test]
    fn given_a_space_then_it_is_converted_to_dash() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("fib", Key::from_name("space").unwrap());
        assert_eq!("fib-", &status.input());
    }
    #[test]
    fn given_en_escape_then_it_returns_a_cancel_action() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("fib", Key::from_name("Escape").unwrap());
        assert_eq!(Some(Action::Cancel), status.result_action());
    }
    #[test]
    fn given_a_return_then_it_returns_the_input_and_its_specific_label_action() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartleby,law"));
        let status = editor.edit("fib", Key::from_name("Return").unwrap());
        assert_eq!("fib", &status.input());
        assert_eq!(
            Some(Action::Label("fib".to_string())),
            status.result_action()
        );
    }
}
