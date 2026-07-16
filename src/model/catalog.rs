use std::io::{Error, Result};
use lexpr::parse::{Result as ParseResult};
use lexpr::Value;
use lexpr::Value::Cons;
use lexpr::Value::Symbol;
use lexpr::Value::Null;

#[derive(Debug, Clone)]
pub struct SubCategory {
    name: String,
    sub_categories: Vec<SubCategory>,
}

impl SubCategory {

    pub fn leave(name: &str) -> Self {
        SubCategory {
            name: name.to_string(),
            sub_categories: vec![],
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn sub_categories(&self) -> Vec<SubCategory> {
        self.sub_categories.clone()
    }

    pub fn from_symbol(value: &Value, root_level: bool) -> Result<SubCategory> {
        match(value) {
            Symbol(symbol) => { 
                if root_level && symbol.to_string() != "-" {
                    Err(Error::other(format!("incorrect s_expression value: missing root symbol in {:?}", value)))
                } else {
                    Ok( SubCategory {
                        name: symbol.to_string(),
                        sub_categories: vec![],
                    })
                }
            },
            _ => Err(Error::other(format!("incorrect s_expression value, symbol expected: {:?}", value))),
        }
    }

    pub fn from_value(value: &Value, root_level: bool) -> Result<SubCategory> {
        println!("{:?}", value);
        match value {
            Symbol(symbol) => Self::from_symbol(value, root_level),
            Cons(cons) => {

                let car = cons.car();
                let mut cdr = cons.cdr();
                match Self::from_symbol(car, root_level) {
                    Ok(mut sub) => {
                        while cdr.is_cons() {
                            match Self::from_value(cdr, false) {
                                Ok(next) => {
                                    sub.sub_categories.push(next);
                                },
                                e => return e
                            }
                            cdr = cdr.as_cons().expect("cdr should be cons").cdr()
                        };
                        Ok(sub)
                    },
                    e => e,
                }
            }
            _ => Err(Error::other(format!("incorrect s_expression value: {:?}", value))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Catalog {
    root: SubCategory,
}

impl Catalog {

    pub fn from_sexpr(source: &str) -> Result<Self> {
        match lexpr::from_str(source) {
            Ok(value) => {
                match SubCategory::from_value(&value, true) {
                    Ok(root) => Ok(
                        Catalog { root }
                    ),
                    Err(err) => Err(Error::other(err)),
                }
            },
            Err(err) => Err(Error::other(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_sub_categories_from_a_s_expression_with_only_root() {
        let catalog = Catalog::from_sexpr("(-)").expect("incorrect s-expression for catalog");
        assert_eq!("-".to_string(), catalog.root.name())
    }
    #[test]
    fn root_subcategorys_should_be_dash() {
        assert!(Catalog::from_sexpr("(meh)").is_err());
    }

    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_a_sub() {
        let catalog = Catalog::from_sexpr("(- foo)").expect("incorrect s-expression for catalog");
        assert_eq!("-", catalog.root.name());
        assert_eq!(1, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories[0].name());
    }
    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_two_subs() {
        let s_expression = "(- foo bar)";
        let catalog = Catalog::from_sexpr("(- foo bar)").expect("incorrect s-expression for catalog");
        assert_eq!("-", catalog.root.name());
        assert_eq!(2, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories[0].name());
        assert_eq!("bar", catalog.root.sub_categories[1].name());
    }
    #[test]
    fn creating_sub_categories_from_s_expression_with_root_and_several_subs() {
        let s_expression = "(- foo bar qux)";
        let catalog = Catalog::from_sexpr(s_expression).expect("incorrect s-expression for catalog");
        assert_eq!("-", catalog.root.name());
        assert_eq!(3, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories[0].name());
        assert_eq!("bar", catalog.root.sub_categories[1].name());
        assert_eq!("qux", catalog.root.sub_categories[2].name());
    }
    #[test]
    fn creating_sub_categories_from_s_expression_with_root_and_sub_sub_catagory() {
        let s_expression = "(- (foo bar) qux)";
        let catalog = Catalog::from_sexpr(s_expression).expect("incorrect s-expression for catalog");
        assert_eq!("-", catalog.root.name());
    }
}
