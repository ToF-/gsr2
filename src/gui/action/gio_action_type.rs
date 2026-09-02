use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter_type::GioActionParameterType;
use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;

// GioActionType describe a GioAction in terms of name and type of parameter
#[derive(Debug)]
pub struct GioActionType {
    name: String,
    action_entry_name: String,
    parameter_type: GioActionParameterType,
}

impl GioActionType {
    pub fn new(name: &str, action_parameter_type: GioActionParameterType) -> Self {
        Self {
            name: name.to_string(),
            action_entry_name: format!("{}.{}", MAIN_CONTROLLER_GROUP_NAME, name),
            parameter_type: action_parameter_type,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn action_entry_name(&self) -> String {
        self.action_entry_name.clone()
    }
    pub fn parameter_type(&self) -> GioActionParameterType {
        self.parameter_type.clone()
    }
}

impl From<Action> for GioActionType {
    fn from(action: Action) -> Self {
        match action {
            Action::AddCategory(_, _) => {
                GioActionType::new("add-category", GioActionParameterType::StringPair)
            }
            Action::AddTag(_) => GioActionType::new("add-tag", GioActionParameterType::String),
            Action::ApplyOrderSetting(_) => {
                GioActionType::new("apply-order-setting", GioActionParameterType::Int32)
            }
            Action::ApplyViewSetting(_) => {
                GioActionType::new("apply-view-setting", GioActionParameterType::Int32)
            }
            Action::Cancel => GioActionType::new("cancel", GioActionParameterType::None),
            Action::CancelSelectionRange => {
                GioActionType::new("cancel-selection-range", GioActionParameterType::None)
            }
            Action::Categorize(_) => {
                GioActionType::new("categorize", GioActionParameterType::String)
            }
            Action::ConfirmDeleteFile => {
                GioActionType::new("confirm-delete-file", GioActionParameterType::None)
            }
            Action::ConfirmMoveFile(_) => {
                GioActionType::new("confirm-move-file", GioActionParameterType::String)
            }
            Action::Dismiss => GioActionType::new("dismiss", GioActionParameterType::None),
            Action::EnterAddTag => {
                GioActionType::new("enter-add-tag", GioActionParameterType::None)
            }
            Action::SelectCategoryForPicture => {
                GioActionType::new("select-category-for-picture", GioActionParameterType::None)
            }
            Action::SelectCategoryAddTarget(_) => {
                GioActionType::new("select-category-add-target", GioActionParameterType::String)
            }
            Action::SelectCategoryMoveTarget(_) => GioActionType::new(
                "select-category-move-target",
                GioActionParameterType::String,
            ),
            Action::EnterIndex => GioActionType::new("enter-index", GioActionParameterType::None),
            Action::EnterFind(_) => GioActionType::new("enter-find", GioActionParameterType::Int32),
            Action::EnterLabel => GioActionType::new("enter-label", GioActionParameterType::None),
            Action::EnterNewCategory => {
                GioActionType::new("enter-new-category", GioActionParameterType::None)
            }
            Action::EnterRemoveTag => {
                GioActionType::new("enter-remove-tag", GioActionParameterType::None)
            }
            Action::EnterRename => GioActionType::new("enter-rename", GioActionParameterType::None),
            Action::Find(_) => GioActionType::new("find", GioActionParameterType::Int32),
            Action::FindNext => GioActionType::new("find-next", GioActionParameterType::None),
            Action::FocusAt(_, _) => {
                GioActionType::new("focus-at", GioActionParameterType::Int32Pair)
            }
            Action::GotoDirectory => {
                GioActionType::new("go-to-directory", GioActionParameterType::None)
            }
            Action::JumpToIndex(_) => {
                GioActionType::new("jump-to-index", GioActionParameterType::Int32)
            }
            Action::JumpToMark(_) => {
                GioActionType::new("jump-to-mark", GioActionParameterType::Char)
            }
            Action::JumpToRandom => {
                GioActionType::new("jump-to-random", GioActionParameterType::None)
            }
            Action::Label(_) => GioActionType::new("label", GioActionParameterType::String),
            Action::Mark(_) => GioActionType::new("mark", GioActionParameterType::Char),
            Action::MoveCategory(_, _) => {
                GioActionType::new("move-category", GioActionParameterType::StringPair)
            }
            Action::MoveFile => GioActionType::new("move-file", GioActionParameterType::None),
            Action::MoveTowards(_) => {
                GioActionType::new("move-towards", GioActionParameterType::Int32)
            }
            Action::Nothing => GioActionType::new("nothing", GioActionParameterType::None),
            Action::PickCatalogChange => {
                GioActionType::new("pick-catalog-change", GioActionParameterType::None)
            }
            Action::PickChange => GioActionType::new("pick-change", GioActionParameterType::None),
            Action::PickOrderSetting => {
                GioActionType::new("pick-order-setting", GioActionParameterType::None)
            }
            Action::PickViewOption => {
                GioActionType::new("pick-view-option", GioActionParameterType::None)
            }
            Action::Quit => GioActionType::new("quit", GioActionParameterType::None),
            Action::QuitDirectory => {
                GioActionType::new("quit-directory", GioActionParameterType::None)
            }
            Action::Rank(_) => GioActionType::new("rank", GioActionParameterType::Int64),
            Action::RemoveCategory(_) => {
                GioActionType::new("remove-category", GioActionParameterType::String)
            }
            Action::RemoveTag(_) => {
                GioActionType::new("remove-tag", GioActionParameterType::String)
            }
            Action::Rename(_) => GioActionType::new("rename", GioActionParameterType::String),
            Action::RepeatAction => {
                GioActionType::new("repeat-action", GioActionParameterType::None)
            }
            Action::RepeatRangeSelection => {
                GioActionType::new("repeat-range-selection", GioActionParameterType::None)
            }
            Action::Select(_) => GioActionType::new("select", GioActionParameterType::String),
            Action::SelectCategoryToMove => {
                GioActionType::new("select-category-to-move", GioActionParameterType::None)
            }
            Action::SelectCategoryToRemove => {
                GioActionType::new("select-category-to-remove", GioActionParameterType::None)
            }
            Action::SelectCategoryMoveTarget(_) => GioActionType::new(
                "select-category-move-target",
                GioActionParameterType::String,
            ),
            Action::SetSelectionAll => {
                GioActionType::new("set-selection-all", GioActionParameterType::None)
            }
            Action::SetSelectionPage => {
                GioActionType::new("set-selection-page", GioActionParameterType::None)
            }
            Action::SetSelectionRangeEnd(_) => {
                GioActionType::new("set-selection-range-end", GioActionParameterType::Int32)
            }
            Action::SetSelectionRangeStart(_) => {
                GioActionType::new("set-selection-range-start", GioActionParameterType::Int32)
            }
            Action::Test(_) => GioActionType::new("test", GioActionParameterType::String),
            Action::ToggleCover => GioActionType::new("toggle-cover", GioActionParameterType::None),
            Action::ToggleCoversView => {
                GioActionType::new("toggle-covers-view", GioActionParameterType::None)
            }
            Action::TogglePalette => {
                GioActionType::new("toggle-palette", GioActionParameterType::None)
            }
            Action::ToggleSelected(_) => {
                GioActionType::new("toggle-selected", GioActionParameterType::None)
            }
            Action::ToggleSelectedAt(_, _) => {
                GioActionType::new("toggle-selected-at", GioActionParameterType::Int32Pair)
            }
            Action::ToggleSingleView => {
                GioActionType::new("toggle-single-view", GioActionParameterType::None)
            }
            Action::ToggleSlideShow => {
                GioActionType::new("toggle-slide-show", GioActionParameterType::None)
            }
            Action::ToggleThumbnailsView => {
                GioActionType::new("toggle-thumbnails-view", GioActionParameterType::None)
            }
            Action::ToggleTwoByTwoView => {
                GioActionType::new("toggle-two-by-two-view", GioActionParameterType::None)
            }
            Action::Unlabel => GioActionType::new("unlabel", GioActionParameterType::None),
            Action::ViewCatalog => GioActionType::new("view-catalog", GioActionParameterType::None),
        }
    }
}
