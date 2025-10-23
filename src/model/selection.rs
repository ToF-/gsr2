use crate::model::label;
use crate::model::tags::{Tags, empty, tags_from_string};

#[derive(Debug,Clone)]
pub struct Selection {
    tags: Tags,
}

impl Selection {
    pub fn from(s: &str) -> Self {
        Selection {
            tags: tags_from_string(s)
        }
    }

    pub fn from_opt(s_opt: &Option<String>) -> Self {
        if let Some(s) = s_opt {
            Self::from(s)
        } else {
            Self::empty()
        }
    }

    pub fn empty() -> Self {
        Selection {
            tags: empty()
        }
    }

    pub fn tags(&self) -> Tags {
        self.tags.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

}
