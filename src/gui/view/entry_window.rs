use crate::env::default_values::{ENTRY_CURSOR_1, ENTRY_CURSOR_2};
use crate::env::default_values::{ENTRY_WINDOW_HEIGHT, ENTRY_WINDOW_WIDTH};
use crate::gui::controller::RcController;
use crate::gui::event::Event;
use crate::gui::mode::Mode;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::glib::{clone, timeout_add_local};
use gtk::glib::{ControlFlow, Propagation};
use gtk::prelude::BoxExt;
use gtk::prelude::Cast;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct EntryWindow {
    window: gtk::Window,
}

#[allow(deprecated)]
impl EntryWindow {
    pub fn new(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        text: &str,
        controller_rc: &RcController,
    ) -> Self {
        let entry_text = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(text)
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
                padding: 1px;
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
        Self::attach_key_pressed_event_handler(&window, controller_rc);
        Self::attach_cursor_blink_event(&window, controller_rc);
        EntryWindow { window }
    }

    fn attach_key_pressed_event_handler(window: &gtk::Window, controller_rc: &RcController) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong]
            controller_rc,
            move |_, key, key_code, modifier_type| {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller.process_event(
                        Event::KeyPressed {
                            key,
                            key_code,
                            modifier_type,
                        },
                        &controller_rc,
                    );
                };
                Propagation::Stop
            }
        ));
        window.add_controller(event_controller_key);
    }
    fn attach_cursor_blink_event(window: &gtk::Window, controller_rc: &RcController) {
        let delay: u64 = 1;
        timeout_add_local(
            Duration::new(delay, 0),
            clone!(
                #[strong]
                window,
                #[strong]
                controller_rc,
                move || {
                    if let Ok(controller) = controller_rc.try_borrow() {
                        if Mode::Editing == controller.state().mode() {
                            Self::append_cursor(&window);
                            ControlFlow::Continue
                        } else {
                            ControlFlow::Break
                        }
                    } else {
                        panic!("can't borrow mut controller")
                    }
                }
            ),
        );
    }

    fn entry_text_label(window: &gtk::Window) -> gtk::Label {
        let label: gtk::Label = window
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
            .expect("can't downcast as label");
        label
    }

    fn prompt_label(window: &gtk::Window) -> gtk::Label {
        let label: gtk::Label = window
            .first_child()
            .expect("can't get first_child")
            .downcast::<gtk::Box>()
            .expect("can't downcast as box")
            .first_child()
            .expect("can't get entry prompt")
            .downcast::<gtk::Label>()
            .expect("can't downcast as label");
        label
    }

    pub fn entry_text(&self) -> gtk::Label {
        Self::entry_text_label(&self.window)
    }

    pub fn set_text(&self, text: &str) {
        self.entry_text().set_text(text);
        Self::append_cursor(&self.window);
    }

    pub fn set_prompt(&self, prompt: &str) {
        Self::prompt_label(&self.window).set_text(prompt);
    }

    fn append_cursor(window: &gtk::Window) {
        let label = Self::entry_text_label(window);
        let mut content = label.text().to_string();
        let last_char = content.pop();
        match last_char {
            None => content.push(ENTRY_CURSOR_1),
            Some(ENTRY_CURSOR_1) => content.push(ENTRY_CURSOR_2),
            Some(ENTRY_CURSOR_2) => content.push(ENTRY_CURSOR_1),
            Some(other) => {
                content.push(other);
                content.push(ENTRY_CURSOR_1)
            }
        }
        label.set_text(&content);
    }

    pub fn popup(&self) {
        self.window.present()
    }

    pub fn close(&self) {
        self.window.close()
    }
}
