use crate::gui::objects::gsr_application_window::Action;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use crate::gui::objects::gsr_treelist_window::GsrTreelistWindow;
use crate::model::shared::Shared;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GsrApplicationWindow {
    pub gsr_entry_window: Shared<GsrEntryWindow>,
    pub gsr_treelist_window: Shared<GsrTreelistWindow>,
    pub entry_on: Cell<bool>,
    pub last_action: Shared<Action>,
}

impl Default for GsrApplicationWindow {
    fn default() -> Self {
        Self {
            gsr_entry_window: Rc::new(RefCell::new(GsrEntryWindow::new())),
            gsr_treelist_window: Rc::new(RefCell::new(GsrTreelistWindow::new())),
            entry_on: Cell::new(false),
            last_action: Rc::new(RefCell::new(Action::Nothing)),
        }
    }
}
impl GsrApplicationWindow {}
#[gtk::glib::object_subclass]
impl ObjectSubclass for GsrApplicationWindow {
    const NAME: &'static str = "GsrApplicationWindow";
    type Type = super::GsrApplicationWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for GsrApplicationWindow {}

impl WidgetImpl for GsrApplicationWindow {}

impl WindowImpl for GsrApplicationWindow {}

impl ApplicationWindowImpl for GsrApplicationWindow {}
