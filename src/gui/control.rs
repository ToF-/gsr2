use crate::gui::mode::Mode;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Control {
    ConfirmEdition,
    CancelEdition,
    Complete,
    DeleteChar,
    DeletePicture,
    SetRange,
    ToggleSelected,
    CancelRange,
    Jump,
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
    ToggleCover,
    ToggleExpand,
    ToggleInformation,
    ToggleFullSize,
    TogglePalette,
    ToggleSingleView,
    ToggleSlideShow,
    SetDisplay,
    SetOrder,
    SetSelection,
    SetRestriction,
    CancelSelection,
    Information,
    Find,
    Label,
    Unlabel,
    AddTag,
    RemoveTag,
    Quit,
    GridTwo,
    GridThree,
    GridFour,
    GridFive,
    GridTen,
    DisplayDate,
    DisplayFocus,
    DisplaySize,
    OrderByName,
    OrderByDate,
    OrderBySize,
    OrderByValue,
    OrderByLabel,
    OrderByColorCount,
    OrderByPalette,
    Randomize,
    RankNoStar,
    RankOneStar,
    RankTwoStars,
    RankThreeStars,
    RepeatLastAction,
}

pub type KeyAndMode = (String, Mode);
pub type Controls = HashMap<KeyAndMode, Control>;

