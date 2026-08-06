use crate::gui::control::Control;
use crate::gui::direction::Direction;
use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;
use crate::gui::gio_action::GioActionParameterType;
use crate::gui::gio_action::GioAction;
use crate::gui::mode::Mode;
use crate::model::category::Category;
use crate::model::change::Change;
use crate::model::find::Find;
use crate::model::label::Label;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::view_option::ViewOption;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
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
    EnterIndex,                    // interactively enter index to jump to
    EnterLabel,                    // enter label to apply to the selected pictures
    EnterRemoveTag,                // enter tag(s) to remove from the selected pictures
    EnterRename,                   // enter new name for selected picture
    Find(Find),                    // find the first picture matching the find criteria
    FindNext,                      // find the next picture matching the current criteria
    GotoDirectory(String),         // view only pictures from a sub directory
    JumpToIndex(usize),            // jump to picture #n
    JumpToMark(char),              // jump to picture marked a|b|…|z
    JumpToRandom,                  // jump to a random picture
    Label(Label),                  // label the selected pictures
    Mark(char),                    // set the mark a|b|…|z to the current picture
    MoveCategory(String, String),  // move the sub category under a category
    MoveFile,                      // move selected picture files
    MoveTowards(Direction),        // move to a direction
    Nothing,                       // do nothing (test)
    PickChange,                    // interactively select what change to make
    PickOrderSetting,              // interactively select which order setting to apply
    PickViewOption,                // interactively select what vieww setting to apply
    Quit,                          // exit from gsr
    QuitDirectory,                 // view all pictures not only sub directory
    Rank(Rank),                    // rank the selected pictures
    RemoveCategory(String),        // remove the sub category from the catalog
    RemoveTag(Label),              // remove tag(s) from the selected pictures
    Rename(String),                // rename the selected picture file
    RepeatAction,                  // redo the last action
    RepeatRangeSelection,          // redo the last selection
    Select(Find),                  // view only pictures matching the find criteria
    SetSelectionAll,               // set the selection to all pictures
    SetSelectionPage,              // set the seleciton to all pictures in the page
    SetSelectionRangeEnd(usize),   // send the selection at the current picture
    SetSelectionRangeStart(usize), // start a selection from the current picture
    Test(String),                  // test action for development test
    ToggleCover,                   // toggle current picture set to cover or not
    ToggleCoversView,              // set the view on off to only covers
    ToggleSelected(usize),         // toggle current picture in or out the selection
    ToggleSingleView,              // set the view to single / back to multiple
    ToggleSlideShow,               // set the slide show on off
    ToggleThumbnailsView,          // set the view to thumbnails / back to previous
    Unlabel,                       // remove label from the selected pictures
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

    pub fn gio_action(&self) -> GioAction {
        match self {
            Action::AddCategory(_, _) => GioAction::new("add-category", GioActionParameterType::StringPair),
            Action::AddTag(_) => GioAction::new("add-tag", GioActionParameterType::String),
            Action::ApplyOrderSetting(_) => GioAction::new("apply-order-setting", GioActionParameterType::String),
            Action::ApplyViewSetting(_) => GioAction::new("apply-view-setting", GioActionParameterType::String),
            Action::CancelSelectionRange => GioAction::new("cancel-selection-range", GioActionParameterType::None),
            Action::Categorize(_) => GioAction::new("categorize", GioActionParameterType::String),
            Action::ConfirmDeleteFile => GioAction::new("confirm-delete-file", GioActionParameterType::None),
            Action::ConfirmMoveFile(_) => GioAction::new("confirm-move-file", GioActionParameterType::String),
            Action::EnterAddTag => GioAction::new("enter-add-tag", GioActionParameterType::None),
            Action::EnterCategory => GioAction::new("enter-add-category", GioActionParameterType::None),
            Action::EnterIndex => GioAction::new("enter-index", GioActionParameterType::None),
            Action::EnterLabel => GioAction::new("enter-label", GioActionParameterType::None),
            Action::EnterRemoveTag => GioAction::new("enter-remove-tag", GioActionParameterType::None),
            Action::EnterRename => GioAction::new("enter-rename", GioActionParameterType::None),
            Action::Find(_) => GioAction::new("find", GioActionParameterType::String),
            Action::FindNext => GioAction::new("find-next", GioActionParameterType::None),
            Action::GotoDirectory(_) => GioAction::new("go-to-directory", GioActionParameterType::String),
            Action::JumpToIndex(_) => GioAction::new("jump-to-index", GioActionParameterType::Int32),
            Action::JumpToMark(_) => GioAction::new("jump-to-mark", GioActionParameterType::Char),
            Action::JumpToRandom => GioAction::new("jump-to-random", GioActionParameterType::None),
            Action::Label(_) => GioAction::new("label", GioActionParameterType::String),
            Action::Mark(_) => GioAction::new("mark", GioActionParameterType::Char),
            Action::MoveCategory(_,_) => GioAction::new("move-category", GioActionParameterType::StringPair),
            Action::MoveFile => GioAction::new("MoveFile", GioActionParameterType::None),
            Action::MoveTowards(_) => GioAction::new("move-towards", GioActionParameterType::String),
            Action::Nothing => GioAction::new("nothing", GioActionParameterType::None),
            Action::PickChange => GioAction::new("pick-change", GioActionParameterType::None),
            Action::PickOrderSetting => GioAction::new("pick-order-setting", GioActionParameterType::None),
            Action::PickViewOption => GioAction::new("pick-view-option", GioActionParameterType::None),
            Action::Quit => GioAction::new("quit", GioActionParameterType::None),
            Action::QuitDirectory => GioAction::new("quit-directory", GioActionParameterType::None),
            Action::Rank(_) => GioAction::new("rank", GioActionParameterType::Int32),
            Action::RemoveCategory(_) => GioAction::new("remove-category", GioActionParameterType::String),
            Action::RemoveTag(_) => GioAction::new("remove-tag", GioActionParameterType::String),
            Action::Rename(_) => GioAction::new("rename", GioActionParameterType::String),
            Action::RepeatAction => GioAction::new("repeat-action", GioActionParameterType::None),
            Action::RepeatRangeSelection => GioAction::new("repeat-range-selection", GioActionParameterType::None),
            Action::Select(_) => GioAction::new("select", GioActionParameterType::String),
            Action::SetSelectionAll => GioAction::new("set-selection-all", GioActionParameterType::None),
            Action::SetSelectionPage => GioAction::new("set-selection-page", GioActionParameterType::None),
            Action::SetSelectionRangeEnd(_) => GioAction::new("set-selection-range-end", GioActionParameterType::Int32),
            Action::SetSelectionRangeStart(_) => GioAction::new("set-selection-range-start", GioActionParameterType::Int32),
            Action::Test(_) => GioAction::new("test", GioActionParameterType::String),
            Action::ToggleCover => GioAction::new("toggle-cover", GioActionParameterType::None),
            Action::ToggleCoversView => GioAction::new("toggle-cover-view", GioActionParameterType::None),
            Action::ToggleSelected(_) => GioAction::new("toggle-selected", GioActionParameterType::None),
            Action::ToggleSingleView => GioAction::new("toggle-single-view", GioActionParameterType::None),
            Action::ToggleSlideShow => GioAction::new("toggle-single-view", GioActionParameterType::None),
            Action::ToggleThumbnailsView => GioAction::new("toggle-thumbnain-view", GioActionParameterType::None),
            Action::Unlabel => GioAction::new("unlabel", GioActionParameterType::None),
            _ => GioAction::new("test", GioActionParameterType::None),
        }
    }
    pub fn single_action_name(key_name: &str, mode: Mode) -> String {
        format!("{}.{}", MAIN_CONTROLLER_GROUP_NAME, "test")
    }

    pub fn from_control(control: &Control) -> Self {
        match control {
            Control::EnterChange => Action::PickChange,
            Control::RankThreeStars => Action::Rank(Rank::ThreeStars),
            Control::SetView => Action::PickViewOption,
            Control::SetOrder => Action::PickOrderSetting,
            _ => Action::Nothing,
        }
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
    fn gio_action_from_action() {
        let action = Action::PickChange;
        let gio_action = action.gio_action();
        assert_eq!("main-controller.enter-change-undefined", gio_action.name());
        assert_eq!(GioActionParameterType::None, gio_action.parameter_type());
    }
}
