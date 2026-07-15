use std::io::{Error, Result};
use lexpr::parse::{Result as ParseResult};
use lexpr::Value;
use lexpr::Cons;

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
}
pub struct Catalog {
    root: SubCategory,
}

impl Catalog {

    pub fn from_sexpr(source: &str) -> Result<Self> {
        match lexpr::from_str(source) {
            Ok(value) => {
                match Self::value_to_subcategory(&value) {
                    Some(root) => Ok(Catalog { root }),
                    None => Err(Error::other(format!("incorrect catalog s-expression"))),
                }
            },
            Err(err) => Err(Error::other(err)),
        }
    }
    fn value_to_subcategory(value: &Value) -> Option<SubCategory> {
        println!("{:?}", value);
        if value.is_cons() {
            let cons = value.as_cons().unwrap();
            let car = cons.car();
            let mut cdr = cons.cdr();
            if car.is_symbol() {
                let name = car.as_symbol().unwrap().to_string();
                if cdr.is_null() {
                    Some(SubCategory::leave(&name))
                } else {
                    let mut sub_categories: Vec<SubCategory> = vec![];
                    while cdr.is_cons() {
                        let cdr_cons = cdr.as_cons().unwrap();
                        let cadr = cdr_cons.car();
                        let ccdr = cdr_cons.cdr();
                        if cadr.is_symbol() {
                            match Self::value_to_subcategory(cdr) {
                                Some(sub_category) => {
                                    sub_categories.push(sub_category);
                                },
                                None => return None,
                            }
                        } else if cadr.is_cons() {
                            cdr = cadr;
                        }
                    };
                    Some(SubCategory {
                        name: name,
                        sub_categories: sub_categories.clone(),
                    })
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_sub_categories_from_a_s_expression_with_only_root() {
        let s_expression = "(- )";
        let catalog = Catalog::from_sexpr(s_expression).expect("incorrect s-expression for catalog");
        assert_eq!("-".to_string(), catalog.root.name())
    }

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
}
