use crate::model::find::Find;
#[derive(Clone, Debug)]
pub struct Criterion {
    find: Find,
    pattern: String,
}

impl Criterion {
    pub fn new(find: Find, pattern: &str) -> Self {
        Self {
            find,
            pattern: pattern.to_string(),
        }
    }
}
impl std::fmt::Display for Criterion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.find, self.pattern)
    }
}
