use crate::gui::view_state::navigator::Navigator;
use crate::gui::view_state::selection::Selection;
use crate::gui::view_state::settings::Settings;
use crate::model::gallery::Gallery;
pub mod navigator;
pub mod selection;
pub mod settings;

#[derive(Debug,Default)]
pub struct ViewState {
    pub settings: Settings,
    pub navigator: Navigator,
    pub selection: Selection,
    pub gallery: Gallery,
    pub focus_at_coords: (i32, i32),
}
