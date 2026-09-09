use crate::gui::direction::Direction;
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
        self.set_current_location(
            new_location.sub_directory(),
            new_location.predicate(),
            new_location.position(),
            new_location.covers_only(),
        );
        self.finder = None;
    }

    pub fn set_old_location(&mut self) {
        let old_location = self.saved_locations.pop();
        self.current_location = old_location.unwrap_or_default();
        self.finder = None;
    }

    pub fn set_current_location(
        &mut self,
        sub_directory: Option<String>,
        predicate: Option<Predicate>,
        position: usize,
        covers_only: bool,
    ) {
        self.current_location =
            Location::new(sub_directory.clone(), predicate, position, covers_only);
        self.gallery.set_sub_folder(sub_directory);
        if self
            .navigator
            .can_move(&Direction::Index { value: position })
        {
            self.navigator
                .move_towards(&Direction::Index { value: position })
        };
    }

    pub fn set_current_location_position(&mut self, position: usize) {
        self.current_location.set_position(position)
    }

    pub fn set_current_location_covers_only(&mut self, covers_only: bool) {
        self.current_location.set_covers_only(covers_only)
    }

    pub fn current_location(&self) -> Location {
        self.current_location.clone()
    }
}
