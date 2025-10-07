use crate::gui::controller::RcController;
use crate::gui::display::picture_label_display;
use crate::gui::view::picture_cell_box::make_picture_cell_box;
use crate::gui::view::picture_frame::make_label;
use crate::model::picture::Picture;
use gtk::prelude::BoxExt;
use gtk::prelude::Cast;
use gtk::prelude::GridExt;
use gtk::prelude::WidgetExt;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct PictureGrid {
    pictures_per_row: Cell<i32>,
    grid: gtk::Grid,
    controller_rc: RcController,
}

impl PictureGrid {
    pub fn new_from_grid(
        grid: &gtk::Grid,
        pictures_per_row: i32,
        controller_rc: &RcController,
    ) -> Self {
        PictureGrid {
            pictures_per_row: pictures_per_row.into(),
            controller_rc: controller_rc.clone(),
            grid: grid.clone(),
        }
    }
    pub fn new(pictures_per_row: i32, controller_rc: &RcController) -> Self {
        let grid = make_grid();
        let picture_grid = PictureGrid {
            pictures_per_row: pictures_per_row.into(),
            grid,
            controller_rc: controller_rc.clone(),
        };
        picture_grid.attach_cells();
        picture_grid
    }

    pub fn grid(&self) -> gtk::Grid {
        self.grid.clone()
    }
 
    pub fn pictures_per_row(&self) -> i32 {
        self.pictures_per_row.get()
    }

    
    pub fn set_label_for(&self, picture: &Picture, col: i32, row: i32, with_focus: bool) {
        assert!(col < self.pictures_per_row.get());
        assert!(row < self.pictures_per_row.get());
        let grid = self.grid();
        if let Some(cell_box) = grid.child_at(col, row) {
            let gtk_picture = cell_box
                .first_child()
                .unwrap()
                .downcast::<gtk::Picture>()
                .unwrap();
            let label = gtk_picture
                .next_sibling()
                .unwrap()
                .downcast::<gtk::Label>()
                .unwrap();
            label.set_text(&picture_label_display(&picture.label(), with_focus))
        }
    }

    #[allow(dead_code)] 
    pub fn size(&self) -> usize {
        let mut count: usize = 0;
        for col in 0..10 {
            for row in 0..10 {
                if self.grid.child_at(col, row).is_some() {
                    count += 1
                }
            }
        };
        count
    }

    pub fn attach_cells(&self) {
        let grid = &self.grid;
        for col in 0..self.pictures_per_row.get() {
            for row in 0..self.pictures_per_row.get() {
                let cell_box = make_picture_cell_box(col, row, &self.controller_rc);
                println!("new cell in ({},{})", col, row);
                grid.attach(&cell_box, col, row, 1, 1)
            }
        }
    }

    pub fn remove_cells(&self) {
        let grid = &self.grid;
        for col in 0..self.pictures_per_row.get() {
            for row in 0..self.pictures_per_row.get() {
                if let Some(cell_box) = grid.child_at(col, row) {
                    println!("remove cell in ({},{})", col, row);
                    grid.remove(&cell_box)
                }
            }
        }
    }

    pub fn set_picture_for(&self, col: i32, row: i32, picture: &gtk::Picture) {
        let grid = self.grid();
        if let Some(widget) = grid.child_at(col, row) {
            let cell_box: gtk::Box = widget.downcast::<gtk::Box>().unwrap();
            while let Some(child) = cell_box.first_child() {
                cell_box.remove(&child)
            }
            cell_box.append(picture);
            cell_box.append(&make_label());
        };
    }

    pub fn set_pictures_per_row(&mut self, new_row_size: i32) {
        println!("changing picture grid pictures per row from {:?} to {}", self.pictures_per_row, new_row_size);
        self.remove_cells();
        self.pictures_per_row.set(new_row_size);
        self.attach_cells();
        println!("pictures per row changed to {:?}", self.pictures_per_row);
        println!("{:?}", self);
    }
}
pub fn make_grid() -> gtk::Grid {
    gtk::Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .hexpand(true)
        .vexpand(true)
        .name("grid")
        .build()
}
