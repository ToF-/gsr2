use crate::gui::view_state::location::Location;
use crate::gui::view_state::navigator::Navigator;
use crate::gui::view_state::selection::Selection;
use crate::gui::view_state::settings::Settings;
use crate::model::finder::Finder;
use crate::model::gallery::Gallery;
pub mod location;
pub mod navigator;
pub mod selection;
pub mod selection_range;
pub mod settings;

#[derive(Debug, Default)]
pub struct ViewState {
    pub settings: Settings,
    pub navigator: Navigator,
    pub selection: Selection,
    pub gallery: Gallery,
    pub focus_at_coords: (i32, i32),
    pub saved_locations: Vec<Location>,
    pub current_location: Location,
    pub finder: Option<Finder>,
}
