use crate::gui::editor::entry_editor::EntryEditor;
use crate::gui::action::Action;
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
    pub fn new_with(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        input: &str,
        action_on_close: Action,
    ) -> Self {
        let obj = Self::new();
        obj.imp().initialize(
            application_window,
            prompt,
            input,
            action_on_close,
        );
        obj
    }

    pub fn present(&self) {
        self.imp().present()
    }

    pub fn close(&self) {
        self.imp().close()
    }

    pub fn input(&self) -> String {
        self.imp().input()
    }

    pub fn set_input(&self, text: &str) {
        self.imp().set_input(text);
    }

    pub fn prompt(&self) -> String {
        self.imp().prompt()
    }

    pub fn set_prompt(&self, text: &str) {
        self.imp().set_prompt(text)
    }

    pub fn attach_key_pressed_editor(
        &self,
        entry_editor_rc: &std::cell::RefCell<EntryEditor>,
        blink: bool,
    ) {
        self.imp().attach_key_pressed_editor(entry_editor_rc, blink);
    }
}
