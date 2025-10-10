use gtk::glib::Propagation;
use crate::gui::event::Event;
use crate::RcController;
use crate::clone;
use gtk::prelude::Cast;
use gtk::prelude::GtkWindowExt;
use gtk::prelude::WidgetExt;
use gtk::prelude::BoxExt;
use gtk::Align;
use crate::env::default_values::{ENTRY_WINDOW_HEIGHT, ENTRY_WINDOW_WIDTH};

use gtk::Orientation;
#[derive(Clone,Debug)]
pub struct EntryWindow {
    window: gtk::Window,
}

impl EntryWindow {
    pub fn new(application_window: &gtk::ApplicationWindow, prompt: &str, text: &str, controller_rc: &RcController) -> Self {
        let entry_text = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(text)
            .build();
        let prompt_label = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(prompt)
            .build();
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
        EntryWindow {
            window: window,
        }
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

    pub fn entry_text(&self) -> String {
        let label: gtk::Label = self.window
            .first_child().expect("can't get first_child")
            .downcast::<gtk::Box>().expect("can't downcast as box")
            .first_child().expect("can't get entry prompt")
            .downcast::<gtk::Label>().expect("can't downcast as label")
            .next_sibling().expect("can't get next label")
            .downcast::<gtk::Label>().expect("can't downcast as label");
        label.label().to_string()
    }

    pub fn popup(&self) {
        self.window.present()
    }

    pub fn window(&self) -> gtk::Window {
        self.window.clone()
    }

    pub fn close(&self) {
        self.window.close()
    }
}




