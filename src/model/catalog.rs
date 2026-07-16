use std::io::{Error, Result};
use lexpr::parse::{Result as ParseResult};
use lexpr::Value;
use lexpr::Value::Cons;
use lexpr::Value::Symbol;

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

    pub fn from_value(value: &Value, root_level: bool) -> Result<SubCategory> {
        println!("{:?}", value);
        match value {
            Cons(cons) => {
                let car = cons.car();
                let cdr = cons.cdr();
                match car {
                    Symbol(symbol) => Ok( SubCategory {
                        name: symbol.to_string(),
                        sub_categories: vec![],
                    }),
                    _ => Err(Error::other(format!("incorrect s_expression value: {:?}", value))),
                }
            },
            _ => Err(Error::other(format!("incorrect s_expression value: {:?}", value))),
        }
    }
}
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
        let s_expression = "(-)";
        let catalog = Catalog::from_sexpr(s_expression).expect("incorrect s-expression for catalog");
        assert_eq!("-".to_string(), catalog.root.name())
    }

    /*
    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_a_sub() {
        let s_expression = "(- animals)";
        let catalog = Catalog::from_sexpr(s_expression).expect("incorrect s-expression for catalog");
        assert_eq!("-", catalog.root.name());
        assert_eq!(1, catalog.root.sub_categories().len());
        assert_eq!("animals", catalog.root.sub_categories[0].name());
    }
    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_two_subs() {
        let s_expression = "(- animals plants)";
        let catalog = Catalog::from_sexpr(s_expression).expect("incorrect s-expression for catalog");
        assert_eq!("-", catalog.root.name());
        assert_eq!(2, catalog.root.sub_categories().len());
        assert_eq!("animals", catalog.root.sub_categories[0].name());
        assert_eq!("plants", catalog.root.sub_categories[1].name());
    }
    */
}
