use crate::gui::objects::gsr_picture_grid::GsrPictureGrid;
use crate::env::default_values::{
    MAX_PICTURES_PER_ROW,
};
use crate::gui::controller::RcController;
use crate::gui::main_controller::MainController;
use crate::gui::objects::gsr_picture_cell_box::GsrPictureCellBox;
use crate::model::picture::Picture;
use gtk::prelude::Cast;
use gtk::prelude::GridExt;
use gtk::prelude::WidgetExt;

#[derive(Clone, Debug)]
pub struct GridView {
    pub gsr_picture_grid: GsrPictureGrid,
    controller_rc: RcController,
}

impl GridView {
    pub fn new_from_grid(gsr_picture_grid: &GsrPictureGrid, controller_rc: &RcController) -> Self {
        GridView {
            controller_rc: controller_rc.clone(),
            gsr_picture_grid: gsr_picture_grid.clone(),
        }
    }
    pub fn new(
        pictures_per_row: i32,
        palette_on: bool,
        controller_rc: &RcController,
        main_controller: &MainController,
    ) -> Self {
        let grid_view = GridView {
            gsr_picture_grid: GsrPictureGrid::new(10, (0, 0), false),
            controller_rc: controller_rc.clone(),
        };
        grid_view.attach_cells(0, pictures_per_row, palette_on, main_controller);
        // grid_view.attach_focus_symbol_change_event();
        grid_view
    }


    pub fn set_focus_at(&self, col: i32, row: i32) {
        self.gsr_picture_grid.set_focus_at(col, row);
    }

    pub fn set_label_text_at(
        &self,
        picture: &Picture,
        col: i32,
        row: i32,
    ) {
        self.gsr_picture_grid.set_label_from_picture_at(picture, col, row);
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.gsr_picture_grid.size()
    }

    pub fn attach_cells(&self, picture_index_start: usize, pictures_per_row: i32, palette_on: bool, main_controller: &MainController) {
        for col in 0..pictures_per_row {
            for row in 0..pictures_per_row {
                let picture_index = ((picture_index_start as i32) + (row * pictures_per_row) + col) as usize;
                let cell_box = GsrPictureCellBox::new(col, row, picture_index, pictures_per_row, palette_on);
                cell_box.connect_main_controller(main_controller);
                self.gsr_picture_grid.attach(&cell_box, col, row, 1, 1)
            }
        }
    }

    pub fn remove_cells(&self) {
        for col in 0..MAX_PICTURES_PER_ROW {
            for row in 0..MAX_PICTURES_PER_ROW {
                if let Some(cell_box) = self.gsr_picture_grid.child_at(col, row) {
                    self.gsr_picture_grid.remove(&cell_box)
                }
            }
        }
    }

    pub fn set_picture_at(
        &self,
        col: i32,
        row: i32,
        picture: &Picture,
        picture_index: usize,
    ) {
        if let Some(widget) = self.gsr_picture_grid.child_at(col, row) {
            let cell_box: GsrPictureCellBox = widget.downcast::<GsrPictureCellBox>().unwrap();
            cell_box.attach_picture(picture, picture_index);
        }
    }

    fn cell_box_at(&self, col: i32, row: i32) -> Option<GsrPictureCellBox> {
        if let Some(widget) = self.gsr_picture_grid.child_at(col, row) {
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
        self.attach_cells(0, pictures_per_row, palette_on, main_controller);
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
