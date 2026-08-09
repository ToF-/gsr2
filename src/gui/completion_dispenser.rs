use crate::model::tags::Tags;
use crate::model::tags::empty_tags;

#[derive(Clone,Debug)]
pub struct CompletionDispenser {
    tags: Tags,
}

impl Default for CompletionDispenser {
    fn default() -> Self {
        Self { tags: empty_tags() }
    }
}
impl CompletionDispenser {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_with(tags: Tags) -> Self {
        Self { tags }
    }

    pub fn candidates(&self, entry: &str) -> Vec<String> {
        let mut entry_tags = entry.split(',');
        if let Some(last_entry_tag) = entry_tags.next_back()
            && last_entry_tag.len() >= 2
        {
            let mut result: Vec<String> = vec![];
            self.tags.iter().for_each(|tag| {
                if tag.starts_with(&last_entry_tag) {
                    result.push(tag.into())
                }
            });
            result.sort();
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tags::tags_from_str;

    #[test]
    fn no_candidates_when_no_prefix() {
        let dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo"));
        assert!(dispenser.candidates("").is_empty());
    }

    #[test]
    fn no_candidates_when_only_one_char_entry() {
        let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo"));
        assert!(dispenser.candidates("f").is_empty());
    }

    #[test]
    fn candidates_when_two_or_more_chars_entry() {
        let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo"));
        assert_eq!(vec!["bar"], dispenser.candidates("ba"));

        let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo,fog"));
        assert_eq!(vec!["fog", "foo"], dispenser.candidates("fo"));
    }

    #[test]
    fn candidates_when_entry_is_a_sequence_of_tags() {
        let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo,fog"));
        assert_eq!(vec!["fog", "foo"], dispenser.candidates("bar,fo"));
    }
}
