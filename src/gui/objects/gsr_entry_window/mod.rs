use crate::gui::main_controller::MainController;
use crate::gui::editor::Editor;
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
        main_controller: &MainController,
        prompt: &str,
        input: &str,
        editor_opt: Option<Editor>,
    ) -> Self {
        let obj = Self::new();
        obj.imp()
            .initialize(application_window, main_controller, prompt, input, editor_opt);
        obj
    }
}
