use gtk::glib
use gtk::subclasse::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GsrEntryWindow(ObjectSubclass<imp::GsrMainWindow>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GsrEntryWindow {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn new_with(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        input: &str,
    ) {
        let obj = Self::new();
        obj.imp()
            .initialize(application_window, prompt, input);
        obj
    }

}
