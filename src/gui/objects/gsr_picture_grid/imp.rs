use std::cell::Cell;
use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct GsrPictureGrid {
    pub focus_at_coords: Cell<(i32, i32)>,
}

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
