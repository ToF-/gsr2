use crate::gui::action::Action;
use crate::gui::editor::Editor;
use crate::gui::editor::editor_mode::EditorMode;
use crate::gui::editor::editor_rules::EditorRules;
use crate::gui::entry_kind::EntryKind;
use crate::model::label::Label;
use crate::model::tags::Tags;
use crate::model::view_option::ViewOption;
use std::str::FromStr;

pub fn view_menu() -> Editor {
    Editor::new(
        "View: 1x1 2x2 3x3 4x4 5x5 Thumbs Covers Date Path Size",
        None,
        EditorMode::Menu,
        |_, ch| {
            matches!(
                ch,
                '1' | '2' | '3' | '4' | '5' | 't' | 'c' | 'd' | 'p' | 's'
            )
        },
        |_, ch| {
            let view_option = match ch {
                '1' => ViewOption::Single,
                '2' => ViewOption::Grid2x2,
                '3' => ViewOption::Grid3x3,
                '4' => ViewOption::Grid4x4,
                '5' => ViewOption::Grid5x5,
                't' => ViewOption::Thumbnails,
                'c' => ViewOption::Covers,
                'd' => ViewOption::FileDate,
                'p' => ViewOption::FilePath,
                's' => ViewOption::FileSize,
                _ => todo!(),
            };
            dbg!(view_option.to_string());
            view_option.to_string()
        },
        |s| {
            dbg!(&s);
            if let Ok(view_option) = ViewOption::from_str(&s) {
                Action::ApplyViewSetting(view_option)
            } else {
                Action::Dismiss
            }
        },
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tags::tags_from_str;
    use gtk::gdk::Key;

    #[test]
    fn given_a_simple_key_lauches_the_corresponding_action() {
        let editor = view_menu();
        let status = editor.edit("", Key::from_name("t").unwrap());
        assert_eq!(
            Some(Action::ApplyViewSetting(ViewOption::Thumbnails)),
            status.result_action()
        );
        let editor = view_menu();
        let status = editor.edit("", Key::from_name("c").unwrap());
        assert_eq!(
            Some(Action::ApplyViewSetting(ViewOption::Covers)),
            status.result_action()
        );
    }
}
