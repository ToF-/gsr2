use crate::gui::mode::Mode;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Control {
    AddTag,
    BackFromDirectory,
    CancelEdition,
    CancelRange,
    CancelSelection,
    Categorize,
    Complete,
    ConfirmEdition,
    ConfirmSelection,
    CopyTemp,
    DeleteChar,
    DeletePicture,
    DisplayDate,
    DisplayFocus,
    DisplaySize,
    Down,
    PickChange,
    EnterFind,
    EnterSelect,
    SetView,
    EnterRank,
    ExtractFileNames,
    FindName,
    FindNext,
    GotoDirectory,
    GotoMark,
    Help,
    Information,
    Jump,
    JumpMarkChar(char),
    Label,
    Left,
    MoveEndPage,
    MoveFirst,
    MoveLast,
    MoveNext,
    MovePicture,
    MovePrev,
    MoveRandom,
    MoveStartPage,
    OrderByCategory,
    OrderByColorCount,
    OrderByCover,
    OrderByDate,
    OrderByLabel,
    OrderByName,
    OrderByPalette,
    OrderByScore,
    OrderBySize,
    OrderByValue,
    PickMark,
    Quit,
    Randomize,
    RankNoStar,
    RankOneStar,
    RankThreeStars,
    RankTwoStars,
    RemoveTag,
    RedoFind,
    Rename,
    RepeatLastAction,
    RepeatRange,
    Right,
    SelectCategory,
    SetDisplay,
    SetMark,
    SetMarkChar(char),
    SetOrder,
    SetSelectionRangeEnd,
    SetSelectionRangeAll,
    SetSelectionRangePage,
    SetRank,
    Test,
    ToggleCover,
    ToggleCoverSelection,
    ToggleExpand,
    ToggleFullSize,
    DisplayPath,
    ToggleBlinking,
    TogglePalette,
    TogglePicturesPerRow(i32),
    ToggleSelected,
    ToggleTwoByTwoView,
    ToggleSingleView,
    ToggleThumbView,
    ToggleSlideShow,
    Uncategorize,
    Unlabel,
    Up,
}

pub type KeyAndMode = (String, Mode);
pub type Controls = HashMap<KeyAndMode, Control>;

