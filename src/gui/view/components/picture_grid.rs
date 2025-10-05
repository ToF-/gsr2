use crate::gui::view::RcController;
use crate::gui::view::components::picture_frame::make_label;
use crate::gui::view::make_picture_cell_box;
use crate::gui::view::picture_label_display;
use crate::model::picture::Picture;
use gtk::prelude::BoxExt;
use gtk::prelude::Cast;
use gtk::prelude::GridExt;
use gtk::prelude::WidgetExt;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct PictureGrid {
    pictures_per_row: i32,
    grid_ref: RefCell<gtk::Grid>,
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
            grid_ref: RefCell::new(grid),
            controller_rc: controller_rc.clone(),
        };
        picture_grid.attach_cells();
        picture_grid
    }

    pub fn grid_ref(&self) -> RefCell<gtk::Grid> {
        self.grid_ref.clone()
    }

    pub fn grid(&self) -> gtk::Grid {
        self.grid_ref.borrow().clone()
    }

    pub fn set_label_for(&self, picture: &Picture, col: i32, row: i32, with_focus: bool) {
        let grid = self.grid();
        if let Some(cell_box) = grid.child_at(col as i32, row as i32) {
            let gtkPicture = cell_box
                .first_child()
                .unwrap()
                .downcast::<gtk::Picture>()
                .unwrap();
            let label = gtkPicture
                .next_sibling()
                .unwrap()
                .downcast::<gtk::Label>()
                .unwrap();
            label.set_text(&picture_label_display(&picture.label(), with_focus))
        }
    }

    pub fn attach_cells(&self) {
        let grid = self.grid_ref.try_borrow().expect("can't borrow");
        for col in 0..self.pictures_per_row {
            for row in 0..self.pictures_per_row {
                let cell_box = make_picture_cell_box(col, row, &self.controller_rc);
                grid.attach(&cell_box, col, row, 1, 1)
            }
        }
    }

    pub fn remove_cells(&self) {
        let grid = self.grid_ref.try_borrow().expect("can't borrow");
        for col in 0..self.pictures_per_row {
            for row in 0..self.pictures_per_row {
                let cell_box = grid.child_at(col, row).unwrap();
                grid.remove(&cell_box);
            }
        }
    }

    pub fn set_picture_for(&self, col: i32, row: i32, picture: &gtk::Picture) {
        let grid = self.grid();
        if let Some(widget) = grid.child_at(col as i32, row as i32) {
            let cell_box: gtk::Box = widget.downcast::<gtk::Box>().unwrap();
            while let Some(child) = cell_box.first_child() {
                cell_box.remove(&child)
            }
            cell_box.append(picture);
            cell_box.append(&make_label());
        };
    }

    pub fn set_pictures_per_row(&mut self, pictures_per_row: i32) {
        self.remove_cells();
        self.pictures_per_row = pictures_per_row;
        self.attach_cells();
    }
}
