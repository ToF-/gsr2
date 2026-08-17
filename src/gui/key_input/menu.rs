use crate::gui::action::Action;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::key_input_mode::KeyInputMode;
use crate::gui::key_input::key_input_rules::KeyInputRules;
use crate::model::change::Change::Category;
use crate::model::change::Change::Cover;
use crate::model::order::Order;
use crate::model::order::Order::ColorCount;
use crate::model::order::Order::Date;
use crate::model::view_option::ViewOption;
use std::str::FromStr;

pub fn view_menu() -> KeyInput {
    KeyInput::new(
        "View: 1x1 2x2 3x3 4x4 5x5 Thumbs Covers Date Path Size Full",
        None,
        KeyInputMode::Menu,
        |_, ch| {
            matches!(
                ch,
                '1' | '2' | '3' | '4' | '5' | 't' | 'c' | 'd' | 'p' | 's' | 'f'
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
                'f' => ViewOption::FullSize,
                _ => todo!(),
            };
            let s: String = (view_option as i32).to_string();
            s
        },
        |s| {
            let n: i32 = s.parse::<i32>().unwrap();
            let view_option = ViewOption::from(n);
            Action::ApplyViewSetting(view_option)
        },
    )
}

pub fn order_menu() -> KeyInput {
    KeyInput::new(
        "Enter a sorting criteria: c(A)tegory (C)olors (D)ate (L)abel (M)ost views (N)ame (P)alette c(O)ver (R)andom (S)ize (V)alue ",
        None,
        KeyInputMode::Menu,
        |_, ch| {
            matches!(
                ch,
                'a' | 'c' | 'd' | 'l' | 'm' | 'n' | 'p' | 'o' | 'r' | 's' | 'v'
            )
        },
        |_, ch| {
            let order_setting = match ch {
                'a' => Order::Category,
                'c' => Order::ColorCount,
                'o' => Order::Cover,
                'd' => Order::Date,
                'l' => Order::Label,
                'n' => Order::Name,
                'p' => Order::Palette,
                'r' => Order::Random,
                'm' => Order::Score,
                's' => Order::Size,
                'v' => Order::Value,
                _ => todo!(),
            };
            let s: String = (order_setting as i32).to_string();
            s
        },
        |s| {
            let n: i32 = s.parse::<i32>().unwrap();
            let order_setting = Order::from(n);
            Action::ApplyOrderSetting(order_setting)
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::model::tags::tags_from_str;
    use gtk::gdk::Key;
    use sper::*;

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
