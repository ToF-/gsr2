use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_picture_frame::GsrApplicationWindow;
use crate::gui::view::View;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GsrPictureFrame {}

impl GsrPictureFrame {
    pub fn shared_view(&self) -> Rc<RefCell<View>> {
        self.obj()
            .root()
            .unwrap()
            .downcast::<GsrApplicationWindow>()
            .expect("GsrPictureFrame not inside a GsrApplicationWindow")
            .imp()
            .shared_view()
    }
    pub fn shared_gallery(&self) -> Rc<RefCell<Gallery>> {
        self.obj()
            .root()
            .unwrap()
            .downcast::<GsrApplicationWindow>()
            .expect("GsrPictureFrame not inside a GsrApplicationWindow")
            .imp()
            .shared_gallery()
    }
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
