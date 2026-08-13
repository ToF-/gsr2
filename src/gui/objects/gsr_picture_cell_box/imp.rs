use gtk::Label as GtkLabel;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;

pub struct GsrPictureCellBox {
    pub col: Cell<i32>,
    pub row: Cell<i32>,
    pub picture_index: Cell<usize>,
    pub pictures_per_row: Cell<i32>,
    pub palette_on: Cell<bool>,
    pub label: RefCell<Option<GtkLabel>>,
    pub has_focus: Cell<bool>,
    pub timeout_rc: RefCell<Option<gtk::glib::SourceId>>,
}

impl Default for GsrPictureCellBox {
    fn default() -> Self {
        Self {
            col: Cell::new(0),
            row: Cell::new(0),
            picture_index: Cell::new(0),
            pictures_per_row: Cell::new(0),
            palette_on: Cell::new(false),
            label: RefCell::new(None),
            has_focus: Cell::new(false),
            timeout_rc: RefCell::new(None),
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
