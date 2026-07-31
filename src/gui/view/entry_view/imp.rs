use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::sync::OnceLock;

use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use crate::gui::entry_controller::EntryController;
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

pub struct EntryView {
    gtk_window_opt: Option<gtk::Window>,
}

impl Default for EntryView {
    fn default() -> Self {
        Self {
            gtk_window_opt: None,
        }
    }
}

#[gtk::glib::object_subclass]
impl ObjectSubclass for EntryView {
    const NAME: &'static str = "EntryView";
    type Type = super::EntryView;
    type ParentType = gtk::glib::Object;
}

impl ObjectImpl for EntryView {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| vec![Signal::builder("closed").build()])
    }
}
impl EntryView {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn gtk_window(&self) -> &gtk::Window {
        if let Some(gtk_window) = &self.gtk_window_opt {
            gtk_window
        } else {
            panic!("entry_view doesn't have an attached gtk window yet")
        }
    }
    pub fn input(&self) -> String {
        if let Some(gtk_window) = self.gtk_window_opt.clone() {
            String::new()
        } else {
            panic!("entry_view doesn't have an attached gtk window yet")
        }
    }

    pub fn set_input(&self, text: &str) {
        if let Some(gtk_window) = &self.gtk_window_opt {
            todo!("set the gtk window input")
        } else {
            panic!("entry_view doesn't have an attached gtk window yet")
        }
    }

    pub fn build_ui(
        &mut self,
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        input: &str,
        entry_controller_rc: &RcEntryController,
    ) {
        let entry_text = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(input)
            .build();
        entry_text.add_css_class("entry");
        let prompt_label = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(prompt)
            .build();
        let prompt_css_provider = CssProvider::new();
        prompt_css_provider.load_from_string(
            "
            label {
                padding: 10px;
                font-size: 16px;
            }
        ",
        );
        prompt_label.style_context().add_provider(
            &prompt_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let entry_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .homogeneous(false)
            .build();
        entry_box.append(&prompt_label);
        entry_box.append(&entry_text);
        let window = gtk::Window::builder()
            .decorated(false)
            .modal(true)
            .default_width(ENTRY_WINDOW_WIDTH)
            .default_height(ENTRY_WINDOW_HEIGHT)
            .transient_for(application_window)
            .build();
        window.set_child(Some(&entry_box));
        let entry_controller: std::cell::RefCell<EntryController> =
            std::cell::RefCell::new(EntryController::new());
        self.gtk_window_opt = Some(window);
        self.attach_key_pressed_event_handler(entry_controller_rc);
    }

    fn attach_key_pressed_event_handler(
        &self,
        entry_controller_rc: &std::cell::RefCell<EntryController>,
    ) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong]
            entry_controller_rc,
            move |_, key, key_code, modfier_type| {
                if let Ok(entry_controller) = entry_controller_rc.try_borrow() {
                    if let Some(key_name) = key.name() {
                        entry_controller.enter(&key_name);
                    }
                }
                Propagation::Stop
            }
        ));
        if let Some(window) = &self.gtk_window_opt {
            window.add_controller(event_controller_key);
        } else {
            panic!("entry_view doesn't have an attached gtk window yet")
        }
    }

    pub fn close(&self) {
        if let Some(window) = &self.gtk_window_opt {
            window.close()
        } else {
            panic!("entry_view doesn't have an attached gtk window yet")
        }
    }
}
