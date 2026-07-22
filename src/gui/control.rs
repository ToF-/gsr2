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
    EnterFind,
    EnterGridSize,
    EnterRank,
    ExtractFileNames,
    FindFirstInLabel,
    FindFirstInName,
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
    MovePictureToLabel,
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
    Quit,
    Randomize,
    RankNoStar,
    RankOneStar,
    RankThreeStars,
    RankTwoStars,
    RemoveTag,
    Rename,
    RepeatLastAction,
    RepeatRange,
    Right,
    SelectCategory,
    SetDisplay,
    SetMark,
    SetMarkChar(char),
    SetOrder,
    SetRange,
    SetRangeAll,
    SetRangePage,
    SetRank,
    SetRestriction,
    SetSelection,
    ToggleCover,
    ToggleCoverSelection,
    ToggleExpand,
    ToggleFullSize,
    ToggleInformation,
    TogglePalette,
    ToggleSelected,
    ToggleSingleView,
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
        "n/p z/a Z/A: next/prev page, end/start of page, last/first page \n\
        return: set range start/end,  space: toggle in/out of range \n\
        esc $ ! _: cancel range, repeat range, whole range, page range\n\
        f/F: find by pattern in name/label \n\
        J: jump to picture #… \n\
        k then a,b,c,d,e : set mark A/B/C/D/E \n\
        j then a,b,c,d,e : jump to mark A/B/C/D/E \n
        \",«,»,(,) : jump to mark A/B/C/D/E \n\
        i/I: toggle information display, display file path \n\
        O then a,c,d,n,p,r,s,v: pick view order \n\
        D then s,t: display size,modified time in title \n\
        v: set/unset cover, V: see all covers \n\
        P: display palette sample \n\
        . or ^ : single view, e: expand, %: full size \n\
        R go to random picture  S: resume slide show\n\
        X/M delete or move (selected) picture(s)\n\
        l/L : label/unlabel picture\n\
        N : rename picture\n\
        *: add tag, /: remove tag\n\
        =/-/#: select pictures having some/all tags, cancel selection\n\
        y: set grid size of 1,4,9,16,25 or 100 pictures per page\n\
        0,1,2,3: set rank, 4: enter rank\n\
        g: view this cover subgroup, G/q: back from subgroup\n\
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
        ((String::from("Return"), Mode::View), Control::SetRange),
        ((String::from("exclam"), Mode::View), Control::SetRangeAll),
        (
            (String::from("nobreakspace"), Mode::View),
            Control::SetRangePage,
        ),
        ((String::from("Escape"), Mode::View), Control::CancelRange),
        ((String::from("dollar"), Mode::View), Control::RepeatRange),
        ((String::from("space"), Mode::View), Control::ToggleSelected),
        (
            (String::from("BackSpace"), Mode::Editing),
            Control::DeleteChar,
        ),
        ((String::from("i"), Mode::View), Control::ToggleInformation),
        ((String::from("I"), Mode::View), Control::Information),
        ((String::from("E"), Mode::View), Control::ExtractFileNames),
        ((String::from("J"), Mode::View), Control::Jump),
        ((String::from("k"), Mode::View), Control::SetMark),
        ((String::from("j"), Mode::View), Control::GotoMark),
        ((String::from("H"), Mode::View), Control::Help),
        ((String::from("f"), Mode::View), Control::EnterFind),
        ((String::from("F"), Mode::View), Control::FindNext),
        ((String::from("n"), Mode::View), Control::MoveNext),
        ((String::from("p"), Mode::View), Control::MovePrev),
        ((String::from("Z"), Mode::View), Control::MoveLast),
        ((String::from("A"), Mode::View), Control::MoveFirst),
        ((String::from("a"), Mode::View), Control::MoveStartPage),
        ((String::from("z"), Mode::View), Control::MoveEndPage),
        ((String::from("v"), Mode::View), Control::ToggleCover),
        (
            (String::from("V"), Mode::View),
            Control::ToggleCoverSelection,
        ),
        ((String::from("e"), Mode::View), Control::ToggleExpand),
        (
            (String::from("percent"), Mode::View),
            Control::ToggleFullSize,
        ),
        ((String::from("P"), Mode::View), Control::TogglePalette),
        ((String::from("R"), Mode::View), Control::MoveRandom),
        ((String::from("X"), Mode::View), Control::DeletePicture),
        ((String::from("S"), Mode::View), Control::ToggleSlideShow),
        ((String::from("l"), Mode::View), Control::Label),
        ((String::from("c"), Mode::View), Control::Categorize),
        ((String::from("L"), Mode::View), Control::Unlabel),
        ((String::from("C"), Mode::View), Control::Uncategorize),
        ((String::from("N"), Mode::View), Control::Rename),
        ((String::from("asterisk"), Mode::View), Control::AddTag),
        ((String::from("slash"), Mode::View), Control::RemoveTag),
        ((String::from("Q"), Mode::View), Control::Quit),
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
        ((String::from("h"), Mode::View), Control::ToggleSingleView),
        (
            (String::from("dead_circumflex"), Mode::View),
            Control::ToggleSingleView,
        ),
        ((String::from("M"), Mode::View), Control::MovePicture),
        ((String::from("m"), Mode::View), Control::MovePictureToLabel),
        ((String::from("y"), Mode::View), Control::EnterGridSize),
        ((String::from("4"), Mode::View), Control::EnterRank),
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
        ((String::from("4"), Mode::View), Control::SetRank),
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
            Some(&Control::Label),
            default_controls().get(&(String::from("l"), V))
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
            Some(&Control::SetRange),
            default_controls().get(&(String::from("Return"), Mode::View))
        );
    }
}
