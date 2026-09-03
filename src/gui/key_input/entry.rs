use crate::env::default_values::SPACE_REPLACEMENT_CHAR_FOR_TAGS;
use crate::gui::key_input::Action;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::KeyInputMode;
use crate::gui::key_input::KeyInputRules;
use crate::model::find::Find;
use crate::model::tags::Tags;

pub fn label_change_entry(completion_tags: Tags) -> KeyInput {
    KeyInput::new(
        "Enter a label",
        Some(completion_tags),
        KeyInputMode::Entry,
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

pub fn add_tags_entry(completion_tags: Tags) -> KeyInput {
    KeyInput::new(
        "Enter new tags to add",
        Some(completion_tags),
        KeyInputMode::Entry,
        |_, ch| matches!(ch, 'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | ','),
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
        |s| Action::AddTag(s),
    )
}

pub fn add_new_category() -> KeyInput {
    KeyInput::new(
        "Enter a new category",
        None,
        KeyInputMode::Entry,
        |_, ch| matches!(ch, 'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' ),
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
        |s| Action::SelectCategoryAddTarget(s),
    )
}

pub fn remove_tags_entry(completion_tags: Tags) -> KeyInput {
    KeyInput::new(
        "Enter new tags to remove",
        Some(completion_tags),
        KeyInputMode::Entry,
        |_, ch| matches!(ch, 'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | ','),
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
        |s| Action::RemoveTag(s),
    )
}

pub fn rename_entry() -> KeyInput {
    KeyInput::new(
        "Enter a name",
        None,
        KeyInputMode::Entry,
        |_, ch| matches!(ch, 'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' ),
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
        |s| Action::Rename(s),
    )
}

pub fn find_criteria_entry(find_criteria: Find, completion_tags: Tags) -> KeyInput {
    let find = find_criteria.clone();
    KeyInput::new(
        &format!("Enter criteria for finding on {}", find.clone().to_string()),
        match find.clone() {
            Find::AllTags | Find::SomeTags => Some(completion_tags),
            _ => None,
        },
        KeyInputMode::Entry,
        match find.clone() {
            Find::AllTags | Find::SomeTags => {
                |_, ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | ',')
            }
            _ => |_, _| true,
        },
        match find.clone() {
            Find::AllTags | Find::SomeTags => |s: String, ch: char| {
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
            _ => |s: String, ch: char| {
                let mut input = s;
                input.push(ch);
                input
            },
        },
        move |s: String| Action::Find(find.clone(), s),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tags::tags_from_str;
    use gtk::gdk::Key;

    #[test]
    fn given_a_simple_key_append_that_to_the_input() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("fi", Key::from_name("b").unwrap());
        assert_eq!("fib", &status.input());
    }
    #[test]
    fn given_an_illegal_key_input_does_not_change() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("fi", Key::from_name("numbersign").unwrap());
        assert_eq!("fi", &status.input());
    }
    #[test]
    fn given_an_uppercase_key_input_is_converted_to_lowercase() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("fi", Key::from_name("B").unwrap());
        assert_eq!("fib", &status.input());
    }
    #[test]
    fn given_an_initial_input_that_is_too_short_complete_produce_no_candidates() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("f", Key::from_name("Tab").unwrap());
        assert_eq!(None, status.candidate_list_tip());
    }
    #[test]
    fn given_an_initial_input_that_completes_with_one_candidates_input_is_set_to_that() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("fo", Key::from_name("Tab").unwrap());
        assert_eq!(None, status.candidate_list_tip());
        assert_eq!("foo", &status.input());
    }
    #[test]
    fn given_an_initial_input_that_completes_with_two_candidates_these_candidates_are_tipped() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("ba", Key::from_name("Tab").unwrap());
        assert_eq!(
            Some("[ bar bartleby ]".to_string()),
            status.candidate_list_tip()
        );
        assert_eq!("ba", &status.input());
    }
    #[test]
    fn given_a_space_then_it_is_converted_to_dash() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("fib", Key::from_name("space").unwrap());
        assert_eq!("fib-", &status.input());
    }
    #[test]
    fn given_en_escape_then_it_returns_a_cancel_action() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("fib", Key::from_name("Escape").unwrap());
        assert_eq!(Some(Action::Cancel), status.result_action());
    }
    #[test]
    fn given_a_return_then_it_returns_the_input_and_its_specific_label_action() {
        let key_input = label_change_entry(tags_from_str("foo,bar,bartleby,law"));
        let status = key_input.edit("fib", Key::from_name("Return").unwrap());
        assert_eq!("fib", &status.input());
        assert_eq!(
            Some(Action::Label("fib".to_string())),
            status.result_action()
        );
    }
}
