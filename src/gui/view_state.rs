use crate::gui::direction::Direction;
use crate::gui::view_state::location::Location;
use crate::gui::view_state::location_stack::LocationStack;
use crate::gui::view_state::navigator::Navigator;
use crate::gui::view_state::selection::Selection;
use crate::gui::view_state::settings::Settings;
use crate::model::finder::Finder;
use crate::model::gallery::Gallery;
use crate::model::predicate::Predicate;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

pub mod location;
pub mod location_stack;
pub mod navigator;
pub mod selection;
pub mod selection_range;
pub mod settings;

#[derive(Debug, Default)]
pub struct ViewState {
    settings: Settings,
    navigator: Navigator,
    selection: Selection,
    gallery_rc: Rc<RefCell<Gallery>>,
    focus_at_coords: (i32, i32),
    locations: LocationStack,
    finder: Option<Finder>,
}

impl ViewState {
    pub fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    pub fn navigator_mut(&mut self) -> &mut Navigator {
        &mut self.navigator
    }

    pub fn set_navigator(&mut self, navigator: Navigator) {
        self.navigator = navigator
    }
    pub fn selected_indices(&self) -> Vec<usize> {
        if self.selection.has_selected() {
            self.selection.indices()
        } else {
            vec![self.gallery().current_picture_index()]
        }
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }

    pub fn finder(&self) -> &Option<Finder> {
        &self.finder
    }

    pub fn finder_mut(&mut self) -> &mut Option<Finder> {
        &mut self.finder
    }

    pub fn set_finder(&mut self, finder: Option<Finder>) {
        self.finder = finder
    }

    pub fn focus_at_coords(&self) -> (i32, i32) {
        self.focus_at_coords
    }

    pub fn gallery_rc(&self) -> Rc<RefCell<Gallery>> {
        self.gallery_rc.clone()
    }

    pub fn set_gallery_rc(&mut self, gallery_rc: Rc<RefCell<Gallery>>) {
        self.gallery_rc = gallery_rc
    }

    pub fn set_focus_at_coords(&mut self, coords: (i32, i32)) {
        self.focus_at_coords = coords
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn gallery(&self) -> Ref<'_, Gallery> {
        self.gallery_rc.borrow()
    }

    pub fn gallery_mut(&self) -> RefMut<'_, Gallery> {
        self.gallery_rc.borrow_mut()
    }

    pub fn locations(&self) -> &LocationStack {
        &self.locations
    }

    pub fn set_new_location(
        &mut self,
        sub_directory: Option<String>,
        predicate_opt: Option<Predicate>,
        position: usize,
        covers_only: bool,
    ) {
        let current_predicate_opt = self.locations.top().predicate();

        let new_predicate_opt = if let Some(predicate) = predicate_opt {
            if let Some(current) = current_predicate_opt {
                Some(Predicate::and(predicate, current))
            } else {
                Some(predicate)
            }
        } else {
            current_predicate_opt
        };

        let new_location = Location::new(sub_directory, new_predicate_opt, position, covers_only);
        self.locations.push(new_location);
        self.finder = None;
    }

    pub fn set_old_location(&mut self) {
        self.locations.pop();
        self.finder = None;
    }

    pub fn current_location(&self) -> Location {
        self.locations.top()
    }

    pub fn current_location_mut(&mut self) -> &mut Location {
        self.locations.top_mut()
    }

    pub fn set_current_location_position(&mut self, position: usize) {
        self.current_location_mut().set_position(position)
    }

    pub fn set_current_location_directory(&mut self, directory: Option<String>) {
        self.current_location_mut().set_sub_directory(directory)
    }
    pub fn set_current_location_covers_only(&mut self, covers_only: bool) {
        self.current_location_mut().set_covers_only(covers_only)
    }
}
