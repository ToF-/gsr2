use crate::gui::view_state::navigator::Navigator;
use crate::gui::view_state::selection::Selection;
use crate::gui::view_state::settings::Settings;
use crate::model::gallery::Gallery;
use std::cell::RefCell;
use std::rc::Rc;
pub mod navigator;
pub mod selection;
pub mod settings;

#[derive(Default)]
pub struct ViewState {
    pub settings: Rc<RefCell<Settings>>,
    pub navigator: Rc<RefCell<Navigator>>,
    pub selection: Rc<RefCell<Selection>>,
    pub gallery: Rc<RefCell<Gallery>>,
}
