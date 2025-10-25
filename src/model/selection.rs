use crate::model::tags::{Tags, empty, tags_from_str};

#[derive(Debug, Clone)]
pub struct Selection {
    tags: Tags,
    restrict: bool,
}

impl Selection {
    pub fn from(s: &str, restrict: bool) -> Self {
        Selection {
            tags: tags_from_str(s),
            restrict,
        }
    }

    pub fn from_opt(s_opt: &Option<String>, restrict: bool) -> Self {
        if let Some(s) = s_opt {
            Self::from(s, restrict)
        } else {
            Self::empty()
        }
    }

    pub fn empty() -> Self {
        Selection {
            tags: empty(),
            restrict: false,
        }
    }

    pub fn tags(&self) -> Tags {
        self.tags.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn intersect_with(&self, tags: Tags) -> bool {
        !self.tags.is_disjoint(&tags)
    }

    pub fn includes(&self, tags: Tags) -> bool {
        self.tags.is_subset(&tags)
    }

    pub fn matches(&self, tags: Tags) -> bool {
        if self.restrict {
            self.includes(tags)
        } else {
            self.intersect_with(tags)
        }
    }
}
