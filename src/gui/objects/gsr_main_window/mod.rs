use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GsrMainWindow(ObjectSubclass<imp::GsrMainWindow>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GsrMainWindow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn initialize(&self, value: usize) {
        self.imp().initialize(value)
    }
}
