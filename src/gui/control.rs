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
    MoveRandom,
    ToggleExpand,
    ToggleFullSize,
    TogglePalette,
    ToggleSingleView,
    ToggleSlideShow,
    Label,
    Quit,
    GridTwo,
    GridThree,
    GridFour,
    GridFive,
    GridTen,
}

pub type Controls = HashMap<String, Control>;

// these default controls are valid on my ergodox bepo modified
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
        (String::from("R"), Control::MoveRandom),
        (String::from("S"), Control::ToggleSlideShow),
        (String::from("l"), Control::Label),
        (String::from("q"), Control::Quit),
        (String::from("Left"), Control::Left),
        (String::from("t"), Control::Left),
        (String::from("Right"), Control::Right),
        (String::from("r"), Control::Right),
        (String::from("Up"), Control::Up),
        (String::from("d"), Control::Up),
        (String::from("Down"), Control::Down),
        (String::from("s"), Control::Down),
        (String::from("period"), Control::ToggleSingleView),
        (String::from("b"), Control::GridTwo),
        (String::from("egrave"), Control::GridTen),
        (String::from("ccedilla"), Control::GridThree),
        (String::from("eacute"), Control::GridFour),
        (String::from("agrave"), Control::GridFive),
    ]);
    controls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_key_name_to_a_control() {
        assert_eq!(Some(&Control::MoveNext), default_controls().get("n"));
        assert_eq!(Some(&Control::MovePrev), default_controls().get("p"));
        assert_eq!(Some(&Control::MoveLast), default_controls().get("Z"));
        assert_eq!(Some(&Control::MoveFirst), default_controls().get("A"));
        assert_eq!(Some(&Control::MoveStartPage), default_controls().get("a"));
        assert_eq!(Some(&Control::MoveEndPage), default_controls().get("z"));
        assert_eq!(Some(&Control::Left), default_controls().get("Left"));
        assert_eq!(Some(&Control::Right), default_controls().get("Right"));
        assert_eq!(Some(&Control::Up), default_controls().get("Up"));
        assert_eq!(Some(&Control::Down), default_controls().get("Down"));
        assert_eq!(Some(&Control::Quit), default_controls().get("q"));
        assert_eq!(Some(&Control::Label), default_controls().get("l"));
        assert_eq!(Some(&Control::TogglePalette), default_controls().get("x"));
        assert_eq!(Some(&Control::ToggleExpand), default_controls().get("e"));
        assert_eq!(Some(&Control::ToggleFullSize), default_controls().get("f"));
        assert_eq!(
            Some(&Control::ToggleSingleView),
            default_controls().get("period")
        );
    }
}
