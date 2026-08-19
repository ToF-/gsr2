use crate::gui::main_controller::MainController;
use std::rc::Rc;
use crate::gui::view::View;
use crate::model::shared::Shared;
use std::cell::RefCell;
use gtk::glib;
use gtk::subclass::prelude::*;
use glib::prelude::*;

#[derive(Default)]
pub struct GsrApplication {
    pub view: RefCell<Option<Shared<View>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for GsrApplication {
    const NAME: &'static str = "GsrApplication";

    type Type = super::GsrApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for GsrApplication {}

impl ApplicationImpl for GsrApplication {}

impl GtkApplicationImpl for GsrApplication {}


impl GsrApplication {
    pub fn set_view(&self, view: Rc<RefCell<View>>) {
        *self.imp().view.borrow_mut() = Some(view);
    }

    pub fn view(&self) -> Rc<RefCell<View>> {
        self.imp()
            .view
            .borrow()
            .as_ref()
            .expect("View has not been initialized")
            .clone()
    }
}
