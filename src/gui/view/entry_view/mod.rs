use crate::gui::entry_controller::EntryController;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::subclass::prelude::ObjectSubclassIsExt;
mod imp;

gtk::glib::wrapper! {
    pub struct EntryView(ObjectSubclass<imp::EntryView>);
}

impl EntryView {
    pub fn new() -> Self {
        gtk::glib::Object::new()

    }
    pub fn new_with(application_window: &gtk::ApplicationWindow, prompt: &str, input: &str) -> Self {
        let obj = Self::new();
        obj.imp().initialize(application_window, prompt, input);
        obj
    }

    pub fn present(&self) {
        self.imp().present()
    }

    pub fn close(&self) {
        self.imp().close()
    }

    pub fn attach_key_pressed_event_handler(&self, entry_controller_rc: &std::cell::RefCell<EntryController>) {
        self.imp().attach_key_pressed_event_handler(entry_controller_rc);
    }
}
