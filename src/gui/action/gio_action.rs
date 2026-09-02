use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter::GioActionParameter;
use crate::gui::action::gio_action_type::GioActionType;
use crate::gui::direction::Direction;
use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;
use crate::model::category::Category;
use crate::model::find::Find;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::view_option::ViewOption;
use gtk::gio::SimpleAction;
use gtk::glib::Variant;
use gtk::prelude::ActionExt;

pub type SimpleActionCall = (String, Option<Variant>);

// GioAction is the conversion of a concrete Action (with parameters) into something activable via a glib or gtk object
#[derive(Debug, Clone)]
pub struct GioAction {
    name: String,
    action_entry_name: String,
    parameter: Option<GioActionParameter>,
}

impl From<(&SimpleAction, Option<&Variant>)> for GioAction {
    fn from(tuple: (&SimpleAction, Option<&Variant>)) -> Self {
        let simple_action: &SimpleAction = tuple.0;
        let variant: Option<Variant> = match tuple.1 {
            None => None,
            Some(value) => Some(value.clone()),
        };
        let parameter: Option<GioActionParameter> = variant.map(|v| GioActionParameter::from(v));
        let name = simple_action.name().clone().to_string();
        Self {
            name: name.clone(),
            action_entry_name: format!("{}.{}", MAIN_CONTROLLER_GROUP_NAME, name),
            parameter: parameter,
        }
    }
}

