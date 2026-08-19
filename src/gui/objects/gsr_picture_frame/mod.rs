use crate::gui::navigator::Navigator;
use crate::gui::view::View;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

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
        let obj: Self = glib::Object::new();
        obj.imp().initialize(view, navigator, gallery);
        obj

    }
}
