use std::collections::HashMap;
use crate::model::sub_category::SubCategory;
use std::fs;
use std::io::{Error, Result};
use lexpr::parse::{Result as ParseResult};
use lexpr::Value;
use lexpr::Value::Cons;
use lexpr::Value::Symbol;
use lexpr::Value::Null;

#[derive(Debug, Clone)]
pub struct Catalog {
    root: SubCategory,
    reverse_tree: HashMap<String,String>,
}

impl Catalog {

    pub fn from_sexpr(source: &str) -> Result<Self> {
        match lexpr::from_str(source) {
            Ok(value) => {
                match SubCategory::from_value(&value) {
                    Ok(root) => {
                    if root.name() == "-" {
                        Ok( Catalog { root, reverse_tree: HashMap::new(), })
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

    pub fn is_a(&self, sub_category_name: &str, category_name: &str) -> bool {
        if sub_category_name == category_name {
            return true
        };
        return false
    }
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
        assert!(!catalog.is_a("bug","bar"));
    }
//    #[test]
    fn is_a_sub_category_relationship_sub_category_case() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux law))").expect("incorrect sexpr");
        assert!(catalog.is_a("bar","foo"));
        assert!(!catalog.is_a("foo","bar"));
    }
}
