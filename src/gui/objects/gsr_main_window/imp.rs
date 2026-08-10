use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

#[derive(Default)]
pub struct GsrMainWindow {}

impl GsrMainWindow {
    pub fn initialize(&self, value: usize) {}
}

#[glib::object_subclass]
impl ObjectSubclass for GsrMainWindow {
    const NAME: &'static str = "GsrMainWindow";
    type Type = super::GsrMainWindow;
    type ParentType = gtk::Window;
}

impl ObjectImpl for GsrMainWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.set_title(Some("My Custom Window"));
        obj.set_default_size(800, 600);

        let label = gtk::Label::new(Some("Hello from subclass"));

        obj.set_child(Some(&label));
    }
}

impl WidgetImpl for GsrMainWindow {}

impl WindowImpl for GsrMainWindow {}
