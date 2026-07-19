use crate::model::tags::empty_tags;
use crate::model::tags::Tags;
use crate::model::tags::tags_from_str;

#[derive(Debug, Clone)]
pub struct Categories {
    names: Tags,
}

impl Categories {
    pub fn empty() -> Self {
        Self {
            names: empty_tags(),
        }
}

pub fn from_string(s: &str) -> Self {
    Self {
        names: tags_from_str(s)
    }
}
}
