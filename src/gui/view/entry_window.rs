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
    pub fn new(application_window: &gtk::ApplicationWindow, prompt: &str, text: &str) -> Self {
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
            .default_width(ENTRY_WINDOW_WIDTH)
            .default_height(ENTRY_WINDOW_HEIGHT)
            .transient_for(application_window)
            .build();
        window.set_child(Some(&entry_box));
        EntryWindow {
            window: window,
        }
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
}




