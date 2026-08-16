use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::editor::Editor;
use crate::gui::editor::change_label_editor::change_label_editor;
use crate::gui::editor::display_information_editor::display_information_editor;
use crate::gui::editor::editor_rules::EditorRules;
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

pub struct GsrEntryWindow {
    pub editor_rc: RefCell<Editor>,
}

impl Default for GsrEntryWindow {
    fn default() -> Self {
        Self {
            editor_rc: RefCell::new(display_information_editor()),
        }
    }
}

impl GsrEntryWindow {
    pub fn initialize(
        &self,
        application_window: &gtk::ApplicationWindow,
        main_controller_rc: &RcMainController,
        editor: Editor,
        initial_input_opt: Option<&str>,
    ) {
        *self.editor_rc.borrow_mut() = editor;
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
        let prompt = self.editor_rc.borrow().prompt();
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
                let editor = this.imp().editor_rc.borrow();
                let input = this.entry_text();
                let status = editor.edit(&input, key);
                let action_opt = if editor.editable() {
                    status.result_action()
                }    else {
                    Some(Action::Dismiss)
                };
                if let Some(action) = action_opt {
                    let action_call = GioAction::from(Action::Dismiss).to_simple_action_call();
                    let name = action_call.0.clone();
                    let variant = action_call.1.clone();
                    let variant_ref: Option<&Variant> = match &variant {
                        None => None,
                        Some(v) => Some(v.as_ref()),
                    };
                    let _ = this.activate_action(&name, variant_ref);
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
