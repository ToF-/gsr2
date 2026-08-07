use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use gtk::glib;
use gtk::subclass::prelude::ApplicationImpl;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GsrApplication {
    main_controller_rc: RcMainController,
}

impl GsrApplication {
    pub fn main_controller_rc(&self) -> RcMainController {
        self.main_controller_rc.clone()
    }

    pub fn set_main_controller_rc(&self, main_controller: MainController) {
        *self.main_controller_rc.borrow_mut() = main_controller;
    }
}
impl Default for GsrApplication {
    fn default() -> Self {
        Self {
            main_controller_rc: Rc::new(RefCell::new(MainController::new(None))),
        }
    }
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
