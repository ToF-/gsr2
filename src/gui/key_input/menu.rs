use crate::gui::action::Action;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::key_input_mode::KeyInputMode;
use crate::gui::key_input::key_input_rules::KeyInputRules;
use crate::model::change::Change;
use crate::model::find::Find;
use crate::model::order::Order;
use crate::model::view_option::ViewOption;

pub fn view_menu() -> KeyInput {
    KeyInput::new(
        "View: 1x1 2x2 3x3 4x4 5x5 thumbs full covers palette Date Filepath Size Categories",
        None,
        KeyInputMode::Menu,
        |_, ch| {
            matches!(
                ch,
                '1' | '2' | '3' | '4' | '5' | 't' | 'c' | 'p' | 'D' | 'F' | 'S' | 'f' | 'C'
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
                'p' => ViewOption::Palette,
                'D' => ViewOption::FileDate,
                'F' => ViewOption::FilePath,
                'S' => ViewOption::FileSize,
                'f' => ViewOption::FullSize,
                'C' => ViewOption::Catalog,
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

pub fn change_menu() -> KeyInput {
    KeyInput::new(
        "Enter what to change: c(A)talog (C)ategory (L)abel (N)ame (R)emove (T)ag (U)nlabel co(V)er on",
        None,
        KeyInputMode::Menu,
        |_, ch| matches!(ch, 'a' | 'c' | 'l' | 'n' | 'r' | 't' | 'u' | 'v'),
        |_, ch| {
            let change = match ch {
                't' => Change::AddTag,
                'a' => Change::Catalog,
                'c' => Change::Category,
                'v' => Change::Cover,
                'l' => Change::Label,
                'n' => Change::Name,
                'r' => Change::RemoveTag,
                'u' => Change::Unlabel,
                _ => todo!(),
            };
            let s: String = (change as i32).to_string();
            s
        },
        |s| {
            let n: i32 = s.parse::<i32>().unwrap();
            let change = Change::from(n);
            match change {
                Change::AddTag => Action::EnterAddTag,
                Change::Catalog => Action::PickCatalogChange,
                Change::Category => Action::SelectCategoryForPicture,
                Change::Cover => Action::ToggleCover,
                Change::Label => Action::EnterLabel,
                Change::Name => Action::EnterRename,
                Change::RemoveTag => Action::EnterRemoveTag,
                Change::Unlabel => Action::Unlabel,
                _ => Action::Nothing,
            }
        },
    )
}
pub fn catalog_menu() -> KeyInput {
    KeyInput::new(
        "Enter what to change: (A)dd category (M)ove category (R)emove category",
        None,
        KeyInputMode::Menu,
        |_, ch| matches!(ch, 'a' | 'm' | 'r'),
        |_, ch| {
            let change = match ch {
                'a' => Change::AddCategory,
                'm' => Change::MoveCategory,
                'r' => Change::RemoveCategory,
                _ => todo!(),
            };
            let s: String = (change as i32).to_string();
            s
        },
        |s| {
            let n: i32 = s.parse::<i32>().unwrap();
            let change = Change::from(n);
            dbg!(&change);
            match change {
                Change::AddCategory => Action::EnterNewCategory,
                Change::MoveCategory => Action::SelectCategoryToMove,
                Change::RemoveCategory => Action::SelectCategoryToRemove,
                _ => Action::Nothing,
            }
        },
    )
}

pub fn find_menu() -> KeyInput {
    KeyInput::new(
        "Find pictures on  C)ategory (B)elongs (L)abel (N)ame (F)ile Path (S)ome Tags (A)ll tags ",
        None,
        KeyInputMode::Menu,
        |_, ch| matches!(ch, 'a' | 'b' | 'c' | 'f' | 'l' | 'n' | 's'),
        |_, ch| {
            let find = match ch {
                'a' => Find::AllTags,
                'b' => Find::SubCategory,
                'c' => Find::Category,
                'l' => Find::Label,
                'n' => Find::Name,
                'f' => Find::FilePath,
                's' => Find::SomeTags,
                _ => todo!(),
            };
            let s: String = (find as i32).to_string();
            s
        },
        |s| {
            let n: i32 = s.parse::<i32>().unwrap();
            let find = Find::from(n);
            Action::EnterFind(find)
        },
    )
}

pub fn select_menu() -> KeyInput {
    KeyInput::new(
        "Select pictures on  C)ategory (B)elongs (L)abel (N)ame (F)ile Path (S)ome Tags (A)ll tags ",
        None,
        KeyInputMode::Menu,
        |_, ch| matches!(ch, 'a' | 'b' | 'c' | 'f' | 'l' | 'n' | 's'),
        |_, ch| {
            let find = match ch {
                'a' => Find::AllTags,
                'b' => Find::SubCategory,
                'c' => Find::Category,
                'l' => Find::Label,
                'n' => Find::Name,
                'f' => Find::FilePath,
                's' => Find::SomeTags,
                _ => todo!(),
            };
            let s: String = (find as i32).to_string();
            s
        },
        |s| {
            let n: i32 = s.parse::<i32>().unwrap();
            let find = Find::from(n);
            Action::EnterSelect(find)
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
