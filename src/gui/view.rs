use crate::Controller;
use crate::display::picture_label_display;
use crate::gen_image::no_thumbnail_picture;
use crate::gui::components::*;
use crate::gui::controller::RcController;
use crate::gui::event::Event::*;
use crate::paths::check_path_exists;
use crate::picture::Picture;
use gtk::ApplicationWindow;
use gtk::glib::clone;
use gtk::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

pub const LEFT_PANE: usize = 0;
pub const RIGHT_PANE: usize = 1;

#[derive(Clone, Debug)]
pub struct View {
    width: i32,
    height: i32,
    cells_per_row: i32,
    application_window_rc: Option<Rc<RefCell<gtk::ApplicationWindow>>>,
}

impl View {
    pub fn new(width: i32, height: i32, cells_per_row: i32) -> Self {
        View {
            width,
            height,
            cells_per_row,
            application_window_rc: None,
        }
    }

    pub fn application_window_rc(&self) -> Rc<RefCell<gtk::ApplicationWindow>> {
        match &self.application_window_rc {
            Some(application_window_rc) => application_window_rc.clone(),
            None => panic!("uninitialized Rc<RefCell<gtk::ApplicationWindow>>"),
        }
    }

    pub fn set_application_window_rc(
        &mut self,
        application_window_rc: Rc<RefCell<gtk::ApplicationWindow>>,
    ) {
        self.application_window_rc = Some(application_window_rc)
    }

    pub fn set_picture_for_single_view(
        application_window: &ApplicationWindow,
        controller: &Controller,
    ) {
        let picture: Picture = controller.current_picture();
        let picture_file_path = picture.file_path();
        let gtkPicture = if let Ok(file_path) = check_path_exists(&PathBuf::from(picture_file_path))
        {
            picture_from_file_path(file_path)
        } else {
            no_thumbnail_picture()
        };
        set_single_view_picture(application_window, &gtkPicture);
    }

    pub fn set_label_for_current_picture(
        application_window: &ApplicationWindow,
        controller: &Controller,
        with_focus: bool,
    ) {
        let navigator = controller.navigator();
        let position = navigator.position();
        let picture = controller.current_picture();
        if !controller.state().single_view() {
            if let Some((row, col)) = navigator.coords_from_position(position) {
                let grid = multiple_view_grid(application_window);
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
        }
    }

    fn set_pictures_for_multiple_view(
        application_window: &ApplicationWindow,
        controller: &Controller,
        pictures_per_row: usize,
    ) {
        let cells_per_row: i32 = pictures_per_row as i32;
        let navigator = controller.navigator();
        let gallery = controller.gallery();
        let grid = multiple_view_grid(application_window);
        for col in 0..cells_per_row {
            for row in 0..cells_per_row {
                let coords = (row as usize, col as usize);
                let cell: gtk::Box = grid
                    .child_at(col, row)
                    .unwrap()
                    .downcast::<gtk::Box>()
                    .unwrap();
                remove_children_from_box(&cell);
                if let Some(index) = navigator.position_from_coords(coords.0, coords.1) {
                    let picture = gallery.picture(index);
                    let is_thumbnail = cells_per_row == 10;
                    let is_focus = index == navigator.position();
                    let picture_file_path = picture.view_file_path(is_thumbnail);
                    let gtkPicture = if let Ok(file_path) =
                        check_path_exists(&PathBuf::from(picture_file_path))
                    {
                        picture_from_file_path(file_path)
                    } else {
                        no_thumbnail_picture()
                    };
                    cell.append(&gtkPicture);
                    let label = make_label_for_picture(&picture, index == navigator.position());
                    cell.append(&label);
                }
            }
        }
    }

    pub fn set_pictures(application_window: &gtk::ApplicationWindow, controller: &Controller) {
        if controller.state().single_view() {
            Self::set_picture_for_single_view(application_window, &controller)
        } else {
            let pictures_per_row = controller.state().pictures_per_row();
            Self::set_pictures_for_multiple_view(application_window, &controller, pictures_per_row)
        }
    }
}
