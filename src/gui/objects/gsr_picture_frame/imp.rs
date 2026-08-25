use crate::gui::view_state::ViewState;
use crate::model::shared::Shared;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_picture_frame::GsrApplicationWindow;
use crate::gui::view::View;
use crate::model::gallery::Gallery;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GsrPictureFrame {}

impl GsrPictureFrame {
    pub fn gsr_application(&self) -> GsrApplication {
        self.obj()
            .root()
            .unwrap()
            .downcast::<GsrApplicationWindow>()
            .expect("GsrPictureFrame not inside a GsrApplicationWindow")
            .gsr_application()
    }
    pub fn shared_view_state(&self) -> Shared<ViewState> {
        self.gsr_application().shared_view_state()
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
