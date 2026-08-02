use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use crate::gui::editor::entry_editor::EntryEditor;
use crate::gui::editor::entry_editor::RcEntryEditor;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use gtk::glib::{ControlFlow, Propagation};
use gtk::glib::{clone, timeout_add_local};
use gtk::prelude::BoxExt;
use gtk::prelude::Cast;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::cell::RefCell;
use std::sync::OnceLock;

pub struct EntryView {
    gtk_window_opt_rc: RefCell<Option<gtk::Window>>,
}

impl Default for EntryView {
    fn default() -> Self {
        Self {
            gtk_window_opt_rc: RefCell::new(None),
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
    pub fn initialize(
        &self,
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        input: &str,
    ) {
        *self.gtk_window_opt_rc.borrow_mut() =
            Some(Self::build_window(application_window, prompt, input))
    }

    pub fn input(&self) -> String {
        self.gtk_window_opt_rc
            .borrow()
            .as_ref()
            .expect("entry_view doesn't have an attached gtk window yet")
            .first_child()
            .expect("can't get first_child")
            .downcast::<gtk::Box>()
            .expect("can't downcast as box")
            .first_child()
            .expect("can't get entry prompt")
            .downcast::<gtk::Label>()
            .expect("can't downcast as label")
            .next_sibling()
            .expect("can't get next label")
            .downcast::<gtk::Label>()
            .expect("can't downcast as label")
            .text()
            .to_string()
    }

    pub fn set_input(&self, text: &str) {
        self.gtk_window_opt_rc
            .borrow()
            .as_ref()
            .expect("entry_view doesn't have an attached gtk window yet")
            .first_child()
            .expect("can't get first_child")
            .downcast::<gtk::Box>()
            .expect("can't downcast as box")
            .first_child()
            .expect("can't get entry prompt")
            .downcast::<gtk::Label>()
            .expect("can't downcast as label")
            .next_sibling()
            .expect("can't get next label")
            .downcast::<gtk::Label>()
            .expect("can't downcast as label")
            .set_text(text);
    }

    pub fn prompt(&self) -> String {
        self.gtk_window_opt_rc
            .borrow()
            .as_ref()
            .expect("entry_view doesn't have an attached gtk window yet")
            .first_child()
            .expect("can't get first_child")
            .downcast::<gtk::Box>()
            .expect("can't downcast as box")
            .first_child()
            .expect("can't get entry prompt")
            .downcast::<gtk::Label>()
            .expect("can't downcast as label")
            .text()
            .to_string()
    }

    pub fn set_prompt(&self, text: &str) {
        self.gtk_window_opt_rc
            .borrow()
            .as_ref()
            .expect("entry_view doesn't have an attached gtk window yet")
            .first_child()
            .expect("can't get first_child")
            .downcast::<gtk::Box>()
            .expect("can't downcast as box")
            .first_child()
            .expect("can't get entry prompt")
            .downcast::<gtk::Label>()
            .expect("can't downcast as label")
            .set_text(text)
    }

    pub fn build_window(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        input: &str,
    ) -> gtk::Window {
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
        window
    }

    pub fn attach_key_pressed_editor(
        &self,
        entry_editor_rc: &std::cell::RefCell<EntryEditor>,
    ) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong]
            entry_editor_rc,
            move |_, key, key_code, modifier_type| {
                if let Ok(entry_editor) = entry_editor_rc.try_borrow() {
                    if let Some(key_name) = key.name() {
                        entry_editor.key_pressed(&key_name);
                    }
                }
                Propagation::Stop
            }
        ));
        self.gtk_window_opt_rc
            .borrow()
            .as_ref()
            .expect("entry_view doesn't have an attached gtk window yet")
            .add_controller(event_controller_key);
    }

    pub fn present(&self) {
        self.gtk_window_opt_rc
            .borrow()
            .as_ref()
            .expect("entry_view doesn't have an attached gtk window yet")
            .present()
    }
    pub fn close(&self) {
        self.gtk_window_opt_rc
            .borrow()
            .as_ref()
            .expect("entry_view doesn't have an attached gtk window yet")
            .close()
    }
}
