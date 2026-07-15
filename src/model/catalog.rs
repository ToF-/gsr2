use std::io::{Error, Result};
use lexpr::parse::{Result as ParseResult};
use lexpr::Value;

pub struct SubCategory {
    name: String,
    sub_categories: Vec<SubCategory>,
}

impl SubCategory {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
pub struct Catalog {
    root: SubCategory,
}

impl Catalog {
    pub fn from_sexpr(source: &str) -> Result<Self> {
        let value: lexpr::Value = lexpr::from_str(source).unwrap();
        Ok( Catalog {
            root: SubCategory {
                name: "-".to_string(),
                sub_categories: vec![], 
            }
        })
    }
}

fn value_to_sub_category(value: &Value) -> Result<SubCategory> {
    todo!()
}
/*
use lexpr::Value;

fn value_to_node(value: &Value) -> Node {
    let items = value
        .to_vec()
        .expect("expected a list");

    let name = items[0]
        .as_symbol()
        .expect("node name must be a symbol")
        .to_string();

    let children = items[1..]
        .iter()
        .flat_map(|v| {
            match v {
                Value::Cons(_) => {
                    // Nested node: (foo ...)
                    vec![value_to_node(v)]
                }

                Value::Symbol(s) => {
                    // Leaf name: lion
                    vec![Node {
                        name: s.to_string(),
                        children: Vec::new(),
                    }]
                }

                _ => panic!("unexpected value: {:?}", v),
            }
        })
        .collect();

    Node {
        name,
        children,
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_sub_categories_from_a_s_expression() {
        let s_expression = "(- (animals (mammals lion deer cat) (birds crow parakeet)) (plants tree rose))";
        let catalog = Catalog::from_sexpr(s_expression).expect("incorrect s-expression for catalog");
        assert_eq!("-".to_string(), catalog.root.name())
    }
}
