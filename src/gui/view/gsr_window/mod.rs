use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GsrWindow(ObjectSubclass<imp::GsrWindow>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GsrWindow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn initialize(&self, value: usize) {
        self.imp().initialize(value)
    }
}
