use crate::model::label::Label;
use std::collections::HashSet;

pub type Tags = HashSet<Label>;

pub fn tags_from_str(s: &str) -> Tags {
    let set: HashSet<String> = s.split(',').map(|s| s.to_string()).collect();
    set
}

pub fn tags_from_vec(v: Vec<String>) -> Tags {
    let set: HashSet<String> = HashSet::<String>::from_iter(v);
    set
}

pub fn tags_as_vec(tags: Tags) -> Vec<String> {
    tags.iter().cloned().collect()
}

pub fn empty_tags() -> Tags {
    HashSet::new()
}
