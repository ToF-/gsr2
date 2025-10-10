use gtk::glib::timeout_add_local;
use gtk::glib::{ControlFlow, Propagation};
use std::time::Duration;
use crate::clone;
use crate::env::default_values::MAX_PICTURES_PER_ROW;
use crate::gui::controller::RcController;
use crate::gui::display::picture_label_display;
use crate::gui::view::picture_cell_box::make_picture_cell_box;
use crate::gui::view::picture_frame::make_label;
use crate::model::picture::Picture;
use gtk::prelude::BoxExt;
use gtk::prelude::Cast;
use gtk::prelude::GridExt;
use gtk::prelude::WidgetExt;
use crate::env::default_values::{FOCUS_SYMBOL_1, FOCUS_SYMBOL_2};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct PictureGrid {
    grid: gtk::Grid,
    focus_symbol_change_on: usize,
    controller_rc: RcController,
}

impl PictureGrid {
    pub fn new_from_grid(grid: &gtk::Grid, controller_rc: &RcController) -> Self {
        PictureGrid {
            controller_rc: controller_rc.clone(),
            focus_symbol_change_on: 0,
            grid: grid.clone(),
        }
    }
    pub fn new(pictures_per_row: i32, controller_rc: &RcController) -> Self {
        let grid = make_grid();
        let picture_grid = PictureGrid {
            grid,
            focus_symbol_change_on: 0,
            controller_rc: controller_rc.clone(),
        };
        picture_grid.attach_cells(pictures_per_row);
        
        picture_grid
    }

    pub fn set_focus_symbol_change_off(&mut self) {
        println!("set_focus_symbol_change_off");
        self.focus_symbol_change_on = 0;
    }

    pub fn set_focus_symbol_change_on(&mut self) {
        println!("set_focus_symbol_change_on");
        self.focus_symbol_change_on = self.focus_symbol_change_on + 1;
    }

    pub fn attach_focus_symbol_change_event(&mut self) {
        let controller_rc = self.controller_rc.clone();
        let picture_grid_rc = Rc::new(RefCell::new(self.clone()));
        if self.focus_symbol_change_on == 0 {
            let delay: u64 = 1;
            timeout_add_local(
                Duration::new(delay, 0),
                clone!(
                    #[strong] picture_grid_rc,
                    #[strong] controller_rc,
                    move || {
                        if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                            if ! controller.state().single_view() {
                                controller.toggle_focus_symbol();
                                if let Ok(mut picture_grid) = picture_grid_rc.try_borrow_mut() {
                                    println!("{:?}", picture_grid.focus_symbol_change_on);
                                }
                                ControlFlow::Continue
                            } else {
                                if let Ok(mut picture_grid) = picture_grid_rc.try_borrow_mut() {
                                    picture_grid.set_focus_symbol_change_off();
                                }
                                ControlFlow::Break

                            }
                        } else {
                            if let Ok(mut picture_grid) = picture_grid_rc.try_borrow_mut() {
                                picture_grid.set_focus_symbol_change_off();
                            }
                            ControlFlow::Continue
                        }
                    }
                ));
                self.focus_symbol_change_on = 1;
        }

    }
    pub fn grid(&self) -> gtk::Grid {
        self.grid.clone()
    }

    pub fn set_label_for(&mut self, picture: &Picture, col: i32, row: i32, with_focus: Option<char>) {
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
        for col in 0..MAX_PICTURES_PER_ROW {
            for row in 0..MAX_PICTURES_PER_ROW {
                if self.grid.child_at(col, row).is_some() {
                    count += 1
                }
            }
        }
        count
    }

    pub fn attach_cells(&self, pictures_per_row: i32) {
        let grid = &self.grid;
        for col in 0..pictures_per_row {
            for row in 0..pictures_per_row {
                let cell_box = make_picture_cell_box(col, row, &self.controller_rc);
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

    pub fn change_dimension(&mut self, pictures_per_row: i32) {
        self.remove_cells();
        self.attach_cells(pictures_per_row);
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