pub fn help_on_controls() -> String {
    format!(
        "{}\n",
        "n: next page  p: prev. page \n\
        a: beginning z: end of page \n\
        A: first picture Z: last picture\n\
        return: set range start/end\n\
        space: toggle in/out of range \n\
        esc: cancel range  $: repeat range\n\
        !: whole range _: page range\n\
        F: find…  N: find next /: redo find\n\
        S: select…\n\
        J: jump to picture #…\n\
        k then a,b,c,d,e : set mark A/B/C/D/E\n\
        j then a,b,c,d,e : jump to mark A/B/C/D/E\n\
        c change…\n\
        0,1,2,3: set rank\n\
        o order by…\n\
        v view option…\n\
        C: view covers only\n\
        P: palette on/off\n\
        . or ^ : single view, e: expand, %: full size \n\
        R go to random picture  S: resume slide show\n\
        M: move selected pictures to target dir set by label\n\
        X: delete selected pictures\n\
        g: view this cover subgroup, or directory\n\
        q: back from subgroup or directory\n\
        q: quit  H:help"
    )
}
// these default controls are valid on my ergodox bepo modified
pub fn default_controls() -> Controls {
    let mut controls: Controls = HashMap::from([
        ((String::from("colon"), Mode::View), Control::SelectCategory),
        ((String::from("question"), Mode::View), Control::CopyTemp),
        (
            (String::from("Escape"), Mode::Editing),
            Control::CancelEdition,
        ),
        (
            (String::from("Return"), Mode::Editing),
            Control::ConfirmEdition,
        ),
        (
            (String::from("Escape"), Mode::Categorizing),
            Control::CancelSelection,
        ),
        (
            (String::from("Return"), Mode::Categorizing),
            Control::ConfirmSelection,
        ),
        ((String::from("Tab"), Mode::Editing), Control::Complete),
        (
            (String::from("Return"), Mode::View),
            Control::SetSelectionRangeEnd,
        ),
        ((String::from("ampersand"), Mode::View), Control::Test),
        (
            (String::from("exclam"), Mode::View),
            Control::SetSelectionRangeAll,
        ),
        (
            (String::from("underscore"), Mode::View),
            Control::SetSelectionRangePage,
        ),
        ((String::from("Escape"), Mode::View), Control::CancelRange),
        ((String::from("dollar"), Mode::View), Control::RepeatRange),
        ((String::from("space"), Mode::View), Control::ToggleSelected),
        (
            (String::from("BackSpace"), Mode::Editing),
            Control::DeleteChar,
        ),
        ((String::from("E"), Mode::View), Control::ExtractFileNames),
        ((String::from("J"), Mode::View), Control::Jump),
        ((String::from("k"), Mode::View), Control::SetMark),
        ((String::from("j"), Mode::View), Control::GotoMark),
        ((String::from("H"), Mode::View), Control::Help),
        ((String::from("slash"), Mode::View), Control::RedoFind),
        ((String::from("S"), Mode::View), Control::EnterSelect),
        ((String::from("F"), Mode::View), Control::EnterFind),
        ((String::from("N"), Mode::View), Control::FindNext),
        ((String::from("n"), Mode::View), Control::MoveNext),
        ((String::from("p"), Mode::View), Control::MovePrev),
        ((String::from("Z"), Mode::View), Control::MoveLast),
        ((String::from("A"), Mode::View), Control::MoveFirst),
        ((String::from("a"), Mode::View), Control::MoveStartPage),
        ((String::from("z"), Mode::View), Control::MoveEndPage),
        ((String::from("e"), Mode::View), Control::ToggleExpand),
        (
            (String::from("percent"), Mode::View),
            Control::ToggleFullSize,
        ),
        (
            (String::from("C"), Mode::View),
            Control::ToggleCoverSelection,
        ),
        ((String::from("P"), Mode::View), Control::TogglePalette),
        ((String::from("B"), Mode::View), Control::ToggleBlinking),
        ((String::from("R"), Mode::View), Control::MoveRandom),
        ((String::from("X"), Mode::View), Control::DeletePicture),
        ((String::from("asterisk"), Mode::View), Control::ToggleSlideShow),
        ((String::from("c"), Mode::View), Control::PickChange),
        ((String::from("Q"), Mode::View), Control::Quit),
        ((String::from("T"), Mode::View), Control::ToggleThumbView),
        ((String::from("W"), Mode::View), Control::ToggleTwoByTwoView),
        ((String::from("Left"), Mode::View), Control::Left),
        ((String::from("t"), Mode::View), Control::Left),
        ((String::from("Right"), Mode::View), Control::Right),
        ((String::from("r"), Mode::View), Control::Right),
        ((String::from("Up"), Mode::View), Control::Up),
        ((String::from("d"), Mode::View), Control::Up),
        ((String::from("Down"), Mode::View), Control::Down),
        ((String::from("s"), Mode::View), Control::Down),
        (
            (String::from("numbersign"), Mode::View),
            Control::CancelSelection,
        ),
        (
            (String::from("period"), Mode::View),
            Control::ToggleSingleView,
        ),
        ((String::from("h"), Mode::View), Control::ToggleSingleView),
        (
            (String::from("dead_circumflex"), Mode::View),
            Control::ToggleSingleView,
        ),
        ((String::from("M"), Mode::View), Control::MovePicture),
        ((String::from("m"), Mode::View), Control::PickMark),
        ((String::from("v"), Mode::View), Control::SetView),
        ((String::from("4"), Mode::View), Control::EnterRank),
        ((String::from("o"), Mode::View), Control::SetOrder),
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
            (String::from("a"), Mode::Setting(Control::SetOrder)),
            Control::OrderByCategory,
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
            (String::from("m"), Mode::Setting(Control::SetOrder)),
            Control::OrderByScore,
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
        ((String::from("4"), Mode::View), Control::EnterRank),
        ((String::from("0"), Mode::View), Control::RankNoStar),
        ((String::from("1"), Mode::View), Control::RankOneStar),
        ((String::from("2"), Mode::View), Control::RankTwoStars),
        ((String::from("3"), Mode::View), Control::RankThreeStars),
        ((String::from("Tab"), Mode::View), Control::RepeatLastAction),
        ((String::from("g"), Mode::View), Control::GotoDirectory),
        ((String::from("G"), Mode::View), Control::BackFromDirectory),
        ((String::from("q"), Mode::View), Control::BackFromDirectory),
    ]);
    for ch in 'a'..='z' {
        controls.insert(
            (ch.to_string(), Mode::Setting(Control::SetMark)),
            Control::SetMarkChar(ch),
        );
        controls.insert(
            (ch.to_string(), Mode::Setting(Control::GotoMark)),
            Control::JumpMarkChar(ch),
        );
    }
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
            default_controls().get(&(String::from("Q"), V))
        );
        assert_eq!(
            Some(&Control::ToggleExpand),
            default_controls().get(&(String::from("e"), V))
        );
        assert_eq!(
            Some(&Control::ToggleFullSize),
            default_controls().get(&(String::from("percent"), V))
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
            Some(&Control::SetSelectionRangeEnd),
            default_controls().get(&(String::from("Return"), Mode::View))
        );
    }
}
