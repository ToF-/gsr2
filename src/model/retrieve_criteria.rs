use crate::model::image_data::ImageData;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::model::categories::Categories;
use crate::model::predicate::Predicate;
use crate::model::tag_selection_criteria::TagSelectionCriteria;
use regex::Regex;
use std::env::args;
use std::io::Error as IOError;
use std::io::Result as IOResult;

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

impl RetrieveCriteria {
    pub fn from_command_line_arguments(
        args: &CommandLineArguments,
        predicate_opt: Option<Predicate>,
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
                Ok(Self {
                    tag_selection_criteria: tag_selection_criteria.clone(),
                    categories: args
                        .categories
                        .clone()
                        .as_ref()
                        .map(|s| Categories::from_string(s)),
                    label: args.label.clone(),
                    extraction: extraction.clone(),
                    color_filter: args.filter.clone(),
                    pattern: regex_opt,
                    cover: args.covers,
                    parent_opt: args.directory.clone(),
                    predicate_opt,
                })
            })
        })
    }

    pub fn matches(&self, file_path: &str, image_data: &ImageData) -> bool {
        false
    }

    pub fn extraction_file_paths(extract_file: &str) -> IOResult<Vec<String>> {
        File::open(extract_file).and_then(|file| {
            let reader = BufReader::new(file);
            reader.lines().collect()
        })
    }
}
