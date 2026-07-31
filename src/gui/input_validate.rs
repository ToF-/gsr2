pub struct InputValidate {
    validator: Box<dyn Fn(&str) -> Option<String>>,
}

impl InputValidate {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for InputValidate {
    fn default() -> Self {
        Self {
            validator: Box::new(|s| { Some(s.to_string()) }),
        }
    }
}