// GUS
impl From<Action> for GioAction {
    fn from(action: Action) -> Self {
        let gio_action_ty = GioActionType::from(action.clone());
        let gio_action_parameter = match action.clone() {
            Action::AddCategory(category_name, target_category_name) => Some(
                GioActionParameter::from((category_name, target_category_name)),
            ),
            Action::AddTag(tag) => Some(GioActionParameter::from(tag)),
            Action::ApplyOrderSetting(order) => Some(GioActionParameter::from(order)),
            Action::ApplyViewSetting(view_option) => Some(GioActionParameter::from(view_option)),
            Action::Cancel => None,
            Action::CancelSelectionRange => None,
            Action::Categorize(category_opt) => Some(GioActionParameter::from(category_opt)),
            Action::ConfirmDeleteFile => None,
            Action::ConfirmMoveFile(file_path) => Some(GioActionParameter::from(file_path)),
            Action::Dismiss => None,
            Action::EnterAddTag => None,
            Action::EnterNewCategory => None,
            Action::SelectCategoryForPicture => None,
            Action::EnterIndex => None,
            Action::EnterFind(find) => Some(GioActionParameter::from(find)),
            Action::EnterLabel => None,
            Action::EnterRemoveTag => None,
            Action::EnterRename => None,
            Action::Find(find, criteria) => Some(GioActionParameter::from((find, criteria))),
            Action::FindNext => None,
            Action::FocusAt(col, row) => Some(GioActionParameter::from((col, row))),
            Action::GotoDirectory => None,
            Action::JumpToIndex(index) => Some(GioActionParameter::from(index)),
            Action::JumpToMark(mark) => Some(GioActionParameter::from(mark)),
            Action::JumpToRandom => None,
            Action::Label(label) => Some(GioActionParameter::from(label)),
            Action::Mark(mark) => Some(GioActionParameter::from(mark)),
            Action::MoveCategory(category_name, target_category_name) => Some(
                GioActionParameter::from((category_name, target_category_name)),
            ),
            Action::MoveFile => None,
            Action::MoveTowards(direction) => Some(GioActionParameter::from(direction)),
            Action::Nothing => None,
            Action::PickChange => None,
            Action::PickOrderSetting => None,
            Action::PickViewOption => None,
            Action::Quit => None,
            Action::QuitDirectory => None,
            Action::Rank(rank) => Some(GioActionParameter::from(rank)),
            Action::RemoveCategory(category_name) => Some(GioActionParameter::from(category_name)),
            Action::RemoveTag(tag) => Some(GioActionParameter::from(tag)),
            Action::Rename(name) => Some(GioActionParameter::from(name)),
            Action::RepeatAction => None,
            Action::RepeatRangeSelection => None,
            Action::Select(find) => Some(GioActionParameter::from(find)),
            Action::SelectCategoryToMove => None,
            Action::SelectCategoryToRemove => None,
            Action::SelectCategoryAddTarget(category_name) => {
                Some(GioActionParameter::from(category_name))
            }
            Action::SelectCategoryMoveTarget(category_name) => {
                Some(GioActionParameter::from(category_name))
            }
            Action::SetSelectionAll => None,
            Action::SetSelectionPage => None,
            Action::SetSelectionRangeEnd(index) => Some(GioActionParameter::from(index)),
            Action::SetSelectionRangeStart(index) => Some(GioActionParameter::from(index)),
            Action::Test(s) => Some(GioActionParameter::from(s)),
            Action::ToggleCover => None,
            Action::ToggleCoversView => None,
            Action::TogglePalette => None,
            Action::ToggleSelected(index) => Some(GioActionParameter::from(index)),
            Action::ToggleSelectedAt(col, row) => Some(GioActionParameter::from((col, row))),
            Action::ToggleSingleView => None,
            Action::ToggleSlideShow => None,
            Action::ToggleThumbnailsView => None,
            Action::ToggleTwoByTwoView => None,
            Action::Unlabel => None,
            Action::PickCatalogChange => None,
            Action::ViewCatalog => None,
        };
        Self {
            name: gio_action_ty.name(),
            action_entry_name: gio_action_ty.action_entry_name(),
            parameter: gio_action_parameter,
        }
    }
}
// QUX
impl From<GioAction> for Action {
    fn from(gio_action: GioAction) -> Self {
        match &gio_action.name() as &str {
            "add-category" => {
                let string_pair: (String, String) =
                    <(String, String)>::from(gio_action.parameter().unwrap());
                Action::AddCategory(string_pair.0, string_pair.1)
            }
            "add-tag" => Action::AddTag(String::from(gio_action.parameter().unwrap())),
            "apply-order-setting" => {
                Action::ApplyOrderSetting(Order::from(gio_action.parameter().unwrap()))
            }
            "apply-view-setting" => {
                Action::ApplyViewSetting(ViewOption::from(gio_action.parameter().unwrap()))
            }
            "cancel" => Action::Cancel,
            "cancel-selection-range" => Action::CancelSelectionRange,
            "categorize" => Action::Categorize(Category::from(gio_action.parameter().unwrap())),
            "confirm-delete-file" => Action::ConfirmDeleteFile,
            "confirm-move-file" => {
                Action::ConfirmMoveFile(String::from(gio_action.parameter().unwrap()))
            }
            "dismiss" => Action::Dismiss,
            "enter-add-tag" => Action::EnterAddTag,
            "enter-new-category" => Action::EnterNewCategory,
            "enter-index" => Action::EnterIndex,
            "enter-find" => Action::EnterFind(Find::from(gio_action.parameter().unwrap())),
            "enter-label" => Action::EnterLabel,
            "enter-remove-tag" => Action::EnterRemoveTag,
            "enter-rename" => Action::EnterRename,
            "find" => {
                let (find, criteria): (Find, String) =
                    <(Find, String)>::from(gio_action.parameter().unwrap());
                Action::Find(find, criteria)
            }
            "find-next" => Action::FindNext,
            "focus-at" => {
                let i32_pair: (i32, i32) = <(i32, i32)>::from(gio_action.parameter().unwrap());
                Action::FocusAt(i32_pair.0, i32_pair.1)
            }
            "go-to-directory" => Action::GotoDirectory,
            "jump-to-index" => Action::JumpToIndex(usize::from(gio_action.parameter().unwrap())),
            "jump-to-mark" => Action::JumpToMark(char::from(gio_action.parameter().unwrap())),
            "jump-to-random" => Action::JumpToRandom,
            "label" => Action::Label(String::from(gio_action.parameter().unwrap())),
            "mark" => Action::Mark(char::from(gio_action.parameter().unwrap())),
            "move-category" => {
                let string_pair: (String, String) =
                    <(String, String)>::from(gio_action.parameter().unwrap());
                Action::MoveCategory(string_pair.0, string_pair.1)
            }
            "move-file" => Action::MoveFile,
            "move-towards" => Action::MoveTowards(Direction::from(gio_action.parameter().unwrap())),
            "nothing" => Action::Nothing,
            "pick-catalog-change" => Action::PickCatalogChange,
            "pick-change" => Action::PickChange,
            "pick-order-setting" => Action::PickOrderSetting,
            "pick-view-option" => Action::PickViewOption,
            "quit" => Action::Quit,
            "quit-directory" => Action::QuitDirectory,
            "rank" => Action::Rank(Rank::from(gio_action.parameter().unwrap())),
            "remove-category" => {
                Action::RemoveCategory(String::from(gio_action.parameter().unwrap()))
            }
            "remove-tag" => Action::RemoveTag(String::from(gio_action.parameter().unwrap())),
            "rename" => Action::Rename(String::from(gio_action.parameter().unwrap())),
            "repeat-action" => Action::RepeatAction,
            "repeat-range-selection" => Action::RepeatRangeSelection,
            "select" => Action::Select(Find::from(gio_action.parameter().unwrap())),
            "select-category-for-picture" => Action::SelectCategoryForPicture,
            "select-category-to-move" => Action::SelectCategoryToMove,
            "select-category-to-remove" => Action::SelectCategoryToRemove,
            "select-category-add-target" => {
                Action::SelectCategoryAddTarget(String::from(gio_action.parameter().unwrap()))
            }
            "select-category-move-target" => {
                Action::SelectCategoryMoveTarget(String::from(gio_action.parameter().unwrap()))
            }
            "set-selection-all" => Action::SetSelectionAll,
            "set-selection-page" => Action::SetSelectionPage,
            "set-selection-range-end" => {
                Action::SetSelectionRangeEnd(usize::from(gio_action.parameter().unwrap()))
            }
            "set-selection-range-start" => {
                Action::SetSelectionRangeStart(usize::from(gio_action.parameter().unwrap()))
            }
            "test" => Action::Test(String::from(gio_action.parameter().unwrap())),
            "toggle-cover" => Action::ToggleCover,
            "toggle-covers-view" => Action::ToggleCoversView,
            "toggle-palette" => Action::TogglePalette,
            "toggle-selected" => {
                Action::ToggleSelected(usize::from(gio_action.parameter().unwrap()))
            }
            "toggle-selected-at" => {
                let i32_pair: (i32, i32) = <(i32, i32)>::from(gio_action.parameter().unwrap());
                Action::ToggleSelectedAt(i32_pair.0, i32_pair.1)
            }
            "toggle-single-view" => Action::ToggleSingleView,
            "toggle-slide-show" => Action::ToggleSlideShow,
            "toggle-thumbnails-view" => Action::ToggleThumbnailsView,
            "toggle-two-by-two-view" => Action::ToggleTwoByTwoView,
            "unlabel" => Action::Unlabel,
            "view-catalog" => Action::ViewCatalog,
            _ => Action::Nothing,
        }
    }
}

