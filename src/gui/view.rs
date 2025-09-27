use crate::control::Control;
use crate::Controller;
use crate::gen_image::no_thumbnail_picture;
use crate::gui::components::*;
use crate::gui::controller::RcController;
use crate::paths::check_path_exists;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow};
use gtk::gdk::Key;
use std::cell::RefCell;
use std::path::PathBuf;
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

    pub fn setup_components(&self, application_window: &gtk::ApplicationWindow) {
        let grid = make_grid(self.cells_per_row);

        let panel = make_panel(&grid);

        let multiple_view_scrolled_window = make_multiple_view_scrolled_window();
        multiple_view_scrolled_window.set_child(Some(&panel));

        assert_eq!(panel_grid(&multiple_view_scrolled_window),panel);

        let frame = make_frame();
        let picture = make_picture();
        frame.append(&picture);
        let single_view_scrolled_window = make_single_view_scrolled_window();
        single_view_scrolled_window.set_child(Some(&frame));

        let view_stack = make_stack();
        let _ = view_stack.add_named(&single_view_scrolled_window,Some("single_view"));
        let _ = view_stack.add_named(&multiple_view_scrolled_window,Some("multiple_view"));
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
                view.set_pictures(&application_window, controller_rc);
                application_window.present()
            } else {
                panic!("cannot borrow");
            }
        } else {
            panic!("cannot borrow");
        }
    }

    pub fn attach_events(&self, window: &gtk::ApplicationWindow, controller_rc: &RcController) {
        let stack = view_stack(window);
        let single_view_scrolled_window = single_view_scrolled_window(window);
        let multiple_view_scrolled_window = multiple_view_scrolled_window(window);
        let panel = multiple_view_scrolled_window.first_child().unwrap();
        let left_pane = left_pane(window);
        let right_pane = right_pane(window);

        let gesture_left_click = gtk::GestureClick::new();
        let view = self;
        gesture_left_click.set_button(1);
        gesture_left_click.connect_pressed(clone!(@strong controller_rc, @strong view, @strong window, => move |_,_,_,_| {
            let mut refresh = false;
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                refresh = controller.process(&Control::MovePrev)
            };
            if refresh {
                view.set_pictures(&window, &controller_rc)
            };
        }));
        left_pane.add_controller(gesture_left_click);

        let gesture_right_click = gtk::GestureClick::new();
        gesture_right_click.set_button(1);
        gesture_right_click.connect_pressed(clone!(@strong controller_rc, @strong view, @strong window => move |_,_,_,_| {
            let mut refresh = false;
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                refresh = controller.process(&Control::MoveNext)
            };
            if refresh {
                view.set_pictures(&window, &controller_rc)
            };
        }));
        right_pane.add_controller(gesture_right_click);

        let evk = gtk::EventControllerKey::new();
        let view = self;
        evk.connect_key_pressed(clone!(@strong controller_rc, @strong view, @strong window => move |_, key, _, _| {
            let mut refresh = false;
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                refresh = controller.process_key(key);
            }
            if refresh {
                view.set_pictures(&window, &controller_rc)
            };
            gtk::Inhibit(false)
        }));
        window.add_controller(evk);
    }

    fn set_picture_for_single_view(
        &self,
        application_window: &ApplicationWindow,
        controller: &Controller,
    ) {
        let gallery = controller.gallery();
        let gtkPicture: gtk::Picture = single_view_picture(application_window);
        let file_path = controller.current_picture().file_path();
        gtkPicture.set_filename(Some(file_path));
    }

    // fn set_picture_for_cell_at(&self,
    //     application_window: &ApplicationWindow,
    //     controller: &Controller,
    //     col: usize, row: usize) {
    //     let navigator_rc = controller.navigator_rc();
    //     let navigator = navigator_rc.borrow();
    //     let gallery = controller.gallery();

    //     ddlet widget = gui.multiple_view_grid.child_at(col as i32, row as i32).expect("cannot find cell box in multiple view grid");
    //     let cell_box = widget.downcast::<gtk::Box>().expect("cannot downcast widget to Box");

    //     while let Some(child) = cell_box.first_child() {
    //         cell_box.remove(&child)
    //     };
    //     if let Some(index) = catalog.index_from_position((col,row)) {
    //         if !catalog.discarded().contains(&index) {
    //             let entry = catalog.entry_at_index(index).unwrap();
    //             let picture = picture_for_entry(entry, catalog);
    //             let label = label_for_entry(entry, index == catalog.index().unwrap());
    //             cell_box.append(&picture);
    //             cell_box.append(&label);
    //         }
    //     }
    // }

    fn set_pictures_for_multiple_view(
        &self,
        application_window: &ApplicationWindow,
        controller: &Controller,
        pictures_per_row: usize,
    ) {
        let cells_per_row: i32 = pictures_per_row as i32;
        let navigator_rc = controller.navigator_rc();
        let navigator = navigator_rc.borrow();
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
                while let Some(child) = cell.first_child() {
                    cell.remove(&child)
                };
                if let Some(index) = navigator.position_from_coords(coords.0, coords.1) {
                    let picture = gallery.picture(index);
                    let gtkPicture =
                        match check_path_exists(&PathBuf::from(picture.view_file_path(true))) {
                            Ok(file_path) => {
                                let gtkPicture = make_picture();
                                gtkPicture.set_filename(Some(file_path));
                                gtkPicture
                            }
                            Err(_) => no_thumbnail_picture(),
                        };
                    cell.append(&gtkPicture);
                }
            }
        }
    }

    pub fn set_pictures(
        &self,
        application_window: &gtk::ApplicationWindow,
        controller_rc: &RcController
    ) {
        let controller = controller_rc.borrow();
        let pictures_per_row = controller.state().pictures_per_row();
        if pictures_per_row == 1 {
            self.set_picture_for_single_view(application_window, &controller)
        } else {
            self.set_pictures_for_multiple_view(application_window, &controller, pictures_per_row)
        }
    }
}
