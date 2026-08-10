use gtk::glib::Properties;
use std::cell::Cell;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default, Properties)]
#[properties(wrapper_type = super::GsrPictureCellBox)]
pub struct GsrPictureCellBox {
       #[property(get, set)]
    col: Cell<i32>,

    #[property(get, set)]
    row: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for GsrPictureCellBox {
    const NAME: &'static str = "GsrPictureCellBox";
    type Type = super::GsrPictureCellBox;
    type ParentType = gtk::Box;
}

impl ObjectImpl for GsrPictureCellBox {}

impl WidgetImpl for GsrPictureCellBox {}

impl BoxImpl for GsrPictureCellBox {}

