use crate::env::default_values::ENTRY_CURSOR_1;
use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::information::information;
use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_entry_window::GsrApplicationWindow;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::glib;
use gtk::glib::subclass::prelude::*;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use std::cell::Cell;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::subclass::prelude::*;

pub struct GsrEntryWindow {
    pub key_input_rc: RefCell<KeyInput>,
    pub cursor_timeout_source_id: RefCell<Option<glib::SourceId>>,
    pub editing: Cell<bool>,
    pub cursor: Cell<char>,
}

impl Default for GsrEntryWindow {
    fn default() -> Self {
        Self {
            key_input_rc: RefCell::new(information()),
            cursor_timeout_source_id: RefCell::new(None),
            editing: Cell::new(false),
            cursor: Cell::new(ENTRY_CURSOR_1),
        }
    }
}

impl GsrEntryWindow {
    pub fn initialize(
        &self,
        application_window: &GsrApplicationWindow,
        main_controller_rc: &RcMainController,
        key_input: KeyInput,
        initial_input_opt: Option<&str>,
    ) {
        *self.key_input_rc.borrow_mut() = key_input.clone();
        let obj = self.obj();
        obj.set_decorated(false);
        obj.set_modal(true);
        obj.set_default_width(ENTRY_WINDOW_WIDTH);
        obj.set_default_height(ENTRY_WINDOW_HEIGHT);
        obj.set_transient_for(Some(application_window));
        let entry_text = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .build();
        if let Some(input) = initial_input_opt {
            entry_text.set_label(&input)
        };
        entry_text.add_css_class("entry");
        let prompt = self.key_input_rc.borrow().prompt();
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
        #[allow(deprecated)]
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
        obj.set_child(Some(&entry_box));

        let main_controller = main_controller_rc.borrow();
        self.obj()
            .insert_action_group("main-controller", Some(&main_controller.gio_action_group()));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GsrEntryWindow {
    const NAME: &'static str = "GsrEntryWindow";
    type Type = super::GsrEntryWindow;
    type ParentType = gtk::Window;
}

impl ObjectImpl for GsrEntryWindow {}

impl WidgetImpl for GsrEntryWindow {}

impl WindowImpl for GsrEntryWindow {}
