use crate::gui::key_input::information::information_key_input;
use crate::gui::key_input::KeyInput;
use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::key_input::key_input_mode::KeyInputMode;
use crate::gui::key_input::key_input_rules::KeyInputRules;
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use glib::Variant;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::glib;
use gtk::glib::Propagation;
use gtk::glib::clone;
use gtk::glib::subclass::prelude::*;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use std::cell::RefCell;

use crate::gui::editor::legacy_editor::LegacyEditor;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

pub struct GsrEntryWindow {
    pub key_input_rc: RefCell<KeyInput>,
}

impl Default for GsrEntryWindow {
    fn default() -> Self {
        Self {
            key_input_rc: RefCell::new(information_key_input()),
        }
    }
}

impl GsrEntryWindow {
    pub fn initialize(
        &self,
        application_window: &gtk::ApplicationWindow,
        main_controller_rc: &RcMainController,
        key_input: KeyInput,
        initial_input_opt: Option<&str>,
    ) {
        *self.key_input_rc.borrow_mut() = key_input;
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
        Self::connect_key_pressed_controller(&obj);
        let main_controller = main_controller_rc.borrow();
        self.obj()
            .insert_action_group("main-controller", Some(&main_controller.gio_action_group()));
    }

    pub fn connect_key_pressed_controller(gsr_entry_window: &super::GsrEntryWindow) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong (rename_to=this)]
            gsr_entry_window,
            move |_, key, _, _| {
                let key_input = this.imp().key_input_rc.borrow();
                let input = this.entry_text();
                let status = key_input.edit(&input, key);
                let action_opt = status.result_action();
                if let Some(action) = action_opt {
                    let action_call = GioAction::from(action).to_simple_action_call();
                    let name = action_call.0.clone();
                    let variant = action_call.1.clone();
                    let variant_ref: Option<&Variant> = match &variant {
                        None => None,
                        Some(v) => Some(v.as_ref()),
                    };
                    match this.activate_action(&name, variant_ref) {
                        Ok(_) => {},
                        Err(e) => eprintln!("{}", e),
                    }
                } else {
                    let new_input = status.input();
                    if new_input != input {
                        this.set_entry_text(&new_input)
                    };
                }
                Propagation::Stop
            }
        ));
        gsr_entry_window.add_controller(event_controller_key);
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
