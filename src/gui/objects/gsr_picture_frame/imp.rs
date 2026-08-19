use crate::model::gallery::Gallery;
use crate::gui::navigator::Navigator;
use crate::gui::view::View;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;

#[derive(Default)]
pub struct GsrPictureFrame {
    pub view: RefCell<View>,
    pub navigator: RefCell<Navigator>,
    pub gallery: RefCell<Gallery>,
}

#[glib::object_subclass]
impl ObjectSubclass for GsrPictureFrame {
    const NAME: &'static str = "GsrPictureFrame";
    type Type = super::GsrPictureFrame;
    type ParentType = gtk::Box;
}

impl ObjectImpl for GsrPictureFrame {}
impl WidgetImpl for GsrPictureFrame {}
impl BoxImpl for GsrPictureFrame {}
