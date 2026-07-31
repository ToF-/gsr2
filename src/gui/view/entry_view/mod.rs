use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use crate::gui::entry_controller::RcEntryController;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::glib::{ControlFlow, Propagation};
use gtk::glib::{clone, timeout_add_local};
use gtk::prelude::BoxExt;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
mod imp;
use crate::gui::entry_controller::EntryController;
use gtk::glib::subclass::prelude::*;
use gtk::prelude::ObjectExt;
use std::cell::RefCell;
use std::rc::Rc;

gtk::glib::wrapper! {
    pub struct EntryView(ObjectSubclass<imp::EntryView>);
}

impl EntryView {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn build_ui(
        &mut self,
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        input: &str,
        entry_controller_rc: &RcEntryController,
    ) {
        self.imp().build_ui(application_window, prompt, input, entry_controller_rc);
    }

    pub fn present(&self) {
        self.imp().gtk_window().present()
    }


    pub fn close(&self) {
        self.imp().gtk_window().close()
    }


}
