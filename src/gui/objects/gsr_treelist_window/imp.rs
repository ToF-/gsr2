use std::cell::Cell;
use crate::env::default_values::TREELIST_WINDOW_HEIGHT;
use crate::env::default_values::TREELIST_WINDOW_WIDTH;
use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::model::catalog::Catalog;
use crate::model::shared::Shared;
use crate::model::sub_category::SubCategory;
use glib::BoxedAnyObject;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::gdk::Display;
use gtk::glib::{Propagation, clone};
use gtk::prelude::BoxExt;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{self};
use gtk::{
    Label, ListItem, ListView, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    glib,
};
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
