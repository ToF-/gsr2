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
    
    /* 
    (-
       (foo
          (bar
           qux))
       law) 
    = (- • (foo • ((bar • (qux • ∅ )) • (law • ∅))))
    Sub {
        name: -
        subs: [
                Sub {
                    name: foo
                    subs: [
                            Sub {
                                name: bar
                                subs: []
                                },
                            Sub {
                                name: qux
                                subs: []
                                }
                           ]
                      },
                Sub {
                    name: law
                    subs: []
                    }
                ]
        }

(-) → (- • ∅)
(- foo) → (- • (foo • ∅))
(- foo bar) → (- • (foo • (bar • ∅)))
(- foo bar qux) → (- • (foo • (bar • (qux • ∅))))
(- foo (bar bog bug)) → (- • (foo • ((bar • (bog • (bug • ∅))) • ∅)))
(- (foo phi pho fux) (bar bog bug)) → (- • ((foo • (phi • (pho • (fux • ∅)))) • ((bar • (bog • (bug • ∅))) • ∅)))
(- (ill) (legal)) → (- • ((ill • ∅) • ((legal • ∅) • ∅)))
*/
    pub fn from_cons(value: &Value) -> Result<Vec<SubCategory>> {
        println!("from_cons: {:?}", value);
        if value.is_null() {
          return Ok(vec![])  
        };
        let cons = value.as_cons().unwrap();
        match cons.car() {
            Symbol(symbol) => { // (foo • …
                println!("into symbol: {:?} with cdr: {:?}", cons.car(), cons.cdr());
                match cons.cdr() {
                    Null =>  //  (foo • ∅)
                        Ok(vec![Self::leave(symbol)]),
                    Cons(_) => { // (foo • (… • …))
                        let mut subs = vec![Self::leave(symbol)];
                        match Self::from_cons(cons.cdr()) {
                            Ok(next) => {
                                subs.extend(next);
                                Ok(subs)
                            },
                            _ => Err(Error::other(format!("incorrect s_expression value for cdr: {:?}", cons.cdr()))),
                        }
                    },
                    _ => Err(Error::other(format!("incorrect s_expression value for cdr: {:?}", cons.cdr()))),
                }
            },
            Cons(outer) => {
                println!("into Cons(Cons {:?}•{:?}", cons.car(), cons.cdr());
                match Self::from_value(cons.car()) {
                    Ok(sub1) => match Self::from_cons(cons.cdr()) {
                        Ok(subs2) => {
                            let mut result = vec![sub1];
                            result.extend(subs2);
                            Ok(result)
                        },
                        Err(e) => Err(Error::other(e)),
                    },
                    Err(e) => Err(Error::other(e)),
                }
            },
            _ => Err(Error::other(format!("incorrect s_expression value for car: {:?}", cons.car()))),
        }
    }


    pub fn from_value(value: &Value) -> Result<SubCategory> {
        println!("from_value: {:?}", value);
        match value {
            Cons(cons) => {
                let car = cons.car();
                let cdr = cons.cdr();
                if car.is_symbol() {
                    let symbol = car.as_symbol().unwrap();
                    if cdr.is_null() {
                        Ok(Self::leave(symbol))
                    } else {
                        Self::from_cons(cdr)
                            .and_then(|subs| {
                                println!("subs: {:?}", subs);
                                Ok( SubCategory {
                                    name: symbol.to_string(),
                                    sub_categories: subs,
                                })
                            })
                    }
                } else if car.is_cons() {
                    if cdr.is_cons() {
                        println!("in cons cons {:?}•{:?}", car, cdr);
                        match Self::from_value(cdr) {
                            Ok(sub) => {
                                Ok(sub)
                            },
                            e => e,
                        }
                    } else {
                        Err(Error::other(format!("incorrect s_expression value: {:?}", value)))
                    }
                } else {
                    Err(Error::other(format!("incorrect s_expression value: {:?}", value)))
                }
            },
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
                match SubCategory::from_value(&value) {
                    Ok(root) => {
                    if root.name == "-" {
                        Ok( Catalog { root })
                    } else {
                        Err(Error::other(format!("incorrect s_expression value: missing root symbol in {:?}", value)))
                    }},
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
        let catalog = Catalog::from_sexpr("(-)").expect("incorrect sexpr");
        assert_eq!("-".to_string(), catalog.root.name())
    }
    #[test]
    fn root_subcategory_name__should_be_dash() {
        assert!(Catalog::from_sexpr("(meh)").is_err());
    }

    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_a_sub() {
        let catalog = Catalog::from_sexpr("(- foo)").expect("incorrect sexpr");
        assert_eq!("-", catalog.root.name());
        assert_eq!(1, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories[0].name());
    }
    #[test]
    fn creating_sub_categories_from_a_s_expression_with_root_and_three_subs() {
        let catalog = Catalog::from_sexpr("(- foo bar qux)").expect("incorrect sexpr");
        assert_eq!("-", catalog.root.name());
        assert_eq!(3, catalog.root.sub_categories().len());
        assert_eq!("foo", catalog.root.sub_categories[0].name());
        assert_eq!("bar", catalog.root.sub_categories[1].name());
        assert_eq!("qux", catalog.root.sub_categories[2].name());
    }
    #[test]
    fn creating_sub_categories_from_s_expression_with_root_and_sub_subs() {
        let catalog = Catalog::from_sexpr("(- (foo bar) (qux law))").expect("incorrect sexpr");
        assert_eq!("-", catalog.root.name());
        println!("{:?}", catalog);
        assert_eq!(2, catalog.root.sub_categories().len());
        // assert_eq!("foo", catalog.root.sub_categories[0].name());
        // assert_eq!("bar", catalog.root.sub_categories[1].name());
        // assert_eq!("qux", catalog.root.sub_categories[2].name());
    }
}
