use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Control {
    ToggleExpand,
    ToggleFullSize,
    TogglePalette,
    Quit,
}

pub type Controls = HashMap<String, Control>;

pub fn default_controls() -> Controls {
    let controls: HashMap<String, Control> = HashMap::from([
        (String::from("e"), Control::ToggleExpand),
        (String::from("f"), Control::ToggleFullSize),
        (String::from("x"), Control::TogglePalette),
        (String::from("q"), Control::Quit),
    ]);
    controls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_key_name_to_a_control() {
        assert_eq!(Some(&Control::TogglePalette), default_controls().get("x"));
        assert_eq!(Some(&Control::ToggleExpand), default_controls().get("e"));
        assert_eq!(Some(&Control::ToggleFullSize), default_controls().get("f"));
    }
}
