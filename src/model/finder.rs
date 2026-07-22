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

#[derive(Debug,Clone)]
pub struct Finder { 

    items: Vec<Picture>,
    predicate: Option<Predicate>,
    position: usize,
}

impl Finder {
    pub fn new(items: Vec<Picture>) -> Self {
            Self { items: items.clone(), position: 0, predicate: None, }
        }

    pub fn first(&mut self, predicate: Predicate) -> Option<usize> 
    {
        self.predicate = Some(predicate);
        self.first_from_index(0)
    }

    pub fn first_from_index(&mut self, start: usize) -> Option<usize>
    {
        self.position = start;
        self.next()
    }

    pub fn next(&mut self) -> Option<usize>
    {

        let predicate = &<std::option::Option<Predicate> as Clone>::clone(&self.predicate).unwrap();
        let function = &predicate.function;
        let index = self.items[self.position..]
            .iter()
            .position(|item| function(&item))
            .map(|i| self.position + i)?;

        self.position = index + 1;
        Some(index)
    }
}

