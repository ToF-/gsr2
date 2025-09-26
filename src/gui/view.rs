use crate::Controller;
use crate::application_state::ApplicationState;
use crate::command_line_interface::CommandLineInterface;
use crate::direction::Direction;
use crate::display::title_display;
use crate::editor::{Editor, InputKind};
use crate::gui::components::*;
use crate::gui::controller::RcController;
use crate::order;
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::gdk::Key;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{
    Align, ApplicationWindow, CssProvider, Grid, Label, Orientation, Picture, ScrolledWindow,
    Widget, gdk,
};
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct View {
    pub width: i32,
    pub height: i32,
    pub cells_per_row: i32,
    pub application_window_rc: Option<Rc<RefCell<gtk::ApplicationWindow>>>,
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

    pub fn application_window_rc(&self) -> &Rc<RefCell<gtk::ApplicationWindow>> {
        match &self.application_window_rc {
            Some(application_window_rc) => application_window_rc,
            None => panic!("uninitialized Rc<RefCell<gtk::ApplicationWindow>>"),
        }
    }

    pub fn set_application_window_rc(
        &mut self,
        application_window_rc: Rc<RefCell<gtk::ApplicationWindow>>,
    ) {
        self.application_window_rc = Some(application_window_rc)
    }

    pub fn setup_components(&self, application_window: &gtk::ApplicationWindow) {
        let grid = make_grid(self.cells_per_row);

        let panel = make_panel(&grid);

        let multiple_view_scrolled_window = make_multiple_view_scrolled_window();
        multiple_view_scrolled_window.set_child(Some(&panel));

        let frame = make_frame();
        let picture = make_picture();
        frame.append(&picture);
        let single_view_scrolled_window = make_single_view_scrolled_window();
        single_view_scrolled_window.set_child(Some(&frame));

        let view_stack = make_stack();
        let _ = view_stack.add_child(&single_view_scrolled_window);
        let _ = view_stack.add_child(&multiple_view_scrolled_window);
        if self.cells_per_row == 1 {
            view_stack.set_visible_child(&single_view_scrolled_window);
        } else {
            view_stack.set_visible_child(&multiple_view_scrolled_window);
        }
        application_window.set_child(Some(&view_stack));
    }

    pub fn build_gui(application: &gtk::Application, controller_rc: &RcController) {
        let window = make_application_window(application);

        // keep a reference to application window for manipulations through Controller::View via events
        if let Ok(mut controller) = controller_rc.try_borrow_mut() {
            let mut view = controller.view();
            view.setup_components(&window);
            view.attach_events(&window, controller_rc);
            let application_window_rc = Rc::new(RefCell::new(window));
            view.set_application_window_rc(application_window_rc);
            controller.set_view(view.clone());
        } else {
            panic!("cannot borrow_mut");
        };
        if let Ok(controller) = controller_rc.try_borrow() {
            let view = controller.view();
            if let Ok(application_window) = view.application_window_rc().try_borrow() {
                view.set_pictures(&application_window, &controller);
                application_window.present()
            } else {
                panic!("cannot borrow");
            }
        } else {
            panic!("cannot borrow");
        }
    }

    pub fn attach_events(&self, window: &gtk::ApplicationWindow, controller_rc: &RcController) {
        let stack = window.first_child().unwrap();
        let single_view_scrolled_window = stack.first_child().unwrap();
        let multiple_view_scrolled_window = stack.last_child().unwrap();
        let panel = multiple_view_scrolled_window.first_child().unwrap();
        let left_pane = panel.first_child().unwrap();
        let right_pane = panel.last_child().unwrap();

        let gesture_left_click = gtk::GestureClick::new();
        gesture_left_click.set_button(1);
        gesture_left_click.connect_pressed(clone!(@strong controller_rc => move |_,_,_,_| {
            println!("left_click {:?}", controller_rc);
        }));
        left_pane.add_controller(gesture_left_click);

        let gesture_right_click = gtk::GestureClick::new();
        gesture_right_click.set_button(1);
        gesture_right_click.connect_pressed(clone!(@strong controller_rc => move |_,_,_,_| {
            println!("right_click {:?}", controller_rc);
        }));
        right_pane.add_controller(gesture_right_click);

        let evk = gtk::EventControllerKey::new();
        evk.connect_key_pressed(clone!(@strong controller_rc => move |_, key, _, _| {
            println!("key {:?}", key);
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                controller.process_key(key);
            }
            gtk::Inhibit(false)
        }));
        window.add_controller(evk);
    }

    fn set_picture_for_single_view(
        &self,
        application_window: &ApplicationWindow,
        controller: &Controller,
    ) {
        let navigator = controller.navigator();
        let gallery = controller.gallery();
        let gtkPicture: gtk::Picture = single_view_picture(application_window);
        let file_path = gallery.picture(navigator.position()).file_path();
        gtkPicture.set_filename(Some(file_path));
    }

    fn set_pictures_for_multiple_view(
        &self,
        application_window: &ApplicationWindow,
        controller: &Controller,
        pictures_per_row: usize,
    ) {
        let cells_per_row: i32 = pictures_per_row as i32;
        let navigator = controller.navigator();
        let gallery = controller.gallery();
        let grid = view_stack(application_window)
            .visible_child()
            .unwrap()
            .downcast::<gtk::ScrolledWindow>()
            .unwrap()
            .first_child()
            .unwrap()
            .first_child()
            .unwrap()
            .downcast::<gtk::Grid>()
            .unwrap()
            .child_at(1, 0)
            .unwrap()
            .downcast::<gtk::Grid>()
            .unwrap();
        for col in 0..cells_per_row {
            for row in 0..cells_per_row {
                let coords = (row as usize, col as usize);
                if let Some(index) = controller
                    .navigator()
                    .position_from_coords(coords.0, coords.1) {
                        let cell: gtk::Box = grid.child_at(col, row).unwrap()
                            .downcast::<gtk::Box>().unwrap();
                        while let Some(child) = cell.first_child() {
                            cell.remove(&child);
                        }
                        let gtkPicture = make_picture();
                        let picture = gallery.picture(index);
                        gtkPicture.set_filename(Some(picture.view_file_path(true)));
                        cell.append(&gtkPicture);
                        println!("…{}", picture.view_file_path(true));
                }
            }
        }
        println!("{:?}", grid);
    }

    pub fn set_pictures(
        &self,
        application_window: &gtk::ApplicationWindow,
        controller: &Controller,
    ) {
        let pictures_per_row = controller.state().pictures_per_row();
        if pictures_per_row == 1 {
            self.set_picture_for_single_view(application_window, controller)
        } else {
            self.set_pictures_for_multiple_view(application_window, controller, pictures_per_row)
        }
    }
}
