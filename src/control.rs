use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Control {
    ToggleExpand,
    TogglePalette,
}

type Controls = HashMap<String, Control>;

pub fn default_controls() -> Controls {
    let controls: HashMap<String, Control> = HashMap::from([
        (String::from("x"), Control::TogglePalette),
        (String::from("e"), Control::ToggleExpand),
    ]);
    controls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_key_name_to_a_control() {
        assert_eq!(Some(&Control::TogglePalette), default_controls().get("x"));
        assert_eq!(Some(&Control::ToggleExpand), default_controls().get("e"))
    }
}
