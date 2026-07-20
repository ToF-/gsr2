use lexpr::print::Options;
use lexpr::to_string_custom;
use crate::model::categories::Categories;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::model::sub_category::SubCategory;
use std::fs;
use std::io::{Error, Result};
use lexpr::parse::{Result as ParseResult};
use lexpr::Value;
use lexpr::Value::Cons;
use lexpr::Value::Symbol;
use lexpr::Value::Null;

type ReverseTree = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Catalog {
    root: SubCategory,
    reverse_tree: ReverseTree,
    s_expression: Value,
}

impl Catalog {

    pub fn from_sexpr(source: &str) -> Result<Self> {
        match lexpr::from_str(source) {
            Ok(value) => {
                match SubCategory::from_value(&value) {
                    Ok(root) => {
                    if root.name() == "-" {
                        let mut tree: ReverseTree = ReverseTree::new();
                        match make_reverse_tree(&mut tree, &root) {
                            Ok(_) => Ok( Catalog { root, reverse_tree: tree, s_expression: value, }),
                            Err(err) => Err(Error::other(err)),
                        }
                    } else {
                        Err(Error::other(format!("incorrect s_expression value: missing root symbol in {:?}", value)))
                    }},
                    Err(err) => Err(Error::other(err)),
                }
            },
            Err(err) => Err(Error::other(err)),
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                Self::from_sexpr(&content)
            },
            Err(e) => Err(Error::other(e)),
        }

    }
    pub fn root(&self) -> SubCategory {
        self.root.clone()
    }

    pub fn s_expression(&self) -> String {
        to_string_custom(&self.s_expression, Options::elisp()).expect("can't pretty print catalog")
    }

    pub fn is_a(&self, target_category_name: &str, sub_category_name: &str) -> bool {
        if self.reverse_tree.get(target_category_name).is_none() {
            return false;
        }
        if self.reverse_tree.get(sub_category_name).is_none() {
            return false;
        }
        if sub_category_name == target_category_name {
            true
        } else {
            if let Some(parent_category_name) = self.reverse_tree.get(sub_category_name) {
                parent_category_name == target_category_name || self.is_a(target_category_name, parent_category_name)
            } else {
                false
            }
        }
    }

    pub fn is_one_of(&self, categories: &Categories, category_name: &str) -> bool {
        for selected_category_name in categories.names() {
            if self.is_a(&selected_category_name, category_name) {
                return true
            }
        }
        return false
    }
}

fn make_reverse_tree(tree: &mut ReverseTree, parent: &SubCategory) -> Result<()> {
    let mut result: Result<()> = Ok(());
    parent.sub_categories().iter().for_each( |child| {
        if result.is_ok() {
            match make_reverse_tree(tree, child) {
                Ok(_) => {},
                Err(err) => { result = Err(err); },
            }
        };
        if result.is_ok() {
            let key: String = child.name();
            let value: String = parent.name();
            match tree.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(value);
                },
                Entry::Occupied(entry) => {
                    result = Err(Error::other(format!("duplicate subcategory:{}", child.name())));
                },
            };
        };
    });
    result
}


pub fn format_value(v: &Value) -> String {
    match v {
        Null => "∅".to_string(),
        Cons(c) => format!("({} • {})", format_value(c.car()), format_value(c.cdr())),
        Symbol(s) => s.to_string(),
        _ => "…".to_string(),
    }
}

