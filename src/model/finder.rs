use crate::model::picture::Picture;
use std::sync::Arc;

#[derive(Clone)]
pub struct Predicate {
    pub function: Arc<dyn Fn(&Picture) -> bool>,
}

impl std::fmt::Debug for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<predicate>")
    }
}

#[derive(Debug, Clone)]
pub struct Finder {
    items: Vec<Picture>,
    predicate: Option<Predicate>,
    position: usize,
}

impl Finder {
    pub fn new(items: Vec<Picture>) -> Self {
        Self {
            items: items.clone(),
            position: 0,
            predicate: None,
        }
    }

    pub fn search_in_progress(&self) -> bool {
        self.predicate.is_some()
    }

    pub fn first(&mut self, predicate: Predicate) -> Option<usize> {
        self.predicate = Some(predicate);
        self.first_from_index(0)
    }

    pub fn first_from_index(&mut self, start: usize) -> Option<usize> {
        self.position = start;
        self.next()
    }

    pub fn next(&mut self) -> Option<usize> {
        let predicate_opt = &<std::option::Option<Predicate> as Clone>::clone(&self.predicate);
        match predicate_opt {
            Some(predicate) => {
                let function = &predicate.function;
                let old_position = self.position;
                let index = self.items[self.position..]
                    .iter()
                    .position(|item| function(&item))
                    .map(|i| self.position + i)?;

                self.position = index + 1;
                if self.position == old_position {
                    eprintln!("end of search");
                    self.predicate = None;
                }
                Some(index)
            }
            None => {
                eprintln!("no current search pattern");
                None
            }
        }
    }
}
