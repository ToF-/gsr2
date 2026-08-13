use crate::env::default_values::MAX_PICTURES_PER_ROW;
use crate::gui::objects::gsr_picture_cell_box::GsrPictureCellBox;
use crate::model::picture::Picture;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

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
    pub fn new(pictures_per_row: i32, focus_at_coords: (i32, i32), palette_on: bool,) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().initialize(pictures_per_row, focus_at_coords, palette_on);
        obj
    }

    pub fn set_picture_at(&self, col: i32, row: i32, picture: &Picture, picture_index: usize) {
        if let Some(widget) = self.child_at(col, row) {
            let cell_box: GsrPictureCellBox = widget
                .downcast::<GsrPictureCellBox>()
                .unwrap();
            cell_box.attach_picture(picture, picture_index);

        }
    }

    pub fn set_label_from_picture_at(&self, picture: &Picture, col: i32, row: i32) {
        if let Some(widget) = self.child_at(col, row) {
            let gsr_picture_cell_box = widget.downcast::<GsrPictureCellBox>()
                .expect("can't downcast to GsrPictureCellBox");
            gsr_picture_cell_box.set_label_from_picture(picture);
        }

    }
    pub fn set_focus_at(&self, col: i32, row: i32) {
        if let Some ((current_col, current_row)) = self.imp().focus_at_coords.get() {
            if let Some(widget) = self.child_at(current_col, current_row) {
                let gsr_picture_cell_box = widget.downcast::<GsrPictureCellBox>()
                    .expect("can't downcast to GsrPictureCellBox");
                gsr_picture_cell_box.leave_focus();
            }
        }
        if let Some(widget) = self.child_at(col, row) {
            let gsr_picture_cell_box = widget.downcast::<GsrPictureCellBox>()
                .expect("can't downcast to GsrPictureCellBox");
            gsr_picture_cell_box.leave_focus();
            self.imp().focus_at_coords.set(Some((col, row)));
        }
    }

    pub fn set_palette_on(&self) {
    }

    pub fn set_palette_off(&self) {
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
