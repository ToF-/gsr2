use crate::cli::command_line_arguments::CommandLineArguments;
use crate::model::catalog::Catalog;
use crate::model::categories::Categories;
use crate::model::color_range::ColorRange;
use crate::model::image_data::ImageData;
use crate::model::picture::Picture;
use crate::model::predicate::Predicate;
use crate::model::tag_selection_criteria::TagSelectionCriteria;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error as IOError;
use std::io::Result as IOResult;

#[derive(Clone, Debug)]
pub struct RetrieveCriteria {
    pub tag_selection_criteria: TagSelectionCriteria,
    pub categories: Option<Categories>,
    pub label: Option<String>,
    pub extraction: Option<Vec<String>>,
    pub color_range_opt: Option<ColorRange>,
    pub pattern: Option<Regex>,
    pub cover: bool,
    pub parent_opt: Option<String>,
    pub predicate_opt: Option<Predicate>,
    pub catalog_opt: Option<Catalog>,
}

impl RetrieveCriteria {
    pub fn new(
        args: &CommandLineArguments,
        predicate_opt: Option<Predicate>,
        catalog_opt: Option<Catalog>,
    ) -> IOResult<Self> {
        let result: IOResult<Option<Vec<String>>> = if let Some(list_file) = &args.extraction {
            match Self::extraction_file_paths(list_file) {
                Ok(list) => Ok(Some(list)),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        };
        result.and_then(|extraction| {
            let result: IOResult<Option<Regex>> = match args.clone().pattern {
                Some(pattern) => match Regex::new(&pattern) {
                    Ok(re) => Ok(Some(re)),
                    Err(e) => Err(IOError::other(e)),
                },
                None => Ok(None),
            };
            result.and_then(|regex_opt| {
                let tag_selection_criteria = TagSelectionCriteria::from_args(args);
                let result = if let Some(filter) = &args.filter {
                    match ColorRange::from_string(&filter) {
                        Ok(color_range) => Ok(Some(color_range)),
                        Err(e) => Err(IOError::other(e)),
                    }
                } else {
                    Ok(None)
                };
                result.and_then(|color_range_opt| {
                    Ok(Self {
                        tag_selection_criteria: tag_selection_criteria.clone(),
                        categories: args
                            .categories
                            .clone()
                            .as_ref()
                            .map(|s| Categories::from_string(s)),
                        label: args.label.clone(),
                        extraction: extraction.clone(),
                        color_range_opt,
                        pattern: regex_opt,
                        cover: args.covers,
                        parent_opt: if args.structured {
                            None
                        } else {
                            args.directory.clone()
                        },
                        predicate_opt,
                        catalog_opt,
                    })
                })
            })
        })
    }

    pub fn matches(&self, picture: &Picture) -> bool {
        let image_data = picture.image_data().unwrap();
        self.check_one_of_categories(&image_data)
            && self.check_tag_selection(&image_data)
            && self.check_label(&image_data)
            && self.check_pattern(&picture.file_path())
            && self.check_extraction(&picture.file_path())
            && self.check_color_range(&picture.file_path())
            && self.check_predicate(&picture)
    }

    fn check_one_of_categories(&self, image_data: &ImageData) -> bool {
        if let Some(ref catalog) = self.catalog_opt
            && let Some(categories) = self.categories.clone()
            && let Some(category_name) = image_data.category_name()
        {
            catalog.is_one_of(&categories, &category_name)
        } else {
            true
        }
    }

    fn check_tag_selection(&self, image_data: &ImageData) -> bool {
        if self.tag_selection_criteria.is_empty() {
            true
        } else {
            self.tag_selection_criteria.matches(image_data.tags.clone())
        }
    }

    fn check_label(&self, image_data: &ImageData) -> bool {
        if self.label.clone().is_none() {
            true
        } else {
            self.label.as_ref().unwrap() == &image_data.label()
        }
    }

    fn check_pattern(&self, file_path: &str) -> bool {
        if self.pattern.clone().is_none() {
            true
        } else {
            self.pattern.as_ref().unwrap().is_match(file_path)
        }
    }

    fn check_extraction(&self, file_path: &str) -> bool {
        if self.extraction.clone().is_none() {
            true
        } else {
            self.extraction
                .as_ref()
                .unwrap()
                .contains(&file_path.to_string())
        }
    }

    fn check_color_range(&self, file_path: &str) -> bool {
        if self.color_range_opt.clone().is_none() {
            true
        } else {
            self.color_range_opt.as_ref().unwrap().matches(file_path)
        }
    }

    fn check_predicate(&self, picture: &Picture) -> bool {
        if let Some(ref predicate) = self.predicate_opt {
            let function = &predicate.function;
            function(picture)
        } else {
            true
        }
    }

    pub fn extraction_file_paths(extract_file: &str) -> IOResult<Vec<String>> {
        File::open(extract_file).and_then(|file| {
            let reader = BufReader::new(file);
            reader.lines().collect()
        })
    }
}
