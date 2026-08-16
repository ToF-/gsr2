use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_entry_window::EntryEditor;
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

#[derive(Default)]
pub struct GsrEntryWindow {
    pub prompt_rc: RefCell<String>,
    pub candidates_rc: RefCell<String>,
    pub input_rc: RefCell<String>,
    pub entry_editor_opt_rc: RefCell<Option<EntryEditor>>,
}

impl GsrEntryWindow {
    pub fn initialize(
        &self,
        application_window: &gtk::ApplicationWindow,
        main_controller_rc: &RcMainController,
        prompt: &str,
        input: &str,
        entry_editor_opt: Option<EntryEditor>,
    ) {
        *self.prompt_rc.borrow_mut() = prompt.to_owned();
        *self.input_rc.borrow_mut() = input.to_owned();
        *self.candidates_rc.borrow_mut() = "".to_owned();
        *self.entry_editor_opt_rc.borrow_mut() = entry_editor_opt;
        let obj = self.obj();
        obj.set_decorated(false);
        obj.set_modal(true);
        obj.set_default_width(ENTRY_WINDOW_WIDTH);
        obj.set_default_height(ENTRY_WINDOW_HEIGHT);
        obj.set_transient_for(Some(application_window));
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
        // with no editor => first pressed key causes a closing
        if self.entry_editor_opt_rc.borrow().is_some() {
            todo!();
        } else {
            Self::connect_key_pressed_to_close(&obj);
        }
        let main_controller = main_controller_rc.borrow();
        self.obj()
            .insert_action_group("main-controller", Some(&main_controller.gio_action_group()));
    }

    pub fn connect_key_pressed_to_close(gsr_entry_window: &super::GsrEntryWindow) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong (rename_to=this)]
            gsr_entry_window,
            move |_, _, _, _| {
                let action_call = GioAction::from(Action::Dismiss).to_simple_action_call();
                let name = action_call.0.clone();
                let variant = action_call.1.clone();
                let variant_ref: Option<&Variant> = match &variant {
                    None => None,
                    Some(v) => Some(v.as_ref()),
                };
                let _ = this.activate_action(&name, variant_ref);
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

impl ObjectImpl for GsrEntryWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.set_title(Some("My Custom Window"));
        obj.set_default_size(800, 600);

        let label = gtk::Label::new(Some("Hello from subclass"));

        obj.set_child(Some(&label));
    }
}

impl WidgetImpl for GsrEntryWindow {}

impl WindowImpl for GsrEntryWindow {}
