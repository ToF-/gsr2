use crate::gui::entry_kind::EntryKind;
use crate::model::order::Order;

#[derive(Debug)]
pub struct Validator {
    entry_kind: EntryKind,
}

impl Validator {
    pub fn new(entry_kind: EntryKind) -> Self {
        Self { entry_kind }
    }

    pub fn validate_entry(&self, entry: &str, ch: char) -> Option<String> {
        if let Some(input) = self.append_char(entry, ch) {
            Some(input)
        } else {
            None
        }
    }

    pub fn append_char(&self, entry: &str, ch: char) -> Option<String> {
        let mut input: String = entry.into();
        if self.valid_entry_char(ch) {
            self.convert_char(entry, ch)
        } else {
            None
        }
    }

    pub fn convert_char(&self, entry: &str, ch: char) -> Option<String> {
        let mut input: String = entry.into();
        let entry_kind = self.entry_kind.clone();
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
            other if other.is_ascii() => {
                input.push(other.to_lowercase().next().unwrap());
            }
            other => input.push(other),
        };
        Some(input)
    }
    pub fn valid_entry_char(&self, ch: char) -> bool {
        match self.entry_kind {
            EntryKind::Catalog => {
                matches!(ch, 'a' | 'm' | 'r')
            }
            EntryKind::Change => {
                matches!(ch, 'a' | 'c' | 'l' | 'n' | 'r' | 't' | 'u' | 'v')
            }
            EntryKind::Number => ch.is_ascii_digit(),
            EntryKind::DeleteConfirmation
            | EntryKind::MoveConfirmation
            | EntryKind::MoveToLabelConfirmation(_) => {
                matches!(ch, 'e' | 'n' | 'o' | 's' | 'y')
            }
            EntryKind::Find | EntryKind::Select => {
                matches!(ch, 'a' | 'b' | 'c' | 'l' | 'n' | 's')
            }
            EntryKind::FindName
            | EntryKind::FindLabel
            | EntryKind::FindCategory
            | EntryKind::FindSubCategory
            | EntryKind::SelectName
            | EntryKind::SelectLabel
            | EntryKind::SelectCategory
            | EntryKind::SelectSubCategory => {
                matches!(ch,
                        'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | '^' | '$' | '.' | '*' | '/' | '{' | '}' | '[' | ']' | '(' | ')' | '\\' )
            }
            EntryKind::AddTag => {
                matches!(ch,
                    'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | ',')
            }
            EntryKind::Label
            | EntryKind::Rename
            | EntryKind::RemoveTag
            | EntryKind::RemoveCategory => {
                matches!(ch,
                        'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ')
            }
            EntryKind::AddCategory | EntryKind::MoveCategory => {
                matches!(ch,
                    'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | ',')
            }
            EntryKind::Categorize => {
                matches!(ch,
                    'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ':')
            }
            EntryKind::FindAllTags
            | EntryKind::FindSomeTags
            | EntryKind::SelectAllTags
            | EntryKind::SelectSomeTags => matches!(ch,
                    'a'..='z' |'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | ',' ),
            EntryKind::Order => matches!(
                ch,
                'a' | 'c' | 'd' | 'p' | 'm' | 'l' | 'n' | 'o' | 'r' | 's' | 'v'
            ),
            EntryKind::View => matches!(
                ch,
                '1' | '2' | '3' | '4' | '5' | 't' | 'c' | 'd' | 'p' | 's'
            ),
            EntryKind::Rank => matches!(ch, '0' | '1' | '2' | '3'),
            EntryKind::Information | EntryKind::Help => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_of_a_validation_for_entering_a_label() {
        let validator = Validator::new(EntryKind::Label);
        assert_eq!(Some("foo"), validator.validate_entry("fo", 'o').as_deref());
    }
    #[test]
    fn example_of_a_validation_for_choosing_a_view_option() {
        let validator = Validator::new(EntryKind::View);
        assert_eq!(Some("Thumbs"), validator.validate_entry("", 't').as_deref());
    }
    #[test]
    fn example_of_a_validation_blocking_a_forbidden_char() {
        let validator = Validator::new(EntryKind::View);
        assert_eq!(None, validator.validate_entry("", 'z').as_deref());
    }
    #[test]
    fn example_of_a_validation_blocking_completion_with_a_forbidden_char() {
        let validator = Validator::new(EntryKind::AddTag);
        assert_eq!(None, validator.validate_entry("my_tag", '$').as_deref());
    }
}
