use crate::model::catalog::Catalog;
use crate::model::categories::Categories;
use crate::model::find::Find;
use crate::model::picture::Picture;
use crate::model::tags::tags_from_str;
use regex::Error;
use regex::Regex;
use std::sync::Arc;

#[derive(Clone)]
pub struct Predicate {
    pub function: Arc<dyn Fn(&Picture) -> bool>,
}

pub fn predicate(pattern: &str, find: Find, catalog: Catalog) -> Result<Predicate, Error> {
    match Regex::new(pattern) {
        Ok(re) => {
            let predicate = match find {
                Find::Name => Predicate {
                    function: Arc::new(move |picture: &Picture| re.is_match(&picture.file_name())),
                },
                Find::Label => Predicate {
                    function: Arc::new(move |picture: &Picture| re.is_match(&picture.label())),
                },
                Find::Category => Predicate {
                    function: Arc::new(move |picture: &Picture| {
                        re.is_match(&picture.category_name())
                    }),
                },
                Find::SubCategory => {
                    let categories: Categories = Categories::from_string(pattern);
                    Predicate {
                        function: Arc::new(move |picture: &Picture| {
                            catalog.is_one_of(&categories, &picture.category_name())
                        }),
                    }
                }
                Find::SomeTags => {
                    let tags = tags_from_str(pattern);
                    Predicate {
                        function: Arc::new(move |picture: &Picture| {
                            picture.tags().intersection(&tags).count() > 0
                        }),
                    }
                }
                Find::AllTags => {
                    let tags = tags_from_str(pattern);
                    Predicate {
                        function: Arc::new(move |picture: &Picture| {
                            tags.is_subset(&picture.tags())
                        }),
                    }
                }
            };
            Ok(predicate)
        }
        Err(e) => Err(e),
    }
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

    pub fn set_items(&mut self, items: Vec<Picture>) {
        self.items = items.clone();
        let len = self.items.len();
        // a change in items len might have occured while a search was on
        if len > 0 {
            if self.position >= len {
                self.position = len - 1
            };
        } else {
            self.position = 0;
            self.predicate = None;
        }
    }

    pub fn search_in_progress(&self) -> bool {
        self.predicate.is_some()
    }

    pub fn find_first(&mut self, predicate: Predicate) -> Option<usize> {
        self.predicate = Some(predicate);
        self.find_first_from_index(0)
    }

    pub fn find_first_from_index(&mut self, start: usize) -> Option<usize> {
        self.position = start;
        self.find_next()
    }

    pub fn find_next(&mut self) -> Option<usize> {
        let predicate_opt = &<std::option::Option<Predicate> as Clone>::clone(&self.predicate);
        match predicate_opt {
            Some(predicate) => {
                let function = &predicate.function;
                let old_position = self.position;
                let index = self.items[self.position..]
                    .iter()
                    .position(|item| function(item))
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
