use itertools::Itertools;
use lexpr::Value;
use lexpr::Value::Cons;
use lexpr::Value::Null;
use lexpr::Value::Symbol;
use lexpr::parse::Result as ParseResult;
use std::io::{Error, Result};

pub const TOP_CATEGORY: &str = "-";

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

    pub fn format_at_level(&self, level: usize) -> String {
        let indent: String = " ".repeat(level * 2);
        if self.sub_categories.is_empty() {
            format!("{}{}", indent, self.name)
        } else {
            let children_string: String = self
                .sub_categories
                .iter()
                .map(|child| format!("\n{}{}", indent, child.format_at_level(level + 1)))
                .collect::<Vec<String>>()
                .join("");
            format!("{}({}{})", indent, self.name, children_string)
        }
    }

    pub fn add_sub_category(&mut self, sub_category_name: &str, category_name: &str) -> Result<()> {
        if self.name == category_name {
            self.sub_categories.push(Self::leave(sub_category_name));
            Ok(())
        } else {
            let mut result: Result<()> = Ok(());
            for mut sub_category in self.sub_categories.iter_mut() {
                let sub_result = sub_category.add_sub_category(sub_category_name, category_name);
                if sub_result.is_err() {
                    result = sub_result;
                    break;
                };
            }
            result
        }
    }

    pub fn remove_sub_category(
        &mut self,
        sub_category_name: &str,
        remove_subs: bool,
    ) -> Result<()> {
        if let Some((index, sub_category)) = self
            .sub_categories
            .iter()
            .enumerate()
            .find(|(_, sub_category)| sub_category.name == sub_category_name)
        {
            if !sub_category.sub_categories.is_empty() && !remove_subs {
                return Err(Error::other(format!(
                    "category: {} has subcategories and cannot be deleted",
                    sub_category_name
                )));
            } else {
                self.sub_categories.remove(index);
                Ok(())
            }
        } else {
            let mut result: Result<()> = Ok(());
            for mut sub_category in self.sub_categories.iter_mut() {
                let sub_result = sub_category.remove_sub_category(sub_category_name, remove_subs);
                if sub_result.is_err() {
                    result = sub_result;
                    break;
                };
            }
            result
        }
    }

    pub fn from_cons(value: &Value) -> Result<Vec<SubCategory>> {
        if value.is_null() {
            return Ok(vec![]);
        };
        let cons = value.as_cons().unwrap();
        match cons.car() {
            Symbol(symbol) => {
                // (foo • …
                match cons.cdr() {
                    Null =>
                    //  (foo • ∅)
                    {
                        Ok(vec![Self::leave(symbol)])
                    }
                    Cons(_) => {
                        // (foo • (… • …))
                        let mut subs = vec![Self::leave(symbol)];
                        match Self::from_cons(cons.cdr()) {
                            Ok(next) => {
                                subs.extend(next);
                                Ok(subs)
                            }
                            _ => Err(Error::other(format!(
                                "incorrect s_expression value for cdr: {:?}",
                                cons.cdr()
                            ))),
                        }
                    }
                    _ => Err(Error::other(format!(
                        "incorrect s_expression value for cdr: {:?}",
                        cons.cdr()
                    ))),
                }
            }
            Cons(_) => {
                let inner = cons.car().as_cons().unwrap();
                if inner.car().is_symbol() && inner.cdr().is_null() {
                    return Err(Error::other(format!(
                        "incorrect s_expression value with singleton: {:?}",
                        value
                    )));
                };
                match Self::from_value(cons.car()) {
                    Ok(sub1) => match Self::from_cons(cons.cdr()) {
                        Ok(subs2) => {
                            let mut result = vec![sub1];
                            result.extend(subs2);
                            Ok(result)
                        }
                        Err(e) => Err(Error::other(e)),
                    },
                    Err(e) => Err(Error::other(e)),
                }
            }
            _ => Err(Error::other(format!(
                "incorrect s_expression value for car: {:?}",
                cons.car()
            ))),
        }
    }

    pub fn from_value(value: &Value) -> Result<SubCategory> {
        match value {
            Cons(cons) => {
                let car = cons.car();
                let cdr = cons.cdr();
                if car.is_symbol() {
                    let symbol = car.as_symbol().unwrap();
                    if cdr.is_null() {
                        Ok(Self::leave(symbol))
                    } else {
                        Self::from_cons(cdr).and_then(|subs| {
                            Ok(SubCategory {
                                name: symbol.to_string(),
                                sub_categories: subs,
                            })
                        })
                    }
                } else if car.is_cons() {
                    if cdr.is_cons() {
                        match Self::from_value(cdr) {
                            Ok(sub) => Ok(sub),
                            e => e,
                        }
                    } else {
                        Err(Error::other(format!(
                            "incorrect s_expression value: {:?}",
                            value
                        )))
                    }
                } else {
                    Err(Error::other(format!(
                        "incorrect s_expression value: {:?}",
                        value
                    )))
                }
            }
            _ => Err(Error::other(format!(
                "incorrect s_expression value: {:?}",
                value
            ))),
        }
    }
}
