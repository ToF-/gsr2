use lexpr::Value;
use lexpr::Value::Cons;
use lexpr::Value::Null;
use lexpr::Value::Symbol;
use std::io::{Error, Result};
use std::ops::ControlFlow;

pub const TOP_CATEGORY: &str = "-";

#[derive(Debug, Clone)]
pub struct SubCategory {
    name: String,
    sub_categories: Vec<SubCategory>,
}

impl SubCategory {
    pub fn leaf(name: &str) -> Self {
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

    pub fn is_top_category(&self) -> bool {
        &self.name == TOP_CATEGORY
    }
    pub fn sub_category_names(&self) -> Vec<String> {
        let mut result = vec![];
        result.push(self.name.clone());
        self.sub_categories.iter().for_each(|child| {
            let mut sub_result = child.sub_category_names();
            result.append(&mut sub_result);
        });
        result
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

    pub fn add_sub_category_leaf(
        &mut self,
        sub_category_name: &str,
        category_name: &str,
    ) -> Result<()> {
        if self.name == category_name {
            self.sub_categories.push(Self::leaf(sub_category_name));
            Ok(())
        } else {
            let mut result: Result<()> = Ok(());
            for sub_category in self.sub_categories.iter_mut() {
                let sub_result =
                    sub_category.add_sub_category_leaf(sub_category_name, category_name);
                if sub_result.is_err() {
                    result = sub_result;
                    break;
                };
            }
            result
        }
    }

    pub fn find_parent_category(&self, target_name: &str) -> Option<SubCategory> {
        if self.name == target_name {
            None
        } else {
            let r = self.sub_categories.iter().try_for_each(|child| {
                if child.name == target_name {
                    return ControlFlow::Break(self.clone());
                };
                if let Some(grand_child) = child.find_parent_category(target_name) {
                    return ControlFlow::Break(grand_child.clone());
                };
                ControlFlow::Continue(())
            });
            if let ControlFlow::Break(child) = r {
                Some(child)
            } else {
                None
            }
        }
    }

    pub fn find_sub_category_by_name(&self, target_name: &str) -> Option<SubCategory> {
        if self.name == target_name {
            Some(self.clone())
        } else {
            let r = self.sub_categories.iter().try_for_each(|child| {
                if child.name == target_name {
                    return ControlFlow::Break(child.clone());
                };
                if let Some(grand_child) = child.find_sub_category_by_name(target_name) {
                    return ControlFlow::Break(grand_child.clone());
                };
                ControlFlow::Continue(())
            });
            if let ControlFlow::Break(child) = r {
                Some(child)
            } else {
                None
            }
        }
    }

    fn add_sub_category_tree_on_name(&mut self, sub_category_tree: &Self, category_name: &str) {
        if self.name == category_name {
            self.sub_categories.push(sub_category_tree.clone());
        } else {
            for sub_category in self.sub_categories.iter_mut() {
                let _ =
                    sub_category.add_sub_category_tree_on_name(sub_category_tree, category_name);
            }
        }
    }

    pub fn add_sub_category_tree(
        &mut self,
        sub_category_tree: &Self,
        category_name: &str,
    ) -> Result<()> {
        match self.find_sub_category_by_name(category_name) {
            Some(_) => {
                self.add_sub_category_tree_on_name(sub_category_tree, category_name);
                Ok(())
            }
            None => Err(Error::other(format!(
                "sub_category {} does not exist",
                category_name
            ))),
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
                Err(Error::other(format!(
                    "category: {} has subcategories and cannot be deleted",
                    sub_category_name
                )))
            } else {
                self.sub_categories.remove(index);
                Ok(())
            }
        } else {
            let mut result: Result<()> = Ok(());
            for sub_category in self.sub_categories.iter_mut() {
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
                        Ok(vec![Self::leaf(symbol)])
                    }
                    Cons(_) => {
                        // (foo • (… • …))
                        let mut subs = vec![Self::leaf(symbol)];
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
                        Ok(Self::leaf(symbol))
                    } else {
                        Self::from_cons(cdr).map(|subs| SubCategory {
                            name: symbol.to_string(),
                            sub_categories: subs,
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

    pub(crate) fn sort(&mut self) {
        for sub_category in self.sub_categories.iter_mut() {
            sub_category.sort()
        }
        self.sub_categories.sort_by_key(|k| k.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_sorted() {
        let mut foo = SubCategory::from_value(
            &lexpr::from_str("(foo (jug mix alf) (qux bag) (bar law))").unwrap(),
        )
        .unwrap();
        foo.sort();
        assert_eq!(
            "(foo\n  (bar\n      law)\n  (jug\n      alf\n      mix)\n  (qux\n      bag))",
            foo.format_at_level(0)
        );
    }
    #[test]
    fn can_remove_a_sub_category_and_downward() {
        let mut foo = SubCategory::from_value(
            &lexpr::from_str("(foo (jug mix alf) (qux bag) (bar law))").unwrap(),
        )
        .unwrap();
        foo.remove_sub_category("jug", true);
        assert_eq!(
            "(foo\n  (qux\n      bag)\n  (bar\n      law))",
            foo.format_at_level(0)
        );
    }
    #[test]
    fn can_add_a_sub_category_with_subs() {
        let mut foo =
            SubCategory::from_value(&lexpr::from_str("(foo (jug mix alf) (bar law))").unwrap())
                .unwrap();
        let qux = SubCategory::from_value(&lexpr::from_str("(qux bag)").unwrap()).unwrap();
        assert!(foo.add_sub_category_tree(&qux, "foo").is_ok());
        foo.sort();
        assert_eq!(
            "(foo\n  (bar\n      law)\n  (jug\n      alf\n      mix)\n  (qux\n      bag))",
            foo.format_at_level(0)
        );
    }
    #[test]
    fn cannot_add_a_sub_category_on_non_existent_name() {
        let mut foo =
            SubCategory::from_value(&lexpr::from_str("(foo (jug mix alf) (bar law))").unwrap())
                .unwrap();
        let qux = SubCategory::from_value(&lexpr::from_str("(qux bag)").unwrap()).unwrap();
        assert!(foo.add_sub_category_tree(&qux, "man").is_err());
    }

    #[test]
    fn can_find_a_sub_category_when_that_sub_is_itself() {
        let cat = SubCategory::from_value(&lexpr::from_str("(foo (bar law) (qux bag))").unwrap())
            .unwrap();
        let sub = cat.find_sub_category_by_name("foo");
        assert!(sub.is_some());
        let foo = sub.unwrap();
        assert_eq!("qux", foo.sub_categories()[1].name);
    }
    #[test]
    fn can_find_a_sub_category_when_that_sub_is_one_of_direct_subs() {
        let cat = SubCategory::from_value(&lexpr::from_str("(foo (bar law) (qux bag))").unwrap())
            .unwrap();
        let sub = cat.find_sub_category_by_name("bar");
        assert!(sub.is_some());
        let bar = sub.unwrap();
        assert_eq!("law", bar.sub_categories()[0].name);
    }
    #[test]
    fn can_find_a_sub_category_when_that_sub_is_one_of_indirect_subs() {
        let cat = SubCategory::from_value(
            &lexpr::from_str("(foo (bar law) (qux (gus (bro bag))))").unwrap(),
        )
        .unwrap();
        let sub = cat.find_sub_category_by_name("bag");
        assert!(sub.is_some());
        let bag = sub.unwrap();
        assert_eq!("bag", bag.name);
    }
    #[test]
    fn can_tell_if_a_sub_category_is_the_top_category() {
        let cat = SubCategory::from_value(
            &lexpr::from_str("(- (bar law) (qux (gus (bro bag))))").unwrap(),
        )
        .unwrap();
        assert!(cat.is_top_category());
    }
    #[test]
    fn can_find_a_sub_category_parent() {
        let cat = SubCategory::from_value(
            &lexpr::from_str("(- (bar law) (qux (gus (bro bag))))").unwrap(),
        )
        .unwrap();
        let bro = cat.find_parent_category("bag");
        assert!(bro.is_some());
        assert_eq!("bro", bro.unwrap().name);
    }
    #[test]
    fn can_tell_all_sub_category_names() {
        let cat = SubCategory::from_value(
            &lexpr::from_str("(- (bar law) (qux (gus (bro bag))))").unwrap(),
        )
        .unwrap();
        assert_eq!(
            vec!["-", "bar", "law", "qux", "gus", "bro", "bag"],
            cat.sub_category_names()
        )
    }
}
