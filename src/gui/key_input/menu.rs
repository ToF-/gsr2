use crate::gui::action::Action;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::key_input_mode::KeyInputMode;
use crate::gui::key_input::key_input_rules::KeyInputRules;
use crate::model::view_option::ViewOption;
use std::str::FromStr;

pub fn view_menu() -> KeyInput {
    KeyInput::new(
        "View: 1x1 2x2 3x3 4x4 5x5 Thumbs Covers Date Path Size",
        None,
        KeyInputMode::Menu,
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
            if let Ok(view_option) = ViewOption::from_str(&s) {
                dbg!(&s, view_option);

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
    fn given_a_simple_key_launches_the_corresponding_action() {
        let key_input = view_menu();
        let status = key_input.edit("", Key::from_name("t").unwrap());
        assert_eq!(
            Some(Action::ApplyViewSetting(ViewOption::Thumbnails)),
            status.result_action()
        );
        let key_input = view_menu();
        let status = key_input.edit("", Key::from_name("c").unwrap());
        assert_eq!(
            Some(Action::ApplyViewSetting(ViewOption::Covers)),
            status.result_action()
        );
    }
}