pub fn format_sub_category(s: &SubCategory) -> String {
    if s.sub_categories().is_empty() {
        format!("{}", s.name())
    } else {
        let items: Vec<String> = s.sub_categories().iter().map(format_sub_category).collect();
        let ssubs: String = items.join(", ");
        format!("{} [{}]", s.name(), ssubs)
    }
}
pub fn format_catalog(c: &Catalog) -> String {
    format!("Cat {}", format_sub_category(&c.root))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn creating_sub_categories_from_a_s_expression_with_only_root() {
        let catalog = Catalog::from_sexpr("(-)").expect("incorrect sexpr");
        assert_eq!("-".to_string(), catalog.root.name())
    }
    #[test]
    fn root_subcategory_name_should_be_dash() {
        assert!(Catalog::from_sexpr("(meh)").is_err());
    }
    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_a_sub() {
        let catalog = Catalog::from_sexpr("(- foo)").expect("incorrect sexpr");
        assert_eq!("-", catalog.root.name());
        assert_eq!(1, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories()[0].name());
    }
    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_three_subs() {
        let catalog = Catalog::from_sexpr("(- foo bar qux)").expect("incorrect sexpr");
        assert_eq!("-", catalog.root.name());
        assert_eq!(3, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories()[0].name());
        assert_eq!("bar", catalog.root.sub_categories()[1].name());
        assert_eq!("qux", catalog.root.sub_categories()[2].name());
    }
    #[test]
    fn creating_sub_categories_from_s_expression_with_root_and_sub_subs() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux law))").expect("incorrect sexpr");
        assert_eq!("-", catalog.root.name());
        println!("{:?}", catalog);
        assert_eq!(2, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories()[0].name());
        assert_eq!("bar", catalog.root.sub_categories()[0].sub_categories()[0].name());
        assert_eq!("qux", catalog.root.sub_categories()[1].name());
        assert_eq!("law", catalog.root.sub_categories()[1].sub_categories()[0].name());
    }
    #[test]
    fn singleton_sub_categories_are_not_allowed() {
        assert!(Catalog::from_sexpr("(- foo (bar))").is_err());
        assert!(Catalog::from_sexpr("(- (foo) bar)").is_err());
        assert!(Catalog::from_sexpr("(- ((foo bar))").is_err());
        assert!(Catalog::from_sexpr("(- (foo bar) (qux (law)))").is_err());
        assert!(Catalog::from_sexpr("(- ((((foo)))))").is_err());
        assert!(Catalog::from_sexpr("((-))").is_err());
    }
    #[test]
    fn duplicate_categories_are_not_allowed() {
        assert!(Catalog::from_sexpr("(- (foo bar) (gus bin (pog (qux bar))))").is_err());
        assert!(Catalog::from_sexpr("(- (foo foo))").is_err());
    }
    #[test]
    fn catalog_can_be_read_from_a_file() {
        let catalog = Catalog::from_file("testdata/catalog.sexp").expect("incorrect catalog or I/O");
        let content = read_to_string("testdata/catalog.sexp").expect("I/O");
        let value =  lexpr::from_str(&content).expect("incorrect sexp");
        println!("{}", format_value(&value));
        let expected = "\"Cat - [foo [bar, qux], bog [gap], pat [jxs [lam, lom, lum], zzz [tic, tac, toe], pin [blo]]]\"";
        assert_eq!(expected, format!("{:?}", format_catalog(&catalog)));
    }
    #[test]
    fn is_a_sub_category_relationship_equality_case() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux law))").expect("incorrect sexpr");
        assert!(catalog.is_a("bar","bar"));
    }
    #[test]
    fn is_a_sub_category_relationship_sub_category_case() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux law))").expect("incorrect sexpr");
        assert!(catalog.is_a("foo","bar"));
        assert!(catalog.is_a("qux","law"));
        assert!(!catalog.is_a("foo","qux"));
    }
    #[test]
    fn is_a_sub_category_relationship_sub_sub_category_case() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux (law bug)))").expect("incorrect sexpr");
        assert!(catalog.is_a("qux","bug"));
        assert!(!catalog.is_a("foo","bug"));
    }
    #[test]
    fn is_a_sub_category_relationship_inexistent_sub_category_case() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux (law bug)))").expect("incorrect sexpr");
        assert!(!catalog.is_a("-","paw"));
        assert!(!catalog.is_a("paw","bar"));
        assert!(!catalog.is_a("paw","paw"));
    }
    #[test]
    fn is_a_sub_category_relationship_root_target_sub_category_case() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux (law bug)))").expect("incorrect sexpr");
        assert!(!catalog.is_a("-","bug"));
    }
    #[test]
    fn is_one_of_categories_from_a_catalog() {
        let categories = Categories::from_string("bam,foo");
        let catalog = Catalog::from_sexpr("(- (foo (bar gus)) (qux (bam bol)))").expect("incorrect sexpr");
        assert!(catalog.is_one_of(&categories, "gus"));
        assert!(!catalog.is_one_of(&categories, "bap"));
        assert!(catalog.is_one_of(&categories, "bol"));
        assert!(catalog.is_one_of(&categories, "foo"));
        assert!(!catalog.is_one_of(&categories, "qux"));
    }
}
