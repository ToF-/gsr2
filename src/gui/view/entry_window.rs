use gtk::prelude::BoxExt;
use gtk::Align;
use crate::env::default_values::{ENTRY_WINDOW_HEIGHT, ENTRY_WINDOW_WIDTH};

use gtk::Orientation;
#[derive(Clone,Debug)]
pub struct EntryWindow {
    entry_window: gtk::Window,
    digits_only: bool,
}

impl EntryWindow {
    pub fn new(application_window: &gtk::ApplicationWindow, prompt: &str, text: &str, digits_only: bool) -> Self {
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
        let entry_window = gtk::Window::builder()
            .content_width(ENTRY_WINDOW_WIDTH)
            .content_height(ENTRY_WINDOW_HEIGHT)
            .transient_for(application_window)
            .build();
        entry_window.set_child(&entry_box);
        EntryWindow {
            entry_window: entry_window,
            digits_only,
        }
    }

    pub fn entry_text(&self) -> String {
        let label = self.entry_window.first_child()
            .unwrap().expect("can't get entry window's child")
            .downcast::<gtk::Box>().expect("can't downcast as box")
            .first_child()
            .unwrap().expect("can't get entry prompt")
            .downcast::<gtk::Label>().expect("can't downcast as label")
            .next_sibling()
            .unwrap().expect("can't get next label")
            .downcast::<gtk::Label>().expect("can't downcast as label")
            .label();
        label
    }
}




