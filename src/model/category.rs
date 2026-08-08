pub(crate) use crate::model::sub_category::TOP_CATEGORY;

pub type Category = Option<String>;

pub fn category_from_string(s: &str) -> Category {
    if s.is_empty() || s == TOP_CATEGORY {
        None
    } else {
        Some(s.to_string())
    }
}

pub fn string_from_category(category: Category) -> String {
    match category {
        Some(s) => s.clone(),
        None => "".to_string(),
    }
}
