use gtk::prelude::GridExt;
use crate::gui::view::make_picture_cell_box;
use crate::gui::view::RcController;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct PictureGrid {
    pictures_per_row: i32,
    grid_rc: Rc<RefCell<gtk::Grid>>,
    controller_rc: RcController,
}

impl PictureGrid {

    pub fn new(pictures_per_row: i32, controller_rc: &RcController) -> Self {
        let grid = gtk::Grid::builder()
            .row_homogeneous(true)
            .column_homogeneous(true)
            .hexpand(true)
            .vexpand(true)
            .name("grid")
            .build();
        let picture_grid = PictureGrid {
            pictures_per_row,
            grid_rc: Rc::new(RefCell::new(grid)),
            controller_rc: controller_rc.clone(),
        };
        picture_grid.attach_cells();
        picture_grid
    }

    pub fn attach_cells(&self) {
        let grid = self.grid_rc.try_borrow().expect("can't borrow");
        for col in 0..self.pictures_per_row {
            for row in 0..self.pictures_per_row {
                let cell_box = make_picture_cell_box(col, row, &self.controller_rc);
                grid.attach(&cell_box, col, row, 1, 1)
            }
        }
    }

    pub fn remove_cells(&self) {
        let grid = self.grid_rc.try_borrow().expect("can't borrow");
        for col in 0..self.pictures_per_row {
            for row in 0..self.pictures_per_row {
                let cell_box = grid.child_at(col, row).unwrap();
                grid.remove(&cell_box);
            }
        }
    }

    pub fn set_pictures_per_row(&mut self, pictures_per_row: i32) {
        self.remove_cells();
        self.pictures_per_row = pictures_per_row;
        self.attach_cells();

    }
}

