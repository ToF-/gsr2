use crate::model::tags::empty_tags;
use crate::model::tags::Tags;

#[derive(Debug)]
pub struct CompletionDispenser {
    tags: Tags,
    entry: String,
}

impl Default for CompletionDispenser {
    fn default() -> Self {
        Self {
            tags: empty_tags(),
            entry: String::new(),
        }
    }
}
impl CompletionDispenser {

    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_with(tags: Tags) -> Self {
        Self {
            tags,
            entry: String::new(),
        }
    }

    pub fn set_entry(&mut self, entry: &str) {
        self.entry = entry.into()

    }

    pub fn candidates(&self) -> Vec<String> {
        let mut entry_tags = self.entry.split(',');
        if let Some(last_entry_tag) = entry_tags.next_back()
            && last_entry_tag.len() >= 2 {
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
            assert!(dispenser.candidates().is_empty());
        }

        #[test]
        fn no_candidates_when_only_one_char_entry() {
            let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo"));
            dispenser.set_entry("f");
            assert!(dispenser.candidates().is_empty());
        }

        #[test]
        fn candidates_when_two_or_more_chars_entry() {
            let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo"));
            dispenser.set_entry("ba");
            assert_eq!(vec!["bar"], dispenser.candidates());

            let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo,fog"));
            dispenser.set_entry("fo");
            assert_eq!(vec!["fog", "foo"], dispenser.candidates());
        }

        #[test]
        fn candidates_when_entry_is_a_sequence_of_tags() {
            let mut dispenser = CompletionDispenser::new_with(tags_from_str("bar,foo,qux,zoo,fog"));
            dispenser.set_entry("bar,fo");
            assert_eq!(vec!["fog", "foo"], dispenser.candidates());
        }
    }
