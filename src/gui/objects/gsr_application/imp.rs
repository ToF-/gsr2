use crate::gui::main_controller::MainController;
use crate::gui::view::View;
use crate::model::shared::Shared;
use gtk::{glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GsrApplication {
}

#[glib::object_subclass]
impl ObjectSubclass for GsrApplication {
    const NAME: &'static str = "GsrApplication";

    type Type = super::GsrApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for GsrApplication {}
impl ApplicationImpl for GsrApplication {
    fn activate(&self) {
        self.parent_activate();
    }
}

impl GtkApplicationImpl for GsrApplication {}


