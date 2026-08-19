use crate::env::default_values::FULL_OPACITY;
use crate::env::default_values::HALF_OPACITY;
use crate::env::default_values::MAX_PICTURES_PER_ROW;
use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_picture_cell_box::GsrPictureCellBox;
use crate::gui::view::View;
use crate::model::gallery::Gallery;
use crate::model::picture::Picture;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

mod imp;

glib::wrapper! {
    pub struct GsrPictureGrid(ObjectSubclass<imp::GsrPictureGrid>)
        @extends gtk::Widget, gtk::Grid,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::Orientable,
            gtk::ConstraintTarget;
}

impl GsrPictureGrid {
    pub fn new(
        view: RefCell<View>,
        navigator: RefCell<Navigator>,
        gallery: RefCell<Gallery>,
    ) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_row_homogeneous(true);
        obj.set_column_homogeneous(true);
        obj.set_hexpand(true);
        obj.set_vexpand(true);
        obj.imp().initialize(view, navigator, gallery);
        obj
    }

    pub fn initialize_pictures(&self, navigator: &Navigator, gallery: &Gallery, palette_on: bool) {
        self.fill_with_cell_boxes();
        {
            let mut view =  self.imp().view.borrow_mut();
            view.set_palette_on(palette_on);
        }
        let pictures_per_row = self.imp().view.borrow().pictures_per_row();
        for col in 0..pictures_per_row {
            for row in 0..pictures_per_row {
                if let Some(index) = navigator.position_from_coords(row as usize, col as usize) {
                    let picture = gallery.picture(index);
                    self.set_picture_at(col, row, &picture, index);
                    let opacity: f64 = if navigator.is_selected(index) {
                        HALF_OPACITY
                    } else {
                        FULL_OPACITY
                    };
                    self.set_picture_opacity_at(col, row, opacity);
                }
            }
        }
        let (col, row) = (self.imp().view.borrow()).focus_at_coords().clone();
        self.set_focus_at(col, row);
    }

    pub fn change_size(&self, pictures_per_row: i32, palette_on: bool) {
        todo!();
    }

    fn remove_all_cell_boxes(&self) {
        for col in 0..MAX_PICTURES_PER_ROW {
            for row in 0..MAX_PICTURES_PER_ROW {
                if let Some(widget) = self.child_at(col, row) {
                    self.remove(&widget)
                }
            }
        }
    }
    fn fill_with_cell_boxes(&self) {
        let view = self.imp().view.borrow();
        let pictures_per_row = view.pictures_per_row();
        let palette_on = view.palette_on();
        self.remove_all_cell_boxes();
        for col in 0..pictures_per_row {
            for row in 0..pictures_per_row {
                let picture_index: usize = (row * pictures_per_row + col) as usize;
                if let Some(widget) = self.child_at(col, row) {
                    self.remove(&widget);
                };
                let gsr_picture_cell_box =
                    GsrPictureCellBox::new(col, row, picture_index, pictures_per_row, palette_on);
                self.attach(&gsr_picture_cell_box, col, row, 1, 1);
            }
        }
    }

    pub fn set_picture_at(&self, col: i32, row: i32, picture: &Picture, picture_index: usize) {
        let view = self.imp().view.borrow();
        let pictures_per_row = view.pictures_per_row();
        let palette_on = view.palette_on();
        if let Some(widget) = self.child_at(col, row) {
            self.remove(&widget);
        };
        let gsr_picture_cell_box =
            GsrPictureCellBox::new(col, row, picture_index, pictures_per_row, palette_on);
        gsr_picture_cell_box.attach_picture(picture, picture_index);
        self.attach(&gsr_picture_cell_box, col, row, 1, 1);
    }

    pub fn set_label_from_picture_at(&self, picture: &Picture, col: i32, row: i32) {
        if let Some(widget) = self.child_at(col, row) {
            let gsr_picture_cell_box = widget
                .downcast::<GsrPictureCellBox>()
                .expect("can't downcast to GsrPictureCellBox");
            gsr_picture_cell_box.set_label_from_picture(picture);
        }
    }

    pub fn set_label_text_at(&self, _col: i32, _row: i32, _text: &str) {}

    pub fn set_focus_at(&self, col: i32, row: i32) {
        let view = self.imp().view.borrow();
        let (current_col, current_row) = view.focus_at_coords();
        if let Some(widget) = self.child_at(current_col, current_row) {
            let gsr_picture_cell_box = widget
                .downcast::<GsrPictureCellBox>()
                .expect("can't downcast to GsrPictureCellBox");
            gsr_picture_cell_box.leave_focus();
        }
        if let Some(widget) = self.child_at(col, row) {
            let gsr_picture_cell_box = widget
                .downcast::<GsrPictureCellBox>()
                .expect("can't downcast to GsrPictureCellBox");
            gsr_picture_cell_box.enter_focus();
            {
            let mut view = self.imp().view.borrow_mut();
                view.set_focus_at_coords((col, row));
            }
        }
    }

    pub fn set_palette_on(&self) {}

    pub fn set_palette_off(&self) {}

    pub fn set_picture_opacity_at(&self, col: i32, row: i32, opacity: f64) {
        if let Some(widget) = self.child_at(col, row) {
            let cell_box: GsrPictureCellBox = widget.downcast::<GsrPictureCellBox>().unwrap();
            cell_box.set_opacity(opacity);
        }
    }

    pub fn size(&self) -> usize {
        let mut count: usize = 0;
        for col in 0..MAX_PICTURES_PER_ROW {
            for row in 0..MAX_PICTURES_PER_ROW {
                if self.child_at(col, row).is_some() {
                    count += 1
                }
            }
        }
        count
    }
}
