use std::collections::HashSet;
use crate::model::label::Label;

pub type Tags = HashSet<Label>;

pub fn tags_from_string(s: &str) -> Tags {
    let set: HashSet<String> = 
        s.split(',')
        .map(|s| s.to_string())
        .collect();
    set
}

pub fn empty() -> Tags {
    HashSet::new()
}
