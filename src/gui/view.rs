use crate::gui::display::title_display;
use crate::Controller;
use crate::file::paths::check_path_exists;
use crate::gui::controller::RcController;
use crate::gui::direction::Direction;
use crate::gui::display::picture_label_display;
use crate::gui::view::components::main_window::MainWindow;
use crate::gui::view::components::picture_cell_box::make_picture_cell_box;
use crate::gui::view::components::picture_frame::PictureFrame;
use crate::gui::view::components::picture_grid::PictureGrid;
use crate::model::gen_image::no_thumbnail_picture;
use crate::model::picture::Picture;
use gtk::Window;
use gtk::gio::File;
use gtk::glib::object::Cast;
use gtk::glib::timeout_add_local;
use gtk::prelude::*;
use gtk::{self};
use gtk::{gdk, Label, Orientation, Picture as GtkPicture};
use std::cell::RefCell;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

pub mod components;

#[derive(Clone, Debug)]
pub struct View {
    main_window_rc: Rc<RefCell<MainWindow>>,
}

impl View {
    pub fn new(main_window: &MainWindow) -> Self {
        View {
            main_window_rc: Rc::new(RefCell::new(main_window.clone())),
        }
    }

    pub fn main_window(&self) -> MainWindow {
        self.main_window_rc.borrow().clone()
    }

    pub fn application_window(&self) -> gtk::ApplicationWindow {
        self.main_window().application_window()
    }

    pub fn picture_grid(&self) -> PictureGrid {
        self.main_window().picture_grid()
    }

    pub fn set_title(&self, controller: &Controller) {
        let title = title_display(controller);
        self.main_window().application_window().set_title(Some(&title))
    }

    pub fn set_picture_for_single_view(&self, controller: &Controller) {
        let picture: Picture = controller.current_picture();
        let picture_file_path = picture.file_path();
        let gtkPicture = if let Ok(file_path) = check_path_exists(&PathBuf::from(picture_file_path))
        {
            Self::picture_from_file_path(file_path)
        } else {
            no_thumbnail_picture()
        };
        let picture_frame = self.main_window().picture_frame();
        picture_frame.set_picture(controller, &gtkPicture);
    }

    pub fn set_label_for_current_picture(&self, controller: &Controller, with_focus: bool) {
        let navigator = controller.navigator();
        let position = navigator.position();
        let picture = controller.current_picture();
        if !controller.state().single_view() {
            if let Some((row, col)) = navigator.coords_from_position(position) {
                let picture_grid = self.picture_grid();
                picture_grid.set_label_for(&picture, col as i32, row as i32, with_focus);
            }
        }
    }

    fn set_pictures_for_multiple_view(&self, controller: &Controller, pictures_per_row: usize) {
        let cells_per_row: i32 = pictures_per_row as i32;
        let navigator = controller.navigator();
        let gallery = controller.gallery();
        let picture_grid = self.picture_grid();
        let grid = picture_grid.grid();
        for col in 0..cells_per_row {
            for row in 0..cells_per_row {
                let coords = (row as usize, col as usize);
                let cell: gtk::Box = grid
                    .child_at(col, row)
                    .unwrap()
                    .downcast::<gtk::Box>()
                    .unwrap();
                Self::remove_children_from_box(&cell);
                if let Some(index) = navigator.position_from_coords(coords.0, coords.1) {
                    let picture = gallery.picture(index);
                    let is_thumbnail = cells_per_row == 10;
                    let is_focus = index == navigator.position();
                    let picture_file_path = picture.view_file_path(is_thumbnail);
                    let gtkPicture = if let Ok(file_path) =
                        check_path_exists(&PathBuf::from(picture_file_path))
                    {
                        Self::picture_from_file_path(file_path)
                    } else {
                        no_thumbnail_picture()
                    };
                    let picture_grid = self.picture_grid();
                    picture_grid.set_picture_for(col, row, &gtkPicture);
                    picture_grid.set_label_for(&picture, col, row, is_focus);
                }
            }
        }
    }

    pub fn set_pictures(&self, controller: &Controller) {
        if controller.state().single_view() {
            self.set_picture_for_single_view(&controller)
        } else {
            let pictures_per_row = controller.state().pictures_per_row();
            self.set_pictures_for_multiple_view(&controller, pictures_per_row)
        }
    }

    pub fn change_dimension(&mut self, pictures_per_row: usize) {
        if let Ok(mut picture_grid) = self.main_window().picture_grid_ref().try_borrow_mut() {
            picture_grid.set_pictures_per_row(pictures_per_row as i32);
        }
    }


    pub fn full_size_arrow_move(&self, direction: Direction) {
        let step: f64 = 100.0;
        let window = self.main_window().frame_window();
        let h = window.hadjustment();
        let v = window.vadjustment();
        match direction {
            Direction::Right => h.set_value(h.value() + step),
            Direction::Left => h.set_value(h.value() - step),
            Direction::Down => v.set_value(v.value() + step),
            Direction::Up => v.set_value(v.value() - step),
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn make_entry_window(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
    ) -> gtk::Window {
        let window: gtk::Window = Window::builder()
            .title(prompt)
            .default_width(300)
            .default_height(30)
            .deletable(false)
            .decorated(true)
            .modal(true)
            .build();
        let entry_label: gtk::Label = Label::new(None);
        window.set_resizable(false);
        window.set_hide_on_close(false);
        window.set_child(Some(&entry_label));
        window.set_modal(true);
        window.set_transient_for(Some(application_window));
        window.present();
        window
    }

    pub fn picture_from_file_path(file_path: &Path) -> gtk::Picture {
        GtkPicture::builder()
            .file(&File::for_path(file_path))
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    pub fn remove_children_from_box(cell_box: &gtk::Box) {
        while let Some(child) = cell_box.first_child() {
            cell_box.remove(&child)
        }
    }


    pub fn single_view(&self) -> bool {
        let stack = self.main_window().stack();
        let child_name = stack.visible_child_name().unwrap();
        child_name == "single_view"
    }

    pub fn toggle_view_stack(&self, controller: &Controller) {
        let stack = self.main_window().stack();
        if controller.state().single_view() {
            stack.set_visible_child_name("single_view")
        } else {
            stack.set_visible_child_name("multiple_view")
        }
    }

}
