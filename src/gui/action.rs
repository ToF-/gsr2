pub mod gio_action;
pub mod gio_action_parameter;
pub mod gio_action_parameter_type;
pub mod gio_action_type;
use crate::gui::control::Control;
use crate::gui::direction::Direction;
use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;
use crate::gui::mode::Mode;
use crate::model::category::Category;
use crate::model::find::Find;
use crate::model::label::Label;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::view_option::ViewOption;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    AddCategory(String, String),  // add a new sub category under a category
    AddTag(Label),                // add tag(s) to the selected pictures
    ApplyOrderSetting(Order),     // apply the order in which see the pictures
    ApplyViewSetting(ViewOption), // change the view setting
    Cancel,                       // cancel current interaction
    CancelSelectionRange,         // cancel selection
    Categorize(Category),         // change the category of the selected pictures
    ConfirmDeleteFile,            // input a yes to deleting selected picture files
    ConfirmMoveFile(String),      // input a yes to moving selected picture files
    Dismiss,                      // dismiss after presentation of an information
    EnterAddTag,                  // enter new tag(s) to add to the selected pictures
    EnterNewCategory,             // enter new category to add to the catalog
    EnterIndex,                   // interactively enter index to jump to
    EnterFind(Find),              // interactively enter criteria for finding
    EnterLabel,                   // enter label to apply to the selected pictures
    EnterRemoveTag,               // enter tag(s) to remove from the selected pictures
    EnterRename,                  // enter new name for selected picture
    Find(Find),                   // find the first picture matching the find criteria
    FindNext,                     // find the next picture matching the current criteria
    GotoDirectory,                // view only pictures from a sub directory
    JumpToIndex(usize),           // jump to picture #n
    JumpToMark(char),             // jump to picture marked a|b|…|z
    JumpToRandom,                 // jump to a random picture
    Label(Label),                 // label the selected pictures
    Mark(char),                   // set the mark a|b|…|z to the current picture
    MoveCategory(String, String), // move the sub category under a category
    MoveFile,                     // move selected picture files
    FocusAt(i32, i32),            // set the picture at col,row the current picture
    MoveTowards(Direction),       // move to a direction
    Nothing,                      // do nothing (test)
    PickCatalogChange,            // interactively select what catalog change to make
    PickChange,                   // interactively select what change to make
    PickOrderSetting,             // interactively select which order setting to apply
    PickViewOption,               // interactively select what vieww setting to apply
    Quit,                         // exit from gsr
    QuitDirectory,                // view all pictures not only sub directory
    Rank(Rank),                   // rank the selected pictures
    RemoveCategory(String),       // remove the sub category from the catalog
    RemoveTag(Label),             // remove tag(s) from the selected pictures
    Rename(String),               // rename the selected picture file
    RepeatAction,                 // redo the last action
    RepeatRangeSelection,         // redo the last selection
    SelectCategoryAddTarget(String), // select category to add another category to in the catalog
    SelectCategoryForPicture,     // select category to apply to the selected pictures
    SelectCategoryMoveTarget(String), // select category to move another category to in the catalog
    SelectCategoryToMove,         // select category to move in the catalog
    SelectCategoryToRemove,       // select category to remove from the catalog
    Select(Find),                 // view only pictures matching the find criteria
    SetSelectionAll,              // set the selection to all pictures
    SetSelectionPage,             // set the seleciton to all pictures in the page
    SetSelectionRangeEnd(usize),  // send the selection at the current picture
    SetSelectionRangeStart(usize), // start a selection from the current picture
    Test(String),                 // test action for development test
    ToggleCover,                  // toggle current picture set to cover or not
    ToggleCoversView,             // set the view on off to only covers
    TogglePalette,                // set the palette visible on / off
    ToggleSelected(usize),        // toggle current picture in or out the selection
    ToggleSelectedAt(i32, i32),   // set the picture at col,row selected or deselected
    ToggleSingleView,             // set the view to single / back to multiple
    ToggleSlideShow,              // set the slide show on off
    ToggleThumbnailsView,         // set the view to thumbnails / back to previous
    ToggleTwoByTwoView,           // set the view to 2x2 / back to previous
    Unlabel,                      // remove label from the selected pictures
    ViewCatalog,                  // show a list of all categories
}

impl Action {
    pub fn is_repeatable(&self) -> bool {
        match self {
            Action::AddTag(_) => true,
            Action::Categorize(_) => true,
            Action::Label(_) => true,
            Action::Rank(_) => true,
            Action::RemoveTag(_) => true,
            Action::Unlabel => true,
            _ => false,
        }
    }

    pub fn single_action_name(_key_name: &str, _mode: Mode) -> String {
        format!("{}.{}", MAIN_CONTROLLER_GROUP_NAME, "test")
    }

    // FOO
    pub fn from_control(control: &Control) -> Self {
        match control {
            Control::BackFromDirectory => Action::QuitDirectory,
            Control::CancelEdition => Action::Cancel,
            Control::Down => Action::MoveTowards(Direction::Down),
            Control::PickChange => Action::PickChange,
            Control::GotoDirectory => Action::GotoDirectory,
            Control::Left => Action::MoveTowards(Direction::Left),
            Control::MoveEndPage => Action::MoveTowards(Direction::PageEnd),
            Control::MoveFirst => Action::MoveTowards(Direction::First),
            Control::MoveLast => Action::MoveTowards(Direction::Last),
            Control::MoveNext => Action::MoveTowards(Direction::NextPage),
            Control::MovePrev => Action::MoveTowards(Direction::PrevPage),
            Control::MoveStartPage => Action::MoveTowards(Direction::PageStart),
            Control::Quit => Action::Quit,
            Control::RankNoStar => Action::Rank(Rank::NoStar),
            Control::RankOneStar => Action::Rank(Rank::OneStar),
            Control::RankThreeStars => Action::Rank(Rank::ThreeStars),
            Control::RankTwoStars => Action::Rank(Rank::TwoStars),
            Control::Right => Action::MoveTowards(Direction::Right),
            Control::SetOrder => Action::PickOrderSetting,
            Control::SetView => Action::PickViewOption,
            Control::ToggleFullSize => Action::ApplyViewSetting(ViewOption::FullSize),
            Control::TogglePalette => Action::TogglePalette,
            Control::ToggleSingleView => Action::ToggleSingleView,
            Control::ToggleThumbView => Action::ToggleThumbnailsView,
            Control::ToggleTwoByTwoView => Action::ToggleTwoByTwoView,
            Control::Up => Action::MoveTowards(Direction::Up),
            _ => Action::Nothing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::action::gio_action_parameter_type::GioActionParameterType;
    use crate::gui::action::gio_action_type::GioActionType;
    use crate::model::label::label_from;

    #[test]
    fn not_all_action_are_repeatable() {
        assert!(Action::Rank(Rank::ThreeStars).is_repeatable());
        assert!(Action::Label(label_from("gio_action_type")).is_repeatable());
        assert!(!Action::AddCategory("gio_action_type".into(), "bar".into()).is_repeatable());
    }

    #[test]
    fn gio_action_type_from_action() {
        let action = Action::PickChange;
        let gio_action_type = GioActionType::from(action);
        assert_eq!("pick-change", gio_action_type.name());
        assert_eq!(
            GioActionParameterType::None,
            gio_action_type.parameter_type()
        );
    }
}
