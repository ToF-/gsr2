use crate::Controller;
use gtk::Align;
use gtk::Orientation;
use gtk::Picture as GtkPicture;
use gtk::prelude::BoxExt;
use gtk::prelude::WidgetExt;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct PictureFrame {
    frame_ref: RefCell<gtk::Box>,
}

impl PictureFrame {
    pub fn new_from_frame(frame: &gtk::Box) -> Self {
        PictureFrame {
            frame_ref: RefCell::new(frame.clone()),
        }
    }

    pub fn new() -> Self {
        let picture = make_picture();
        let frame = make_frame();
        let label = make_label();
        frame.append(&picture);
        frame.append(&label);
        PictureFrame {
            frame_ref: RefCell::new(frame),
        }
    }

    pub fn frame(&self) -> gtk::Box {
        self.frame_ref.borrow().clone()
    }

    pub fn set_picture(&self, controller: &Controller, picture: &gtk::Picture) {
        let frame: gtk::Box = self.frame();
        while let Some(child) = frame.first_child() {
            frame.remove(&child)
        }
        let state = controller.state();
        if state.expand_on() {
            picture.set_valign(Align::Fill);
            picture.set_halign(Align::Fill);
        } else {
            picture.set_valign(Align::Center);
            picture.set_halign(Align::Center);
        };
        picture.set_can_shrink(!state.full_size_on());
        frame.append(picture);
    }
}

fn make_frame() -> gtk::Box {
    gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .homogeneous(false)
        .build()
}

fn make_picture() -> gtk::Picture {
    GtkPicture::builder().hexpand(true).vexpand(true).build()
}

pub fn make_label() -> gtk::Label {
    let label = gtk::Label::new(None);
    label.set_valign(Align::Center);
    label.set_halign(Align::Center);
    label
}
