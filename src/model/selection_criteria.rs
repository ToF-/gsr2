use crate::cli::args::Args;
use crate::model::tags::{Tags, empty_tags, tags_from_str};

pub const SOME_TAGS: bool = false;
pub const ALL_TAGS: bool = true;

#[derive(Debug, Clone)]
pub struct SelectionCriteria {
    tags: Tags,
    restrict: bool,
}

impl SelectionCriteria {
    pub fn from(s: &str, restrict: bool) -> Self {
        SelectionCriteria {
            tags: tags_from_str(s),
            restrict,
        }
    }

    pub fn from_args(args: &Args) -> Self {
        if let Some(labels) = &args.select {
            SelectionCriteria::from(labels, SOME_TAGS)
        } else if let Some(labels) = &args.restrict {
            SelectionCriteria::from(labels, ALL_TAGS)
        } else {
            SelectionCriteria::empty()
        }
    }

    pub fn empty() -> Self {
        SelectionCriteria {
            tags: empty_tags(),
            restrict: ALL_TAGS,
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
