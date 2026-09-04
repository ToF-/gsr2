use crate::model::predicate::Predicate;
#[derive(Clone, Debug)]
pub struct Location {
    sub_directory: Option<String>,
    predicate: Option<Predicate>,
    position: usize,
    covers_only: bool,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            sub_directory: None,
            predicate: None,
            position: 0,
            covers_only: false,
        }
    }
}
impl Location {
    pub fn new(
        sub_directory: Option<String>,
        predicate: Option<Predicate>,
        position: usize,
        covers_only: bool,
    ) -> Self {
        Self {
            sub_directory,
            predicate,
            position,
            covers_only,
        }
    }

    pub fn sub_directory(&self) -> Option<String> {
        self.sub_directory.clone()
    }

    pub fn predicate(&self) -> Option<Predicate> {
        self.predicate.clone()
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn covers_only(&self) -> bool {
        self.covers_only
    }
}
