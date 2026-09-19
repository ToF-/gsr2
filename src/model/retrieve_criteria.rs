use crate::model::predicate::Predicate;
use regex::Regex;
use crate::model::categories::Categories;
use crate::model::tag_selection_criteria::TagSelectionCriteria;

#[derive(Debug)]
pub struct RetrieveCriteria {
    pub tag_selection_criteria: TagSelectionCriteria,
    pub categories: Option<Categories>,
    pub label: Option<String>,
    pub extraction: Option<Vec<String>>,
    pub color_filter: Option<String>,
    pub pattern: Option<Regex>,
    pub cover: bool,
    pub parent_opt: Option<String>,
    pub predicate_opt: Option<Predicate>,
}


