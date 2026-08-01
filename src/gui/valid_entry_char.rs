use crate::gui::entry_kind::EntryKind;
pub fn valid_entry_char(entry_kind: EntryKind, ch: char) -> bool {
    match entry_kind {
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
