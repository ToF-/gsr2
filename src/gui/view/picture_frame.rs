use crate::Controller;
use crate::env::default_values::{FRAME_PALETTE_AREA_HEIGHT, FRAME_PALETTE_AREA_WIDTH};
use crate::gui::view::palette_area::make_palette_area;
use gtk::Align;
use gtk::Orientation;
use gtk::Picture as GtkPicture;
use gtk::prelude::BoxExt;
use gtk::prelude::WidgetExt;

#[derive(Clone, Debug)]
pub struct PictureFrame {
    frame: gtk::Box,
}

impl PictureFrame {
    pub fn new_from_frame(frame: &gtk::Box) -> Self {
        PictureFrame {
            frame: frame.clone(),
        }
    }

    pub fn new() -> Self {
        let picture = make_picture();
        let frame = make_frame();
        let label = make_label();
        frame.append(&picture);
        frame.append(&label);
        PictureFrame { frame }
    }

    pub fn frame(&self) -> gtk::Box {
        self.frame.clone()
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
        if controller.state().palette_on() {
            if let Some(image_data) = controller.current_picture().image_data() {
                let palette_area = make_palette_area(
                    image_data.palette().sample(),
                    FRAME_PALETTE_AREA_WIDTH,
                    FRAME_PALETTE_AREA_HEIGHT,
                );
                frame.append(&palette_area)
            }
        }
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
