use std::rc::Rc;
use std::cell::RefCell;

pub struct PictureGrid {
    pictures_per_row: i32,
    grid_rc: Rc<RefCell<gtk::Grid>>,
}

