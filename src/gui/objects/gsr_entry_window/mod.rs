use crate::env::default_values::ENTRY_CURSOR_1;
use crate::env::default_values::ENTRY_CURSOR_2;
use crate::gui::action::gio_action::GioAction;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::key_input_rules::KeyInputRules;
use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use glib::Variant;
use gtk::glib;
use gtk::glib::Propagation;
use gtk::glib::clone;
use gtk::prelude::Cast;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use std::time::Duration;

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
            key_input.clone(),
            initial_input_opt,
        );
        Self::connect_key_pressed_controller(&obj);
        /* does not work properly with key_pressed
        if key_input.clone().key_input_mode() == KeyInputMode::Entry {
            Self::attach_cursor_blink_event(&obj);
        }
        */
        obj
    }

    pub fn set_prompt_text(&self, text: &str) {
        self.first_child()
            .expect("child is not set")
            .downcast::<gtk::Box>()
            .expect("child is not a Box")
            .first_child()
            .expect("box has no prompt")
            .downcast::<gtk::Label>()
            .expect("prompt is not a label")
            .set_label(text);
    }

    pub fn entry(&self) -> gtk::Label {
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
    }

    pub fn entry_text(&self) -> String {
        self.entry().label().to_string()
    }
    pub fn set_entry_text(&self, text: &str) {
        self.entry().set_label(text);
        self.imp().editing.set(false);
    }
    pub fn attach_cursor_blink_event(gsr_entry_window: &Self) -> glib::SourceId {
        glib::timeout_add_local(
            Duration::from_millis(500),
            clone!(
                #[strong]
                gsr_entry_window,
                move || {
                    let editing = gsr_entry_window.imp().editing.get();
                    println!("editing:{}", editing);
                    if !editing {
                        Self::append_cursor(&gsr_entry_window);
                        gtk::glib::ControlFlow::Continue
                    } else {
                        Self::remove_cursor(&gsr_entry_window);
                        gtk::glib::ControlFlow::Break
                    }
                }
            ),
        )
    }

    fn append_cursor(&self) {
        let cursor = self.imp().cursor.get();
        let mut content = self.entry_text();
        let last_char = content.pop();
        match last_char {
            None => content.push(cursor),
            Some(ENTRY_CURSOR_1) => content.push(ENTRY_CURSOR_2),
            Some(ENTRY_CURSOR_2) => content.push(ENTRY_CURSOR_1),
            Some(other) => {
                content.push(other);
                content.push(cursor);
            }
        }
        self.set_entry_text(&content);
        self.imp().cursor.set(match cursor {
            ENTRY_CURSOR_1 => ENTRY_CURSOR_2,
            ENTRY_CURSOR_2 => ENTRY_CURSOR_1,
            _ => ENTRY_CURSOR_1,
        })
    }

    fn remove_cursor(&self) {
        let content = self.entry_text();
        let new_content: String = content
            .chars()
            .filter(|c| *c != ENTRY_CURSOR_1 && *c != ENTRY_CURSOR_2)
            .collect();
        self.set_entry_text(&new_content);
    }

    pub fn connect_key_pressed_controller(gsr_entry_window: &Self) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong (rename_to=this)]
            gsr_entry_window,
            move |_, key, _, _| {
                this.remove_cursor();
                let key_input = this.imp().key_input_rc.borrow();
                let input = this.entry_text();
                let status = key_input.edit(&input, key);
                let action_opt = status.result_action();
                if let Some(action) = action_opt {
                    let action_call = GioAction::from(action.clone()).to_simple_action_call();
                    let name = action_call.0.clone();
                    let variant = action_call.1.clone();
                    let variant_ref: Option<&Variant> = match &variant {
                        None => None,
                        Some(v) => Some(v.as_ref()),
                    };
                    dbg!(&action.clone(), &name.clone(), &variant.clone());
                    match this.activate_action(&name, variant_ref) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "connect_key_pressed_controller for gsr_entry_window {} {:?} : {}",
                                name, variant_ref, e
                            )
                        }
                    }
                } else {
                    if let Some(list_tip) = status.candidate_list_tip() {
                        this.set_prompt_text(&list_tip);
                    } else {
                        let key_input = this.imp().key_input_rc.borrow();
                        this.set_prompt_text(&key_input.prompt());
                    }
                    let new_input = status.input();
                    this.set_entry_text(&new_input);
                    this.append_cursor();
                }
                Propagation::Stop
            }
        ));
        gsr_entry_window.add_controller(event_controller_key);
    }
}
