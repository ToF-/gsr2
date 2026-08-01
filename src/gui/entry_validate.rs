use crate::gui::valid_entry_char::valid_entry_char;
use crate::gui::entry_kind::EntryKind;
use crate::model::order::Order;

pub fn entry_validate(entry_kind:EntryKind, entry: &str, ch: char) -> Option<String> {
    let mut input: String = entry.into();
    Some(input)
}

pub fn append_char(entry_kind:EntryKind, entry: &str, ch: char) -> Option<String> {
    let mut input: String = entry.into();
    if valid_entry_char(entry_kind, ch) {
        convert_char(entry_kind, entry, ch)
    } else {
       None 
    }
}

pub fn convert_char(entry_kind: EntryKind, entry: &str, ch: char) -> Option<String> {
    let mut input: String = entry.into();
    match ch {
        ' ' if entry_kind == EntryKind::FindAllTags => input.push(','),
        ' ' if entry_kind == EntryKind::FindSomeTags => input.push(','),
        ' ' if entry_kind == EntryKind::FindSubCategory => input.push(','),
        ' ' if entry_kind == EntryKind::SelectAllTags => input.push(','),
        ' ' if entry_kind == EntryKind::SelectSomeTags => input.push(','),
        ' ' if entry_kind == EntryKind::SelectSubCategory => input.push(','),
        ' ' => input.push('-'),
        c if entry_kind == EntryKind::Order => {
            let order: Order = match c {
                'a' => Order::Category,
                'c' => Order::ColorCount,
                'd' => Order::Date,
                'l' => Order::Label,
                'n' => Order::Name,
                'o' => Order::Cover,
                'm' => Order::Score,
                'p' => Order::Palette,
                'r' => Order::Random,
                's' => Order::Size,
                'v' => Order::Value,
                _ => todo!(),
            };
            input = format!("{}", order);
        }
        c if entry_kind == EntryKind::Find => {
            let criterion = match c {
                'a' => "AllTags",
                'b' => "SubCategory",
                'c' => "Category",
                'l' => "Label",
                'n' => "Name",
                's' => "SomeTags",
                _ => todo!(),
            };
            input = criterion.to_string();
        }
        c if entry_kind == EntryKind::Select => {
            let criterion = match c {
                'a' => "AllTags",
                'b' => "SubCategory",
                'c' => "Category",
                'l' => "Label",
                'n' => "Name",
                's' => "SomeTags",
                _ => todo!(),
            };
            input = criterion.to_string();
        }
        c if entry_kind == EntryKind::Change => {
            let change = match c {
                'a' => "Catalog",
                'c' => "Category",
                'l' => "Label",
                'n' => "Name",
                'r' => "RemoveTag",
                't' => "AddTag",
                'u' => "Unlabel",
                'v' => "Cover",
                _ => todo!(),
            };
            input = change.to_string();
        }
        c if entry_kind == EntryKind::View => {
            let view = match c {
                '1' => "1",
                '2' => "2",
                '3' => "3",
                '4' => "4",
                '5' => "5",
                't' => "Thumbs",
                'c' => "Covers",
                'p' => "Path",
                'd' => "Date",
                's' => "Size",

                _ => todo!(),
            };
            input = view.to_string();
        }
        c if entry_kind == EntryKind::Catalog => {
            let change = match c {
                'a' => "AddCategory",
                'm' => "MoveCategory",
                'r' => "RemoveCategory",
                _ => todo!(),
            };
            input = change.to_string();
        }
        other if other.is_ascii() => input.push(other.to_lowercase().next().unwrap()),
        other => input.push(other),
    }
    Some(input)
}
