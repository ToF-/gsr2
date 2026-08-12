use std::cell::Cell;
use crate::env::default_values::{
    MAX_PICTURES_PER_ROW,
};
use crate::gui::controller::RcController;
use crate::gui::main_controller::MainController;
use crate::gui::mode::Mode;
use crate::gui::objects::gsr_picture_cell_box::GsrPictureCellBox;
use crate::model::picture::Picture;
use gtk::glib::timeout_add_local;
use gtk::glib::{ControlFlow, clone};
use gtk::prelude::Cast;
use gtk::prelude::GridExt;
use gtk::prelude::WidgetExt;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct GridView {
    grid: gtk::Grid,
    controller_rc: RcController,
    focus_coords: Cell<(i32, i32)>,
}

impl GridView {
    pub fn new_from_grid(grid: &gtk::Grid, controller_rc: &RcController) -> Self {
        GridView {
            controller_rc: controller_rc.clone(),
            grid: grid.clone(),
            focus_coords: Cell::new((0,0)),
        }
    }
    pub fn new(
        pictures_per_row: i32,
        palette_on: bool,
        controller_rc: &RcController,
        main_controller: &MainController,
    ) -> Self {
        let grid = make_grid();
        let grid_view = GridView {
            grid,
            focus_coords: Cell::new((0,0)),
            controller_rc: controller_rc.clone(),
        };
        grid_view.attach_cells(pictures_per_row, palette_on, main_controller);
        // grid_view.attach_focus_symbol_change_event();
        grid_view
    }


    pub fn grid(&self) -> gtk::Grid {
        self.grid.clone()
    }

    pub fn set_focus_at(&self, col: i32, row: i32) {
        dbg!(&self.focus_coords);
        let (current_col,current_row) = self.focus_coords.get();
        println!("set_focus_at ({},{}) with current focus : ({},{})", col, row, current_col, current_row);
        let grid = self.grid();
        if let Some(cell) = grid.child_at(current_col, current_row) {
            let cell_box = cell
                .downcast::<GsrPictureCellBox>()
                .expect("not a GsrPictureCellBox");
                cell_box.leave_focus()
            }
        if let Some(cell) = grid.child_at(col, row) {
            let cell_box = cell
                .downcast::<GsrPictureCellBox>()
                .expect("not a GsrPictureCellBox");
                cell_box.enter_focus();
        }
        self.focus_coords.set((col, row));
        dbg!(&self.focus_coords);
    }
    pub fn set_label_text_at(
        &self,
        picture: &Picture,
        col: i32,
        row: i32,
        with_focus: Option<char>,
    ) {
        let grid = self.grid();
        if let Some(cell) = grid.child_at(col, row) {
            let cell_box = cell
                .downcast::<GsrPictureCellBox>()
                .expect("not a GsrPictureCellBox");
            match with_focus {
                None => cell_box.leave_focus(),
                Some(_) => cell_box.enter_focus(),
            }
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        let mut count: usize = 0;
        for col in 0..MAX_PICTURES_PER_ROW {
            for row in 0..MAX_PICTURES_PER_ROW {
                if self.grid.child_at(col, row).is_some() {
                    count += 1
                }
            }
        }
        count
    }

    pub fn attach_cells(&self, pictures_per_row: i32, palette_on: bool, main_controller: &MainController) {
        let grid = &self.grid;
        for col in 0..pictures_per_row {
            for row in 0..pictures_per_row {
                let cell_box = GsrPictureCellBox::new(col, row, pictures_per_row, palette_on);
                cell_box.connect_main_controller(main_controller);
                grid.attach(&cell_box, col, row, 1, 1)
            }
        }
    }

    pub fn remove_cells(&self) {
        let grid = &self.grid;
        for col in 0..MAX_PICTURES_PER_ROW {
            for row in 0..MAX_PICTURES_PER_ROW {
                if let Some(cell_box) = grid.child_at(col, row) {
                    grid.remove(&cell_box)
                }
            }
        }
    }

    pub fn set_picture_at(
        &self,
        col: i32,
        row: i32,
        picture: &Picture,
        has_focus: bool,
    ) {
        let grid = self.grid();
        if let Some(widget) = grid.child_at(col, row) {
            let cell_box: GsrPictureCellBox = widget.downcast::<GsrPictureCellBox>().unwrap();
            cell_box.attach_picture(picture, has_focus);
        }
    }

    fn cell_box_at(&self, col: i32, row: i32) -> Option<GsrPictureCellBox> {
        if let Some(widget) = self.grid.child_at(col, row) {
            widget.downcast::<GsrPictureCellBox>().ok()
        } else {
            None
        }
    }

    fn picture_at(&self, col: i32, row: i32) -> Option<gtk::Picture> {
        if let Some(cell_box) = self.cell_box_at(col, row) {
            cell_box
                .first_child()
                .unwrap()
                .downcast::<gtk::Picture>()
                .ok()
        } else {
            None
        }
    }

    pub fn set_picture_opacity_at(&self, col: i32, row: i32, opacity: f64) {
        if let Some(picture) = self.picture_at(col, row) {
            picture.set_opacity(opacity)
        }
    }

    pub fn change_dimension(&self, pictures_per_row: i32, palette_on: bool, main_controller: &MainController) {
        self.remove_cells();
        self.attach_cells(pictures_per_row, palette_on, main_controller);
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
