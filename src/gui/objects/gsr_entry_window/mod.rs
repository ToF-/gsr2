use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::key_input::KeyInput;
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use gtk::glib;
use gtk::prelude::Cast;
use gtk::prelude::WidgetExt;
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
        application_window: &GsrApplicationWindow,
        main_controller_rc: &RcMainController,
        key_input: KeyInput,
        initial_input_opt: Option<&str>,
    ) -> Self {
        let obj = Self::new();
        obj.imp().initialize(
            application_window,
            main_controller_rc,
            key_input,
            initial_input_opt,
        );
        obj
    }

    pub fn entry_text(&self) -> String {
        self.first_child()
            .expect("child is not set")
            .downcast::<gtk::Box>()
            .expect("child is not a Box")
            .first_child()
            .expect("box has no prompt")
            .next_sibling()
            .expect("box has no entry")
            .downcast::<gtk::Label>()
            .expect("entry is not a label")
            .label()
            .to_string()
    }
    pub fn set_entry_text(&self, text: &str) {
        self.first_child()
            .expect("child is not set")
            .downcast::<gtk::Box>()
            .expect("child is not a Box")
            .first_child()
            .expect("box has no prompt")
            .next_sibling()
            .expect("box has no entry")
            .downcast::<gtk::Label>()
            .expect("entry is not a label")
            .set_label(text)
    }
}
