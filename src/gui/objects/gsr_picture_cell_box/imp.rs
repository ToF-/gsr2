use gtk::Label as GtkLabel;
use std::cell::RefCell;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

pub struct GsrPictureCellBox {
    pub col: Cell<i32>,
    pub row: Cell<i32>,
    pub pictures_per_row: Cell<i32>,
    pub palette_on: Cell<bool>,
    pub label: RefCell<Option<GtkLabel>>,
    pub has_focus: Cell<bool>,
    time_out_rc: RefCell<Option<gtk::glib::SourceId>>,
}

impl Default for GsrPictureCellBox {
    fn default() -> Self {
        Self {
            col: Cell::new(0),
            row: Cell::new(0),
            pictures_per_row: Cell::new(0),
            palette_on: Cell::new(false),
            label: RefCell::new(None),
            has_focus: Cell::new(false),
            time_out_rc: RefCell::new(None),
        }
    }
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
