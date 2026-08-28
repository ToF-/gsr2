use crate::gui::view_mode::ViewMode;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_application_window::picture_opacity;
use crate::gui::view::gtk_picture_from_file_path;
use gtk::Picture as GtkPicture;

use crate::env::default_values::FRAME_PALETTE_AREA_HEIGHT;
use crate::env::default_values::FRAME_PALETTE_AREA_WIDTH;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::view::palette_area::make_palette_area;
use gtk::Align;
use gtk::Orientation;
use gtk::gio::File as GtkFile;
use gtk::prelude::BoxExt;
use std::path::Path;

use crate::file::paths::check_path_exists;
use crate::model::picture::Picture;
use crate::model::thumbnail::no_thumbnail_picture;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::path::PathBuf;

mod imp;

glib::wrapper! {
    pub struct GsrPictureFrame(ObjectSubclass<imp::GsrPictureFrame>)
        @extends gtk::Widget, gtk::Box,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::Orientable,
            gtk::ConstraintTarget;
}

impl GsrPictureFrame {
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder()
            .property("orientation", Orientation::Vertical)
            .property("spacing", 0)
            .property("halign", Align::Fill)
            .property("valign", Align::Fill)
            .property("hexpand", true)
            .property("vexpand", true)
            .property("homogeneous", false)
            .build();
        obj
    }

    pub fn gsr_application(&self) -> GsrApplication {
        self.root()
            .and_then(|root| root.downcast::<GsrApplicationWindow>().ok())
            .expect("GsrPictureGrid is not inside a Window")
            .gsr_application()
    }

    fn remove_children(&self) {
        while let Some(child) = self.first_child() {
            self.remove(&child);
        }
    }

    pub fn gtk_picture_from_file_path(file_path: &Path) -> gtk::Picture {
        GtkPicture::builder()
            .file(&GtkFile::for_path(file_path))
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    pub fn set_gtk_picture(&self, gtk_picture: gtk::Picture) {
        let shared_view_state = self.imp().shared_view_state();
        let view_state = shared_view_state.borrow();
        self.remove_children();
        match view_state.settings.single_view_mode() {
            ViewMode::Normal => {
                gtk_picture.set_valign(Align::Center);
                gtk_picture.set_halign(Align::Center);
            }, 
            ViewMode::Expanded => {
                gtk_picture.set_valign(Align::Fill);
                gtk_picture.set_halign(Align::Fill);
            }, 
            ViewMode::FullSize => {
                gtk_picture.set_valign(Align::Center);
                gtk_picture.set_halign(Align::Center);
                gtk_picture.set_can_shrink(!view_state.settings.full_size_on());
            }
        };
        self.append(&gtk_picture);
    }

    pub fn set_palette_area(&self, gtk_drawing_area: gtk::DrawingArea) {
        self.append(&gtk_drawing_area)
    }

    pub fn set_picture_opacity(&self, opacity: f64) {
        self.set_opacity(opacity);
    }

    pub fn set_picture(&self, picture_opt: Option<Picture>) {
        if let Some(picture) = picture_opt {
            let picture_file_path = picture.file_path();
            let gtk_picture = if let Ok(file_path) =
                check_path_exists(&PathBuf::from(picture_file_path.clone()))
            {
                gtk_picture_from_file_path(file_path)
            } else {
                no_thumbnail_picture()
            };
            self.set_gtk_picture(gtk_picture);
            let shared_view_state = self.imp().shared_view_state();
            let view_state = shared_view_state.borrow();
            if view_state.settings.palette_on()
                && let Some(image_data) = picture.image_data()
            {
                let palette_area = make_palette_area(
                    image_data.palette().sample(),
                    FRAME_PALETTE_AREA_WIDTH,
                    FRAME_PALETTE_AREA_HEIGHT,
                );
                self.set_palette_area(palette_area);
            }
        } else {
            let gtk_picture = no_thumbnail_picture();
            self.set_gtk_picture(gtk_picture);
        }
    }

    pub fn set_current_picture(&self) {
        let (position, selected) = {
            let shared_view_state = self.gsr_application().shared_view_state();
            let view_state = shared_view_state.borrow();
            let position = view_state.navigator.position();
            let selected = view_state.selection.contains(position);
            (position, selected)
        };
        let picture_opt = {
            let shared_view_state = self.gsr_application().shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            view_state.gallery.set_current_picture_index(position);
            if view_state.gallery.len() > 0 {
                Some(view_state.gallery.current_picture())
            } else {
                None
            }
        };
        self.set_picture(picture_opt);
        self.set_picture_opacity(picture_opacity(selected));
    }
}

pub fn make_label() -> gtk::Label {
    let label = gtk::Label::new(None);
    label.set_valign(Align::Center);
    label.set_halign(Align::Center);
    label
}
