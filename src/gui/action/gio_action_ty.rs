use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter_type::GioActionParameterType;
use crate::gui::main_controller::MAIN_CONTROLLER_GROUP_NAME;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct GioActionTy {
    name: String,
    action_entry_name: String,
    parameter_type: GioActionParameterType,
}

impl GioActionTy {
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

impl From<Action> for GioActionTy {
    fn from(action: Action) -> Self {
        match action {
            Action::AddCategory(_, _) => {
                GioActionTy::new("add-category", GioActionParameterType::StringPair)
            }
            Action::AddTag(_) => GioActionTy::new("add-tag", GioActionParameterType::String),
            Action::ApplyOrderSetting(_) => {
                GioActionTy::new("apply-order-setting", GioActionParameterType::String)
            }
            Action::ApplyViewSetting(_) => {
                GioActionTy::new("apply-view-setting", GioActionParameterType::String)
            }
            Action::CancelSelectionRange => {
                GioActionTy::new("cancel-selection-range", GioActionParameterType::None)
            }
            Action::Categorize(_) => GioActionTy::new("categorize", GioActionParameterType::String),
            Action::ConfirmDeleteFile => {
                GioActionTy::new("confirm-delete-file", GioActionParameterType::None)
            }
            Action::ConfirmMoveFile(_) => {
                GioActionTy::new("confirm-move-file", GioActionParameterType::String)
            }
            Action::EnterAddTag => GioActionTy::new("enter-add-tag", GioActionParameterType::None),
            Action::EnterCategory => {
                GioActionTy::new("enter-category", GioActionParameterType::None)
            }
            Action::EnterIndex => GioActionTy::new("enter-index", GioActionParameterType::None),
            Action::EnterLabel => GioActionTy::new("enter-label", GioActionParameterType::None),
            Action::EnterRemoveTag => {
                GioActionTy::new("enter-remove-tag", GioActionParameterType::None)
            }
            Action::EnterRename => GioActionTy::new("enter-rename", GioActionParameterType::None),
            Action::Find(_) => GioActionTy::new("find", GioActionParameterType::String),
            Action::FindNext => GioActionTy::new("find-next", GioActionParameterType::None),
            Action::GotoDirectory(_) => {
                GioActionTy::new("go-to-directory", GioActionParameterType::String)
            }
            Action::JumpToIndex(_) => {
                GioActionTy::new("jump-to-index", GioActionParameterType::Int32)
            }
            Action::JumpToMark(_) => GioActionTy::new("jump-to-mark", GioActionParameterType::Char),
            Action::JumpToRandom => {
                GioActionTy::new("jump-to-random", GioActionParameterType::None)
            }
            Action::Label(_) => GioActionTy::new("label", GioActionParameterType::String),
            Action::Mark(_) => GioActionTy::new("mark", GioActionParameterType::Char),
            Action::MoveCategory(_, _) => {
                GioActionTy::new("move-category", GioActionParameterType::StringPair)
            }
            Action::MoveFile => GioActionTy::new("move-file", GioActionParameterType::None),
            Action::MoveTowards(_) => {
                GioActionTy::new("move-towards", GioActionParameterType::String)
            }
            Action::Nothing => GioActionTy::new("nothing", GioActionParameterType::None),
            Action::PickChange => GioActionTy::new("pick-change", GioActionParameterType::None),
            Action::PickOrderSetting => {
                GioActionTy::new("pick-order-setting", GioActionParameterType::None)
            }
            Action::PickViewOption => {
                GioActionTy::new("pick-view-option", GioActionParameterType::None)
            }
            Action::Quit => GioActionTy::new("quit", GioActionParameterType::None),
            Action::QuitDirectory => {
                GioActionTy::new("quit-directory", GioActionParameterType::None)
            }
            Action::Rank(_) => GioActionTy::new("rank", GioActionParameterType::Int32),
            Action::RemoveCategory(_) => {
                GioActionTy::new("remove-category", GioActionParameterType::String)
            }
            Action::RemoveTag(_) => GioActionTy::new("remove-tag", GioActionParameterType::String),
            Action::Rename(_) => GioActionTy::new("rename", GioActionParameterType::String),
            Action::RepeatAction => GioActionTy::new("repeat-action", GioActionParameterType::None),
            Action::RepeatRangeSelection => {
                GioActionTy::new("repeat-range-selection", GioActionParameterType::None)
            }
            Action::Select(_) => GioActionTy::new("select", GioActionParameterType::String),
            Action::SetSelectionAll => {
                GioActionTy::new("set-selection-all", GioActionParameterType::None)
            }
            Action::SetSelectionPage => {
                GioActionTy::new("set-selection-page", GioActionParameterType::None)
            }
            Action::SetSelectionRangeEnd(_) => {
                GioActionTy::new("set-selection-range-end", GioActionParameterType::Int32)
            }
            Action::SetSelectionRangeStart(_) => {
                GioActionTy::new("set-selection-range-start", GioActionParameterType::Int32)
            }
            Action::Test(_) => GioActionTy::new("test", GioActionParameterType::String),
            Action::ToggleCover => GioActionTy::new("toggle-cover", GioActionParameterType::None),
            Action::ToggleCoversView => {
                GioActionTy::new("toggle-cover-view", GioActionParameterType::None)
            }
            Action::ToggleSelected(_) => {
                GioActionTy::new("toggle-selected", GioActionParameterType::None)
            }
            Action::ToggleSingleView => {
                GioActionTy::new("toggle-single-view", GioActionParameterType::None)
            }
            Action::ToggleSlideShow => {
                GioActionTy::new("toggle-single-view", GioActionParameterType::None)
            }
            Action::ToggleThumbnailsView => {
                GioActionTy::new("toggle-thumbnain-view", GioActionParameterType::None)
            }
            Action::Unlabel => GioActionTy::new("unlabel", GioActionParameterType::None),
            _ => GioActionTy::new("test", GioActionParameterType::None),
        }
    }
}
