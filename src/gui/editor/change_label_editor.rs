 use crate::gui::editor::EditorRules;
 use crate::gui::editor::Editor;
 use crate::model::label::Label;
use std::sync::Arc;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::editor::EditorStatus;
use crate::gui::editor::Action;
use crate::model::tags::Tags;
use crate::gui::entry_kind::EntryKind;


pub fn change_label_editor(completion_tags: Tags) -> Editor {
    Editor::new(EntryKind::Label,
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
        })

}
#[cfg(test)]
mod tests {
    use crate::model::tags::tags_from_str;
    use super::*;

    #[test]
    fn given_a_simple_key_append_that_to_the_input() {
        let editor = change_label_editor(tags_from_str("foo,bar,bartelby,law"));
    }
    
}