impl GioAction {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn action_entry_name(&self) -> String {
        self.action_entry_name.clone()
    }

    pub fn parameter(&self) -> Option<GioActionParameter> {
        self.parameter.clone()
    }

    pub fn parameter_as_variant(&self) -> Option<gtk::glib::Variant> {
        self.parameter.clone().map(|p| p.variant().clone())
    }

    pub fn to_simple_action_call(&self) -> SimpleActionCall {
        (self.action_entry_name(), self.parameter_as_variant())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::category::category_from_string;

    fn check_action_to_and_from(action: Action) {
        let source = action.clone();
        let gio_action = GioAction::from(source.clone());
        let target = Action::from(gio_action);
        assert_eq!(target, source);
    }

    #[test]
    fn gio_action_from_action_and_vice_versa() {
        check_action_to_and_from(Action::AddCategory("foo".to_string(), "bar".to_string()));
        check_action_to_and_from(Action::AddTag("foo".to_string()));
        check_action_to_and_from(Action::ApplyOrderSetting(Order::Name));
        check_action_to_and_from(Action::ApplyViewSetting(ViewOption::Thumbnails));
        check_action_to_and_from(Action::CancelSelectionRange);
        check_action_to_and_from(Action::Categorize(category_from_string("foo")));
        check_action_to_and_from(Action::ConfirmDeleteFile);
        check_action_to_and_from(Action::ConfirmMoveFile("foo".to_string()));
        check_action_to_and_from(Action::EnterAddTag);
        check_action_to_and_from(Action::EnterNewCategory);
        check_action_to_and_from(Action::SelectCategoryForPicture);
        check_action_to_and_from(Action::EnterIndex);
        check_action_to_and_from(Action::EnterLabel);
        check_action_to_and_from(Action::EnterRemoveTag);
        check_action_to_and_from(Action::EnterRename);
        check_action_to_and_from(Action::Find(Find::Label));
        check_action_to_and_from(Action::FindNext);
        check_action_to_and_from(Action::FocusAt(3, 8));
        check_action_to_and_from(Action::GotoDirectory);
        check_action_to_and_from(Action::JumpToIndex(4807));
        check_action_to_and_from(Action::JumpToMark('f'));
        check_action_to_and_from(Action::JumpToRandom);
        check_action_to_and_from(Action::Label("foo".to_string()));
        check_action_to_and_from(Action::Mark('f'));
        check_action_to_and_from(Action::MoveCategory("foo".to_string(), "bar".to_string()));
        check_action_to_and_from(Action::MoveFile);
        check_action_to_and_from(Action::MoveTowards(Direction::Down));
        check_action_to_and_from(Action::MoveTowards(Direction::NextPage));
        check_action_to_and_from(Action::Nothing);
        check_action_to_and_from(Action::PickChange);
        check_action_to_and_from(Action::PickOrderSetting);
        check_action_to_and_from(Action::PickViewOption);
        check_action_to_and_from(Action::Quit);
        check_action_to_and_from(Action::QuitDirectory);
        check_action_to_and_from(Action::Rank(Rank::ThreeStars));
        check_action_to_and_from(Action::RemoveCategory("foo".to_string()));
        check_action_to_and_from(Action::RemoveTag("foo".to_string()));
        check_action_to_and_from(Action::Rename("foo".to_string()));
        check_action_to_and_from(Action::RepeatAction);
        check_action_to_and_from(Action::RepeatRangeSelection);
        check_action_to_and_from(Action::Select(Find::Label));
        check_action_to_and_from(Action::SelectCategoryForPicture);
        check_action_to_and_from(Action::SelectCategoryToMove);
        check_action_to_and_from(Action::SelectCategoryToRemove);
        check_action_to_and_from(Action::SelectCategoryAddTarget("foo".to_string()));
        check_action_to_and_from(Action::SetSelectionAll);
        check_action_to_and_from(Action::SetSelectionPage);
        check_action_to_and_from(Action::SetSelectionRangeEnd(4807));
        check_action_to_and_from(Action::SetSelectionRangeStart(4807));
        check_action_to_and_from(Action::Test("foo".to_string()));
        check_action_to_and_from(Action::ToggleCover);
        check_action_to_and_from(Action::ToggleCoversView);
        check_action_to_and_from(Action::TogglePalette);
        check_action_to_and_from(Action::ToggleSelected(4807));
        check_action_to_and_from(Action::ToggleSelectedAt(3, 8));
        check_action_to_and_from(Action::ToggleSingleView);
        check_action_to_and_from(Action::ToggleSlideShow);
        check_action_to_and_from(Action::ToggleThumbnailsView);
        check_action_to_and_from(Action::Unlabel);
    }
}
