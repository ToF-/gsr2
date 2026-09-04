use crate::model::catalog::Catalog;
use crate::model::categories::Categories;
use crate::model::criterion::Criterion;
use crate::model::find::Find;
use crate::model::picture::Picture;
use crate::model::tags::tags_from_str;
use regex::Error;
use regex::Regex;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub struct Predicate {
    pub function: Arc<dyn Fn(&Picture) -> bool>,
    pub criteria: Vec<Criterion>,
}

impl Predicate {
    pub fn new(pattern: &str, find: Find, catalog: Catalog) -> Result<Self, Error> {
        let result = match Regex::new(pattern) {
            Ok(re) => {
                let function: Arc<dyn Fn(&Picture) -> bool> = match find {
                    Find::Name => {
                        Arc::new(move |picture: &Picture| re.is_match(&picture.file_name()))
                    }
                    Find::FilePath => {
                        Arc::new(move |picture: &Picture| re.is_match(&picture.file_path()))
                    }
                    Find::Label => Arc::new(move |picture: &Picture| re.is_match(&picture.label())),
                    Find::Category => {
                        Arc::new(move |picture: &Picture| re.is_match(&picture.category_name()))
                    }
                    Find::SubCategory => {
                        let categories: Categories = Categories::from_string(pattern);
                        Arc::new(move |picture: &Picture| {
                            catalog.is_one_of(&categories, &picture.category_name())
                        })
                    }
                    Find::SomeTags => {
                        let tags = tags_from_str(pattern);
                        Arc::new(move |picture: &Picture| {
                            picture.tags().intersection(&tags).count() > 0
                        })
                    }
                    Find::AllTags => {
                        let tags = tags_from_str(pattern);
                        Arc::new(move |picture: &Picture| tags.is_subset(&picture.tags()))
                    }
                };
                Ok(function)
            }
            Err(e) => Err(e),
        };
        result.map(|function| Self {
            function: function,
            criteria: vec![Criterion::new(find, pattern)],
        })
    }
    pub fn and(predicate: Self, other: Self) -> Self {
        let function: Arc<dyn Fn(&Picture) -> bool> = Arc::new(move |picture: &Picture| {
            (predicate.function)(picture) && (other.function)(picture)
        });
        let mut criteria = predicate.criteria.clone();
        let mut other_criteria = other.criteria.clone();
        criteria.append(&mut other_criteria);
        Self { function, criteria }
    }
}

impl std::fmt::Debug for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<predicate>");
        let s = self
            .criteria
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}
impl Display for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .criteria
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}
