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
    pub view:      RefCell<Option<Shared<View>>>,
    pub navigator: RefCell<Option<Shared<Navigator>>>,
    pub gallery:   RefCell<Option<Shared<Gallery>>>,
}

impl Default for GsrPictureGrid {
    fn default() -> Self {
        Self {
            view:      RefCell::new(None),
            navigator: RefCell::new(None),
            gallery:   RefCell::new(None),
        }
    }
}
impl GsrPictureGrid {
    pub fn initialize(&self,
        view: Shared<View>,
        navigator: Shared<Navigator>,
        gallery: Shared<Gallery>,
    ) {
        *(self.view.borrow_mut()) = Some(view.clone());
        *(self.navigator.borrow_mut()) = Some(navigator.clone());
        *(self.gallery.borrow_mut()) = Some(gallery.clone());
    }

    pub fn view(&self) -> View {
        self.view.borrow().as_ref().unwrap().borrow().clone()
    }
    pub fn navigator(&self) -> Navigator {
        self.navigator.borrow().as_ref().unwrap().borrow().clone()
    }
    pub fn gallery(&self) -> Gallery {
        self.gallery.borrow().as_ref().unwrap().borrow().clone()
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
