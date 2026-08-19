use crate::gui::navigator::Navigator;
use crate::gui::view::View;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GsrPictureGrid {
    pub view: Shared<View>,
    pub navigator: Shared<Navigator>,
    pub gallery: Shared<Gallery>,
}

impl Default for GsrPictureGrid {
    fn default() -> Self {
        Self {
            view: Rc::new(RefCell::new(View::default())),
            navigator: Rc::new(RefCell::new(Navigator::default())),
            gallery: Rc::new(RefCell::new(Gallery::default())),
        }
    }
}
impl GsrPictureGrid {
    pub fn initialize(
        &self,
        view: RefCell<View>,
        navigator: RefCell<Navigator>,
        gallery: RefCell<Gallery>,
    ) {
        // register the shared tools
        *self.view.borrow_mut() = view.borrow().clone();
        *self.navigator.borrow_mut() = navigator.borrow().clone();
        *self.gallery.borrow_mut() = gallery.borrow().clone();
    }
}
#[glib::object_subclass]
impl ObjectSubclass for GsrPictureGrid {
    const NAME: &'static str = "GsrPictureGrid";
    type Type = super::GsrPictureGrid;
    type ParentType = gtk::Grid;
}

impl ObjectImpl for GsrPictureGrid {}

impl WidgetImpl for GsrPictureGrid {}

impl GridImpl for GsrPictureGrid {}
