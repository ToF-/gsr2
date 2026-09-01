use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::model::catalog::Catalog;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;

mod imp;

glib::wrapper! {
    pub struct GsrTreelistWindow(ObjectSubclass<imp::GsrTreelistWindow>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GsrTreelistWindow {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn new_with(
        application_window: &GsrApplicationWindow,
        main_controller_rc: &RcMainController,
        catalog: &Catalog,
        prompt: &str,
        initial_item_opt: Option<&str>,
    ) -> Self {
        let obj = Self::new();
        obj.imp().initialize(
            application_window,
            main_controller_rc,
            catalog,
            prompt,
            initial_item_opt,
        );
        obj
    }
}
