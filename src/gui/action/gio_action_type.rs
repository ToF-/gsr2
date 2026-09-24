use crate::gui::action::A;
use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter_type::GAPT;
use crate::gui::action::gio_action_parameter_type::GioActionParameterType;
use crate::gui::controller::MAIN_CONTROLLER_GROUP_NAME;

// GioActionType describe a GioAction in terms of name and type of parameter
#[derive(Debug)]
pub struct GioActionType {
    name: String,
    action_entry_name: String,
    parameter_type: GioActionParameterType,
}

pub type GAT = GioActionType;

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
            A::AddCategory(_, _) => GAT::new("add-category", GAPT::StringPair),
            A::AddTag(_) => GAT::new("add-tag", GAPT::String),
            A::ApplyOrderSetting(_) => GAT::new("apply-order-setting", GAPT::Int32),
            A::ApplyViewSetting(_) => GAT::new("apply-view-setting", GAPT::Int32),
            A::Cancel => GAT::new("cancel", GAPT::None),
            A::CancelSelectionRange => GAT::new("cancel-selection-range", GAPT::None),
            A::Categorize(_) => GAT::new("categorize", GAPT::String),
            A::DeleteSelectedPicture(_) => GAT::new("delete-selected-picture", GAPT::String),
            A::Dismiss => GAT::new("dismiss", GAPT::None),
            A::EnterAddTag => GAT::new("enter-add-tag", GAPT::None),
            A::EnterDeletePicture => GAT::new("enter-delete-picture", GAPT::None),
            A::EnterExtractFileNames => GAT::new("enter-extract-filenames", GAPT::None),
            A::EnterFind(_) => GAT::new("enter-find", GAPT::Int32),
            A::EnterIndex => GAT::new("enter-index", GAPT::None),
            A::EnterLabel => GAT::new("enter-label", GAPT::None),
            A::EnterNewCategory => GAT::new("enter-new-category", GAPT::None),
            A::EnterRemoveTag => GAT::new("enter-remove-tag", GAPT::None),
            A::EnterRename => GAT::new("enter-rename", GAPT::None),
            A::EnterSelect(_) => GAT::new("enter-select", GAPT::Int32),
            A::ExtractFileNames(_) => GAT::new("extract-filenames", GAPT::String),
            A::Find(_, _) => GAT::new("find", GAPT::Int32String),
            A::FindNext => GAT::new("find-next", GAPT::None),
            A::FocusAt(_, _) => GAT::new("focus-at", GAPT::Int32Pair),
            A::GotoDirectory => GAT::new("go-to-directory", GAPT::None),
            A::Help => GAT::new("help", GAPT::None),
            A::JumpToIndex(_) => GAT::new("jump-to-index", GAPT::Int32),
            A::JumpToMark(_) => GAT::new("jump-to-mark", GAPT::Char),
            A::JumpToRandom => GAT::new("jump-to-random", GAPT::None),
            A::Label(_) => GAT::new("label", GAPT::String),
            A::Mark(_) => GAT::new("mark", GAPT::Char),
            A::MoveCategory(_, _) => GAT::new("move-category", GAPT::StringPair),
            A::MoveFile => GAT::new("move-file", GAPT::None),
            A::MoveSelectedPicture(_) => GAT::new("move-selected-picture", GAPT::String),
            A::MoveTowards(_) => GAT::new("move-towards", GAPT::Int32),
            A::Nothing => GAT::new("nothing", GAPT::None),
            A::NextSlide => GAT::new("next-slide", GAPT::None),
            A::PickCatalogChange => GAT::new("pick-catalog-change", GAPT::None),
            A::PickChange => GAT::new("pick-change", GAPT::None),
            A::PickFindOption => GAT::new("pick-find-option", GAPT::None),
            A::PickMark => GAT::new("pick-mark", GAPT::None),
            A::PickOrderSetting => GAT::new("pick-order-setting", GAPT::None),
            A::PickSelectOption => GAT::new("pick-select-option", GAPT::None),
            A::PickTargetMark => GAT::new("pick-target-mark", GAPT::None),
            A::PickViewOption => GAT::new("pick-view-option", GAPT::None),
            A::Quit => GAT::new("quit", GAPT::None),
            A::QuitDirectory => GAT::new("quit-directory", GAPT::None),
            A::Rank(_) => GAT::new("rank", GAPT::Int64),
            A::RedoFind => GAT::new("redo-find", GAPT::None),
            A::RemoveCategory(_) => GAT::new("remove-category", GAPT::String),
            A::RemoveTag(_) => GAT::new("remove-tag", GAPT::String),
            A::Rename(_) => GAT::new("rename", GAPT::String),
            A::ResumeSlideShow => GAT::new("resume-slideshow", GAPT::None),
            A::RepeatAction => GAT::new("repeat-action", GAPT::None),
            A::RepeatRangeSelection => GAT::new("repeat-range-selection", GAPT::None),
            A::Select(_, _) => GAT::new("select", GAPT::Int32String),
            A::SelectCategoryAddTarget(_) => GAT::new("select-category-add-target", GAPT::String),
            A::SelectCategoryForPicture => GAT::new("select-category-for-picture", GAPT::None),
            A::SelectCategoryMoveTarget(_) => GAT::new("select-category-move-target", GAPT::String),
            A::SelectCategoryToMove => GAT::new("select-category-to-move", GAPT::None),
            A::SelectCategoryToRemove => GAT::new("select-category-to-remove", GAPT::None),
            A::SetSelectionAll => GAT::new("set-selection-all", GAPT::None),
            A::SetSelectionPage => GAT::new("set-selection-page", GAPT::None),
            A::SetSelectionRangeEnd(_) => GAT::new("set-selection-range-end", GAPT::Int32),
            A::SetSelectionRangeStart(_) => GAT::new("set-selection-range-start", GAPT::Int32),
            A::Test(_) => GAT::new("test", GAPT::String),
            A::ToggleBlinking => GAT::new("toggle-blinking", GAPT::None),
            A::ToggleCover => GAT::new("toggle-cover", GAPT::None),
            A::ToggleCoversView => GAT::new("toggle-covers-view", GAPT::None),
            A::ToggleExpand => GAT::new("toggle-expand", GAPT::None),
            A::TogglePalette => GAT::new("toggle-palette", GAPT::None),
            A::TogglePicturesPerRow(_) => GAT::new("toggle-pictures-per-row", GAPT::Int32),
            A::ToggleSelected => GAT::new("toggle-selected", GAPT::None),
            A::ToggleSelectedAt(_, _) => GAT::new("toggle-selected-at", GAPT::Int32Pair),
            A::ToggleSlideShow => GAT::new("toggle-slide-show", GAPT::None),
            A::ToggleThumbnailsView => GAT::new("toggle-thumbnails-view", GAPT::None),
            A::ToggleTwoByTwoView => GAT::new("toggle-two-by-two-view", GAPT::None),
            A::Unlabel => GAT::new("unlabel", GAPT::None),
            A::ViewCatalog => GAT::new("view-catalog", GAPT::None),
        }
    }
}
