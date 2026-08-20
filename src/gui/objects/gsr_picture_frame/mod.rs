use crate::env::default_values::FRAME_PALETTE_AREA_HEIGHT;
use crate::env::default_values::FRAME_PALETTE_AREA_WIDTH;
use crate::gui::controller::Controller;
use crate::gui::main_controller::MainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::view::palette_area::make_palette_area;
use gtk::Align;
use gtk::Orientation;
use gtk::Picture as GtkPicture;
use gtk::gio::File as GtkFile;
use gtk::prelude::BoxExt;
use gtk::prelude::WidgetExt;
use std::path::Path;

use crate::file::paths::check_path_exists;
use crate::gui::navigator::Navigator;
use crate::gui::view::View;
use crate::gui::view::legacy_main_view::gtk_picture_from_file_path;
use crate::model::gallery::Gallery;
use crate::model::picture::Picture;
use crate::model::shared::Shared;
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
    pub fn new(view: Shared<View>, navigator: Shared<Navigator>, gallery: Shared<Gallery>) -> Self {
        let obj: Self = glib::Object::builder()
            .property("orientation", Orientation::Vertical)
            .property("spacing", 0)
            .property("halign", Align::Fill)
            .property("valign", Align::Fill)
            .property("hexpand", true)
            .property("vexpand", true)
            .property("homogeneous", false)
            .build();

        obj.imp().initialize(view, navigator, gallery);
        obj
    }

    fn remove_children(&self) {
        while let Some(child) = self.first_child() {
            self.remove(&child);
        }
    }

    fn add_chidren(&self) {
        let picture = make_picture();
        let label = make_label();
        self.append(&picture);
        self.append(&label);
    }

    fn get_application_window(&self) -> Option<GsrApplicationWindow> {
        self.root()
            .and_then(|root| root.downcast::<GsrApplicationWindow>().ok())
    }

    pub fn gtk_picture_from_file_path(file_path: &Path) -> gtk::Picture {
        GtkPicture::builder()
            .file(&GtkFile::for_path(file_path))
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    pub fn set_gtk_picture(&self, gtk_picture: gtk::Picture) {
        self.remove_children();
        let view = self.imp().view();
        if view.expand_on() {
            gtk_picture.set_valign(Align::Fill);
            gtk_picture.set_halign(Align::Fill);
        } else {
            gtk_picture.set_valign(Align::Center);
            gtk_picture.set_halign(Align::Center);
        };

        gtk_picture.set_can_shrink(!view.full_size_on());
        self.append(&gtk_picture);
    }

    pub fn set_palette_area(&self, gtk_drawing_area: gtk::DrawingArea) {
        self.append(&gtk_drawing_area)
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
            let view = self.imp().view();
            if view.palette_on()
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
        let gallery = self.imp().gallery();
        let picture_opt = if gallery.len() > 0 {
            Some(gallery.current_picture())
        } else {
            None
        };
        self.set_picture(picture_opt);
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
