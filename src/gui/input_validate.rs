use std::rc::Rc;
#[derive(Clone)]
pub struct InputValidate {
    validator: Rc<dyn Fn(&str) -> Option<String>>,
}

impl InputValidate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(self, s: &str) -> Option<String> {
        self.validator.v
    }
}
impl Default for InputValidate {
    fn default() -> Self {
        Self {
            validator: Rc::new(|s: &str| Some(s.to_string())),
        }
    }
}
