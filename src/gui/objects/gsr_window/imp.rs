use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

#[derive(Default)]
pub struct GsrWindow {
    foo: Cell<usize>,
}

impl GsrWindow {
    pub fn initialize(&self, value: usize) {
        self.foo.set(value);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GsrWindow {
    const NAME: &'static str = "GsrWindow";
    type Type = super::GsrWindow;
    type ParentType = gtk::Window;
}

impl ObjectImpl for GsrWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.set_title(Some("My Custom Window"));
        obj.set_default_size(800, 600);

        let label = gtk::Label::new(Some("Hello from subclass"));

        obj.set_child(Some(&label));
    }
}

impl WidgetImpl for GsrWindow {}

impl WindowImpl for GsrWindow {}
