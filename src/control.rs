use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Control {
    Left,
    Right,
    Up,
    Down,
    MoveNext,
    MovePrev,
    MoveLast,
    MoveFirst,
    MoveEndPage,
    MoveStartPage,
    ToggleExpand,
    ToggleFullSize,
    TogglePalette,
    ToggleSingleView,
    Quit,
}

pub type Controls = HashMap<String, Control>;

pub fn default_controls() -> Controls {
    let controls: HashMap<String, Control> = HashMap::from([
        (String::from("n"), Control::MoveNext),
        (String::from("p"), Control::MovePrev),
        (String::from("Z"), Control::MoveLast),
        (String::from("A"), Control::MoveFirst),
        (String::from("a"), Control::MoveStartPage),
        (String::from("z"), Control::MoveEndPage),
        (String::from("e"), Control::ToggleExpand),
        (String::from("f"), Control::ToggleFullSize),
        (String::from("x"), Control::TogglePalette),
        (String::from("q"), Control::Quit),
        (String::from("Left"), Control::Left),
        (String::from("Right"), Control::Right),
        (String::from("Up"), Control::Up),
        (String::from("Down"), Control::Down),
        (String::from("period"), Control::ToggleSingleView),
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
