use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Control {
    TogglePalette,
}

type Controls = HashMap<String, Control>;

pub fn default_controls() -> Controls {
    let controls: HashMap<String, Control> =
        HashMap::from([(String::from("x"), Control::TogglePalette)]);
    controls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_key_name_to_a_control() {
        assert_eq!(Some(&Control::TogglePalette), default_controls().get("x"))
    }
}