// these default controls are valid on my ergodox bepo modified
pub fn default_controls() -> Controls {
    let controls: Controls = HashMap::from([
        (
            (String::from("Escape"), Mode::Editing),
            Control::CancelEdition,
        ),
        (
            (String::from("Return"), Mode::Editing),
            Control::ConfirmEdition,
        ),
        ((String::from("Tab"), Mode::Editing), Control::Complete),
        ((String::from("Return"), Mode::View), Control::SetRange),
        ((String::from("Escape"), Mode::View), Control::CancelRange),
        ((String::from("space"), Mode::View), Control::ToggleSelected),
        (
            (String::from("BackSpace"), Mode::Editing),
            Control::DeleteChar,
        ),
        ((String::from("i"), Mode::View), Control::ToggleInformation),
        ((String::from("I"), Mode::View), Control::Information),
        ((String::from("J"), Mode::View), Control::Jump),
        ((String::from("F"), Mode::View), Control::Find),
        ((String::from("n"), Mode::View), Control::MoveNext),
        ((String::from("p"), Mode::View), Control::MovePrev),
        ((String::from("Z"), Mode::View), Control::MoveLast),
        ((String::from("A"), Mode::View), Control::MoveFirst),
        ((String::from("a"), Mode::View), Control::MoveStartPage),
        ((String::from("z"), Mode::View), Control::MoveEndPage),
        ((String::from("c"), Mode::View), Control::ToggleCover),
        ((String::from("e"), Mode::View), Control::ToggleExpand),
        ((String::from("f"), Mode::View), Control::ToggleFullSize),
        ((String::from("P"), Mode::View), Control::TogglePalette),
        ((String::from("R"), Mode::View), Control::MoveRandom),
        ((String::from("X"), Mode::View), Control::DeletePicture),
        ((String::from("S"), Mode::View), Control::ToggleSlideShow),
        ((String::from("l"), Mode::View), Control::Label),
        ((String::from("L"), Mode::View), Control::Unlabel),
        ((String::from("asterisk"), Mode::View), Control::AddTag),
        ((String::from("slash"), Mode::View), Control::RemoveTag),
        ((String::from("q"), Mode::View), Control::Quit),
        ((String::from("Left"), Mode::View), Control::Left),
        ((String::from("t"), Mode::View), Control::Left),
        ((String::from("Right"), Mode::View), Control::Right),
        ((String::from("r"), Mode::View), Control::Right),
        ((String::from("Up"), Mode::View), Control::Up),
        ((String::from("d"), Mode::View), Control::Up),
        ((String::from("Down"), Mode::View), Control::Down),
        ((String::from("s"), Mode::View), Control::Down),
        ((String::from("equal"), Mode::View), Control::SetSelection),
        ((String::from("minus"), Mode::View), Control::SetRestriction),
        (
            (String::from("numbersign"), Mode::View),
            Control::CancelSelection,
        ),
        (
            (String::from("period"), Mode::View),
            Control::ToggleSingleView,
        ),
        ((String::from("b"), Mode::View), Control::GridTwo),
        ((String::from("egrave"), Mode::View), Control::GridTen),
        ((String::from("ccedilla"), Mode::View), Control::GridThree),
        ((String::from("eacute"), Mode::View), Control::GridFour),
        ((String::from("agrave"), Mode::View), Control::GridFive),
        ((String::from("D"), Mode::View), Control::SetDisplay),
        ((String::from("O"), Mode::View), Control::SetOrder),
        (
            (String::from("d"), Mode::Setting(Control::SetDisplay)),
            Control::DisplayDate,
        ),
        (
            (String::from("s"), Mode::Setting(Control::SetDisplay)),
            Control::DisplaySize,
        ),
        (
            (String::from("f"), Mode::Setting(Control::SetOrder)),
            Control::DisplayFocus,
        ),
        (
            (String::from("n"), Mode::Setting(Control::SetOrder)),
            Control::OrderByName,
        ),
        (
            (String::from("d"), Mode::Setting(Control::SetOrder)),
            Control::OrderByDate,
        ),
        (
            (String::from("s"), Mode::Setting(Control::SetOrder)),
            Control::OrderBySize,
        ),
        (
            (String::from("r"), Mode::Setting(Control::SetOrder)),
            Control::Randomize,
        ),
        (
            (String::from("v"), Mode::Setting(Control::SetOrder)),
            Control::OrderByValue,
        ),
        (
            (String::from("l"), Mode::Setting(Control::SetOrder)),
            Control::OrderByLabel,
        ),
        (
            (String::from("c"), Mode::Setting(Control::SetOrder)),
            Control::OrderByColorCount,
        ),
        (
            (String::from("p"), Mode::Setting(Control::SetOrder)),
            Control::OrderByPalette,
        ),
        ((String::from("0"), Mode::View), Control::RankNoStar),
        ((String::from("1"), Mode::View), Control::RankOneStar),
        ((String::from("2"), Mode::View), Control::RankTwoStars),
        ((String::from("3"), Mode::View), Control::RankThreeStars),
        ((String::from("Tab"), Mode::View), Control::RepeatLastAction),
    ]);
    controls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_key_name_and_mode_to_a_control() {
        const V: Mode = Mode::View;
        assert_eq!(
            Some(&Control::MoveNext),
            default_controls().get(&(String::from("n"), V))
        );
        assert_eq!(
            Some(&Control::MovePrev),
            default_controls().get(&(String::from("p"), V))
        );
        assert_eq!(
            Some(&Control::MoveLast),
            default_controls().get(&(String::from("Z"), V))
        );
        assert_eq!(
            Some(&Control::MoveFirst),
            default_controls().get(&(String::from("A"), V))
        );
        assert_eq!(
            Some(&Control::MoveStartPage),
            default_controls().get(&(String::from("a"), V))
        );
        assert_eq!(
            Some(&Control::MoveEndPage),
            default_controls().get(&(String::from("z"), V))
        );
        assert_eq!(
            Some(&Control::Left),
            default_controls().get(&(String::from("Left"), V))
        );
        assert_eq!(
            Some(&Control::Right),
            default_controls().get(&(String::from("Right"), V))
        );
        assert_eq!(
            Some(&Control::Up),
            default_controls().get(&(String::from("Up"), V))
        );
        assert_eq!(
            Some(&Control::Down),
            default_controls().get(&(String::from("Down"), V))
        );
        assert_eq!(
            Some(&Control::Quit),
            default_controls().get(&(String::from("q"), V))
        );
        assert_eq!(
            Some(&Control::Label),
            default_controls().get(&(String::from("l"), V))
        );
        assert_eq!(
            Some(&Control::ToggleExpand),
            default_controls().get(&(String::from("e"), V))
        );
        assert_eq!(
            Some(&Control::ToggleFullSize),
            default_controls().get(&(String::from("f"), V))
        );
        assert_eq!(
            Some(&Control::ToggleSingleView),
            default_controls().get(&(String::from("period"), V))
        );
        assert_eq!(
            Some(&Control::CancelEdition),
            default_controls().get(&(String::from("Escape"), Mode::Editing))
        );
        assert_eq!(
            Some(&Control::ConfirmEdition),
            default_controls().get(&(String::from("Return"), Mode::Editing))
        );
        assert_eq!(
            Some(&Control::SetRange),
            default_controls().get(&(String::from("Return"), Mode::View))
        );
    }
}
