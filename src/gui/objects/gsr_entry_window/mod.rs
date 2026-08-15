use crate::gui::editor::entry_editor::EntryEditor;
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GsrEntryWindow(ObjectSubclass<imp::GsrEntryWindow>)
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
        main_controller_rc: &RcMainController,
        prompt: &str,
        input: &str,
        entry_editor_opt: Option<EntryEditor>,
    ) -> Self {
        let obj = Self::new();
        obj.imp().initialize(
            application_window,
            main_controller_rc,
            prompt,
            input,
            entry_editor_opt,
        );
        obj
    }
}
