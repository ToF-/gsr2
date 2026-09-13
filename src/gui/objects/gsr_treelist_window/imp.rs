use crate::model::shared::Shared;
use gtk::glib;
#[allow(deprecated)]
use gtk::subclass::prelude::*;
use gtk::{self};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GsrTreelistWindow {
    pub selected: Shared<String>,
    pub position: Cell<u32>,
}

impl Default for GsrTreelistWindow {
    fn default() -> Self {
        Self {
            selected: Rc::new(RefCell::new(String::new())),
            position: Cell::new(0),
        }
    }
}

impl GsrTreelistWindow {}
#[gtk::glib::object_subclass]
impl ObjectSubclass for GsrTreelistWindow {
    const NAME: &'static str = "GsrTreelistWindow";
    type Type = super::GsrTreelistWindow;
    type ParentType = gtk::Window;
}

impl ObjectImpl for GsrTreelistWindow {}

impl WidgetImpl for GsrTreelistWindow {}

impl WindowImpl for GsrTreelistWindow {}
