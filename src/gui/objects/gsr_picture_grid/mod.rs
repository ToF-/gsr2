use crate::env::default_values::FULL_OPACITY;
use crate::env::default_values::HALF_OPACITY;
use crate::env::default_values::MAX_PICTURES_PER_ROW;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::objects::gsr_picture_cell_box::GsrPictureCellBox;
use crate::model::picture::Picture;
use gtk::glib;
use gtk::prelude::*;

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
    pub fn new() -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_row_homogeneous(true);
        obj.set_column_homogeneous(true);
        obj.set_hexpand(true);
        obj.set_vexpand(true);
        obj
    }

    pub fn gsr_application(&self) -> GsrApplication {
        self.root()
            .and_then(|root| root.downcast::<GsrApplicationWindow>().ok())
            .expect("GsrPictureGrid is not inside a Window")
            .gsr_application()
    }
    pub fn initialize_pictures(&self) {
        let shared_view = self.gsr_application().shared_view();
        let view = shared_view.borrow();
        let shared_navigator = self.gsr_application().shared_navigator();
        let navigator = shared_navigator.borrow();
        let shared_gallery = self.gsr_application().shared_gallery();
        let gallery = shared_gallery.borrow();
        dbg!(&navigator);
        let pictures_per_row = view.pictures_per_row();
        for col in 0..pictures_per_row {
            for row in 0..pictures_per_row {
                if self.child_at(col, row).is_none() {
                    let gsr_picture_cell_box = GsrPictureCellBox::new(
                        col,
                        row,
                        0,
                        view.pictures_per_row(),
                        view.palette_on(),
                    );
                    self.attach(&gsr_picture_cell_box, col, row, 1, 1);
                }
                if let Some(index) = navigator.position_from_coords(row as usize, col as usize) {
                    dbg!(index);
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
    }

    pub fn change_size(&self, _pictures_per_row: i32, _palette_on: bool) {
        todo!();
    }

    pub fn set_picture_at(&self, col: i32, row: i32, picture: &Picture, picture_index: usize) {
        let binding = self.gsr_application().shared_view();
        let view = binding.borrow();
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

    pub fn set_focus_at(&self, col: i32, row: i32) {
        dbg!(self.gsr_application().shared_view());
        let (current_col, current_row) = {
            let binding = self.gsr_application().shared_view();
            let view = binding.borrow();
            view.focus_at_coords()
        };
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
                let binding = self.gsr_application().shared_view();
                let mut view = binding.borrow_mut();
                view.set_focus_at_coords((col, row));
            }
        }
    }

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
