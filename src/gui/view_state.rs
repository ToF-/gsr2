use crate::gui::view_state::location::Location;
use crate::gui::view_state::navigator::Navigator;
use crate::gui::view_state::selection::Selection;
use crate::gui::view_state::settings::Settings;
use crate::model::finder::Finder;
use crate::model::gallery::Gallery;
use crate::model::predicate::Predicate;
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

impl ViewState {
    pub fn set_new_location(
        &mut self,
        sub_directory: Option<String>,
        predicate: Option<Predicate>,
        position: usize,
        covers_only: bool,
    ) {
        let current_predicate = self.current_location.predicate().clone();
        let new_predicate = if predicate.is_some() && current_predicate.is_some() {
            Some(Predicate::and(
                predicate.unwrap(),
                current_predicate.unwrap(),
            ))
        } else {
            predicate
        };
        let new_location = Location::new(sub_directory, new_predicate, position, covers_only);
        self.saved_locations.push(self.current_location.clone());
        self.current_location = new_location.clone()
    }

    pub fn set_old_location(&mut self) {
        let old_location = self.saved_locations.pop();
        self.current_location = old_location.unwrap_or_default()
    }
}
