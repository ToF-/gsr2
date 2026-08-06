use crate::gui::main_controller_action::ActionParameterType;
use crate::gui::main_controller_action::MainControllerAction;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;
use crate::gui::direction::Direction;
use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;
use crate::gui::mode::Mode;
use crate::model::category::Category;
use crate::model::change::Change;
use crate::model::find::Find;
use crate::model::label::Label;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::view_option::ViewOption;

#[derive(Debug, Clone)]
pub enum Action {
    AddCategory(String, String),   // add a new sub category under a category
    AddTag(Label),                 // add tag(s) to the selected pictures
    ApplyOrderSetting(Order),      // apply the order in which see the pictures
    ApplyViewSetting(ViewOption),  // change the view setting
    CancelSelectionRange,          // cancel selection
    Categorize(Category),          // change the category of the selected pictures
    ConfirmDeleteFile,             // input a yes to deleting selected picture files
    ConfirmMoveFile(String),       // input a yes to moving selected picture files
    EnterAddTag,                   // enter new tag(s) to add to the selected pictures
    EnterCategory,                 // enter category to apply to the selected pictures
    EnterChange(Change), // launch a specific change interaction
    EnterIndex,          // interactively enter index to jump to
    EnterLabel,          // enter label to apply to the selected pictures
    EnterRemoveTag,      // enter tag(s) to remove from the selected pictures
    EnterRename,         // enter new name for selected picture
    Find(Find),          // find the first picture matching the find criteria
    FindNext,            // find the next picture matching the current criteria
    GotoDirectory(String), // view only pictures from a sub directory
    JumpToIndex(usize),  // jump to picture #n
    JumpToMark(char),    // jump to picture marked a|b|…|z
    JumpToRandom,        // jump to a random picture
    Label(Label),        // label the selected pictures
    Mark(char),          // set the mark a|b|…|z to the current picture
    MoveCategory(String, String), // move the sub category under a category
    MoveFile,            // move selected picture files
    MoveTowards(Direction), // move to a direction
    Nothing,             // do nothing (test)
    PickChange,          // interactively select what change to make
    PickOrderSetting,    // interactively select which order setting to apply
    PickViewOption,      // interactively select what vieww setting to apply
    Quit,                // exit from gsr
    QuitDirectory,       // view all pictures not only sub directory
    Rank(Rank),          // rank the selected pictures
    RemoveCategory(String), // remove the sub category from the catalog
    RemoveTag(Label),    // remove tag(s) from the selected pictures
    Rename(String),      // rename the selected picture file
    RepeatAction,        // redo the last action
    RepeatRangeSelection, // redo the last selection
    Select(Find),        // view only pictures matching the find criteria
    SetSelectionAll,     // set the selection to all pictures
    SetSelectionPage,    // set the seleciton to all pictures in the page
    SetSelectionRangeEnd(usize), // send the selection at the current picture
    SetSelectionRangeStart(usize), // start a selection from the current picture
    Test(String),        // test action for development test
    ToggleCover,         // toggle current picture set to cover or not
    ToggleCoversView,    // set the view on off to only covers
    ToggleSelected(usize), // toggle current picture in or out the selection
    ToggleSingleView,    // set the view to single / back to multiple
    ToggleSlideShow,     // set the slide show on off
    ToggleThumbnailsView, // set the view to thumbnails / back to previous
    Unlabel,             // remove label from the selected pictures
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

    pub fn main_controller_action(&self) -> MainControllerAction {
        match self {
            Action::EnterChange(Change::Undefined) => MainControllerAction::new("foo", ActionParameterType::None),
            _ => MainControllerAction::new("test", ActionParameterType::None),
        }
    }
    pub fn single_action_name(key_name: &str, mode: Mode) -> String {
        format!("{}.{}", MAIN_CONTROLLER_GROUP_NAME, "test")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::label::label_from;

    #[test]
    fn not_all_action_are_repeatable() {
        assert!(Action::Rank(Rank::ThreeStars).is_repeatable());
        assert!(Action::Label(label_from("foo")).is_repeatable());
        assert!(!Action::AddCategory("foo".into(), "bar".into()).is_repeatable());
    }

    #[test]
    fn main_controller_action_from_action() {
        let action = Action::EnterChange(Change::Undefined);
        let mca = action.main_controller_action();
        assert_eq!("main-controller.enter-change-undefined", mca.name());
        assert_eq!(ActionParameterType::None, mca.parameter_type());
    }
}
