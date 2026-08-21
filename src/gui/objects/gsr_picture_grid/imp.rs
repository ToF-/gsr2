use crate::gui::navigator::Navigator;
use crate::gui::view::View;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GsrPictureGrid {}

impl GsrPictureGrid {}

#[glib::object_subclass]
impl ObjectSubclass for GsrPictureGrid {
    const NAME: &'static str = "GsrPictureGrid";
    type Type = super::GsrPictureGrid;
    type ParentType = gtk::Grid;
}

impl ObjectImpl for GsrPictureGrid {}

impl WidgetImpl for GsrPictureGrid {}

impl GridImpl for GsrPictureGrid {}
