use crate::Controller;
use crate::gui::control::Control;
use crate::env::default_values::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::gui::direction::Direction;
use crate::gui::display::picture_label_display;
use crate::model::gen_image::no_thumbnail_picture;
use crate::gui::controller::RcController;
use crate::gui::event::Event::{KeyPressed, NextSlideDelay, PaneClicked, PictureClicked};
use crate::file::paths::check_path_exists;
use crate::model::picture::Picture;
use gtk::Window;
use gtk::gio::File;
use gtk::glib::clone;
use gtk::glib::timeout_add_local;
use gtk::prelude::*;
use gtk::{self};
use gtk::{Align, Application, ApplicationWindow, Grid, gdk};
use gtk::{CssProvider, Label, Orientation, Picture as GtkPicture, ScrolledWindow};
use std::cell::RefCell;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

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

    pub fn set_picture_for_single_view(controller: &Controller) {
        let picture: Picture = controller.current_picture();
        let picture_file_path = picture.file_path();
        let application_window_rc = controller.view().application_window_rc();
        let application_window = application_window_rc.borrow();
        let gtkPicture = if let Ok(file_path) = check_path_exists(&PathBuf::from(picture_file_path))
        {
            Self::picture_from_file_path(file_path)
        } else {
            no_thumbnail_picture()
        };
        Self::set_single_view_picture(&application_window, &controller, &gtkPicture);
    }

    pub fn set_label_for_current_picture(controller: &Controller, with_focus: bool) {
        let navigator = controller.navigator();
        let position = navigator.position();
        let picture = controller.current_picture();
        let application_window_rc = controller.view().application_window_rc();
        let application_window = application_window_rc.borrow();
        if !controller.state().single_view() {
            if let Some((row, col)) = navigator.coords_from_position(position) {
                let grid = Self::multiple_view_grid(&application_window);
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

    fn set_pictures_for_multiple_view(controller: &Controller, pictures_per_row: usize) {
        let cells_per_row: i32 = pictures_per_row as i32;
        let navigator = controller.navigator();
        let gallery = controller.gallery();
        let application_window_rc = controller.view().application_window_rc();
        let application_window = application_window_rc.borrow();
        let grid = Self::multiple_view_grid(&application_window);
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
                    cell.append(&gtkPicture);
                    let label = Self::make_label_for_picture(&picture, is_focus);
                    cell.append(&label);
                }
            }
        }
    }

    pub fn set_pictures(_application_window: &gtk::ApplicationWindow, controller: &Controller) {
        if controller.state().single_view() {
            Self::set_picture_for_single_view(&controller)
        } else {
            let pictures_per_row = controller.state().pictures_per_row();
            Self::set_pictures_for_multiple_view(&controller, pictures_per_row)
        }
    }
    // basic settings when starting up gtk application
    pub fn startup_gui(_application: &gtk::Application) {
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data(
            "window { background-color:black;} image { margin:1em ; } label { color:white; }",
        );
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &css_provider,
            1000,
        );
    }

    // make the application, defining its start up method and its activate method
    // on activation, it will build the components, which will all use a RefCell on the controller
    // to pass control of events
    pub fn make_application(application_id: &str, controller_rc: RcController) -> gtk::Application {
        let application = Application::builder()
            .application_id(application_id)
            .build();
        application.connect_startup(|application| Self::startup_gui(application));
        application.connect_activate(clone!(@strong controller_rc
        => move |application: &gtk::Application| {
            Self::make_application_components(&application, &controller_rc)
        }));
        application
    }

    // create all the components, attach event managers to them, then setup the view part of the
    // controller so that we have these references available.
    // Controller → View → ApplicationWindow → components → Controller
    // Controller has a counted reference on the View, it can manipulate and change some components
    // visibility, eg. switching frow single to multiple view
    // View has a reference to the ApplicationWindow
    // ApplicationWindow has components
    // components have event managers attached to them
    // event manager have a counted reference on the controller and send it an event message
    //
    pub fn make_application_components(
        application: &gtk::Application,
        controller_rc: &RcController,
    ) {
        let pictures_per_row: i32;
        let application_window = Self::make_application_window(application, controller_rc);
        {
            let controller = controller_rc.try_borrow().expect("can't borrow");
            pictures_per_row = controller.state().pictures_per_row() as i32;
            Self::setup_components(&application_window, pictures_per_row);
        }
        if let Ok(controller) = controller_rc.try_borrow_mut() {
            let view = controller.view();
            let slideshow = controller.args().slideshow();
            Self::attach_events(&application_window, &view, slideshow, controller_rc);
            Self::attach_grid_picture_events(pictures_per_row, &application_window, controller_rc);
        } else {
            panic!("can't borrow mut");
        };
        if let Ok(mut controller) = controller_rc.try_borrow_mut() {
            let application_window_rc = Rc::new(RefCell::new(application_window.clone()));
            let mut view = controller.view();
            view.set_application_window_rc(application_window_rc);
            controller.set_view(view.clone());
        } else {
            panic!("can't borrow mut");
        };
        {
            let controller = controller_rc.try_borrow().expect("can't borrow");
            let _view = controller.view();
            View::set_pictures(&application_window, &controller);
        }
        application_window.present();
    }

    // attach event mananger to some components
    pub fn attach_events(
        application_window: &gtk::ApplicationWindow,
        _view: &View,
        slideshow_opt: Option<i32>,
        controller_rc: &RcController,
    ) {
        let left_pane = Self::left_pane(application_window);
        let right_pane = Self::right_pane(application_window);

        let gesture_left_click = gtk::GestureClick::new();
        gesture_left_click.set_button(1);
        gesture_left_click.connect_pressed(
            clone!(@strong controller_rc, @strong application_window, => move |_,_,_,_| {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller.process_event(
                        PaneClicked { button: 1, pane_number: LEFT_PANE },
                        &application_window,
                        &controller_rc);
                } else {
                    panic!("can't borrow mut controller");
                }
            }),
        );
        left_pane.add_controller(gesture_left_click);

        let gesture_right_click = gtk::GestureClick::new();
        gesture_right_click.set_button(1);
        gesture_right_click.connect_pressed(
            clone!(@strong controller_rc, @strong application_window => move |_,_,_,_| {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller.process_event(
                        PaneClicked { button: 1, pane_number: RIGHT_PANE },
                        &application_window,
                        &controller_rc);
                } else {
                    panic!("can't borrow mut controller");
                }

            }),
        );
        right_pane.add_controller(gesture_right_click);

        let evk = gtk::EventControllerKey::new();
        evk.connect_key_pressed(
            clone!(@strong controller_rc, @strong application_window => move |_, key, key_code, modifier_type| {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller.process_event(
                        KeyPressed { key, key_code, modifier_type },
                        &application_window,
                        &controller_rc);
                };
                gtk::Inhibit(false)
            }),
        );
        application_window.add_controller(evk);
        if let Some(seconds) = slideshow_opt {
            Self::attach_slideshow_event(seconds, application_window, controller_rc)
        }
    }

    pub fn attach_slideshow_event(
        seconds: i32,
        application_window: &gtk::ApplicationWindow,
        controller_rc: &RcController,
    ) {
        let delay: u64 = seconds.try_into().unwrap();
        println!("setting slideshow delay to {} seconds", delay);
        timeout_add_local(
            Duration::new(delay, 0),
            clone!(@strong controller_rc, @strong application_window => move | | {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    if controller.state().slideshow_on() {
                        controller.process_event(
                            NextSlideDelay,
                            &application_window,
                            &controller_rc)
                    };
                    Continue(controller.state().slideshow_on())
                } else {
                    panic!("can't borrow mut controller")
                }
            }),
        );
    }

    pub fn attach_grid_picture_events(
        cells_per_row: i32,
        window: &gtk::ApplicationWindow,
        controller_rc: &RcController,
    ) {
        let grid = Self::multiple_view_grid(window);
        for col in 0..cells_per_row {
            for row in 0..cells_per_row {
                let cell_box: gtk::Box = grid
                    .child_at(col, row)
                    .unwrap()
                    .downcast::<gtk::Box>()
                    .unwrap();
                let gesture_left_click = gtk::GestureClick::new();
                gesture_left_click.set_button(1);
                gesture_left_click.connect_pressed(clone!(@strong col, @strong row, @strong controller_rc, @strong window => move |_,_,_,_| {
                    if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                        controller.process_event(
                            PictureClicked { button: 1, col, row },
                            &window,
                            &controller_rc);
                    }
                }));
                cell_box.add_controller(gesture_left_click);
                let gesture_right_click = gtk::GestureClick::new();
                gesture_right_click.set_button(3);
                gesture_right_click.connect_pressed(clone!(@strong col, @strong row, @strong controller_rc, @strong window => move |_,_,_,_| {
                    if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                        controller.process_event(
                            PictureClicked { button: 3, col, row },
                            &window,
                            &controller_rc);
                    }
                }));
                cell_box.add_controller(gesture_right_click);
            }
        }
    }

    pub fn setup_components(application_window: &gtk::ApplicationWindow, pictures_per_row: i32) {
        let grid = Self::make_grid(pictures_per_row);

        let panel = Self::make_panel(&grid);

        let multiple_view_scrolled_window = Self::make_multiple_view_scrolled_window();
        multiple_view_scrolled_window.set_child(Some(&panel));

        assert_eq!(Self::panel_grid(&multiple_view_scrolled_window), panel);

        let frame = Self::make_frame();
        let picture = Self::make_picture();
        frame.append(&picture);
        frame.append(&Self::make_label());
        let single_view_scrolled_window = Self::make_single_view_scrolled_window();
        single_view_scrolled_window.set_child(Some(&frame));

        let view_stack = Self::make_stack();
        let _ = view_stack.add_named(&single_view_scrolled_window, Some("single_view"));
        let _ = view_stack.add_named(&multiple_view_scrolled_window, Some("multiple_view"));
        if pictures_per_row == 1 {
            view_stack.set_visible_child(&single_view_scrolled_window);
        } else {
            view_stack.set_visible_child(&multiple_view_scrolled_window);
        }
        application_window.set_child(Some(&view_stack));
    }

    pub fn full_size_arrow_move(&self, direction: Direction) {
        let step: f64 = 100.0;
        let application_window_rc = self.application_window_rc();
        let application_window = application_window_rc.borrow();
        let w = Self::single_view_scrolled_window(&application_window);
        let wh_adj = w.hadjustment();
        let wv_adj = w.vadjustment();
        match direction {
            Direction::Right => wh_adj.set_value(wh_adj.value() + step),
            Direction::Left => wh_adj.set_value(wh_adj.value() - step),
            Direction::Down => wv_adj.set_value(wv_adj.value() + step),
            Direction::Up => wv_adj.set_value(wv_adj.value() - step),
            _ => {}
        }
    }

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
        println!("{:?}", application_window.first_child());
        println!(
            "{:?}",
            application_window.first_child().map(|w| w.next_sibling())
        );
        window.set_transient_for(Some(application_window));
        println!("{:?}", window);
        println!("{:?}", application_window.first_child());
        println!(
            "{:?}",
            application_window.first_child().map(|w| w.next_sibling())
        );
        window.present();
        window
    }

    pub fn make_application_window(
        application: &gtk::Application,
        controller_rc: &RcController,
    ) -> gtk::ApplicationWindow {
        let controller = controller_rc.borrow();
        let args = controller.args();
        ApplicationWindow::builder()
            .application(application)
            .title("gsr2")
            .default_width(args.width.unwrap())
            .default_height(args.height.unwrap())
            .build()
    }
    #[allow(dead_code)]
    pub fn make_palette_area() -> gtk::DrawingArea {
        let palette_area = gtk::DrawingArea::new();
        palette_area.set_valign(Align::Center);
        palette_area.set_halign(Align::Center);
        palette_area
    }
    pub fn make_single_view_scrolled_window() -> gtk::ScrolledWindow {
        ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .build()
    }
    pub fn make_frame() -> gtk::Box {
        gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .homogeneous(false)
            .build()
    }

    pub fn make_picture() -> gtk::Picture {
        GtkPicture::builder().hexpand(true).vexpand(true).build()
    }

    pub fn picture_from_file_path(file_path: &Path) -> gtk::Picture {
        GtkPicture::builder()
            .file(&File::for_path(file_path))
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    pub fn make_label() -> gtk::Label {
        let label = gtk::Label::new(None);
        label.set_valign(Align::Center);
        label.set_halign(Align::Center);
        label
    }

    pub fn make_label_for_picture(picture: &Picture, with_focus: bool) -> gtk::Label {
        let label = gtk::Label::new(Some(&picture_label_display(&picture.label(), with_focus)));
        label.set_valign(Align::Center);
        label.set_halign(Align::Center);
        label
    }

    pub fn make_stack() -> gtk::Stack {
        gtk::Stack::builder().hexpand(true).vexpand(true).build()
    }

    pub fn make_multiple_view_scrolled_window() -> gtk::ScrolledWindow {
        ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .build()
    }

    pub fn make_grid(cells_per_row: i32) -> gtk::Grid {
        let grid = gtk::Grid::builder()
            .row_homogeneous(true)
            .column_homogeneous(true)
            .hexpand(true)
            .vexpand(true)
            .name("grid")
            .build();
        Self::attach_cells(&grid, cells_per_row);
        grid
    }

    pub fn attach_cells(grid: &gtk::Grid, cells_per_row: i32) {
        for col in 0..cells_per_row {
            for row in 0..cells_per_row {
                let cell_box = Self::make_cell_box();
                grid.attach(&cell_box, col, row, 1, 1);
            }
        }
    }

    pub fn remove_cells(grid: &gtk::Grid, cells_per_row: i32) {
        for col in 0..cells_per_row {
            for row in 0..cells_per_row {
                let cell_box = grid.child_at(col, row).unwrap();
                grid.remove(&cell_box);
            }
        }
    }

    pub fn make_panel(view_grid: &gtk::Grid) -> gtk::Grid {
        let panel = Grid::new();
        panel.set_hexpand(true);
        panel.set_vexpand(true);
        panel.set_row_homogeneous(true);
        panel.set_column_homogeneous(false);
        let buttons_css_provider = CssProvider::new();
        buttons_css_provider.load_from_data(
            "
            label {
                color: gray;
                font-size: 12px;
            }
            text-button {
                background-color: black;
            }
        ",
        );
        let left_pane = Label::new(Some("←"));
        let right_pane = Label::new(Some("→"));
        left_pane.set_width_chars(10);
        right_pane.set_width_chars(10);
        left_pane.style_context().add_provider(
            &buttons_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        right_pane.style_context().add_provider(
            &buttons_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        panel.attach(&left_pane, 0, 0, 1, 1);
        panel.attach(view_grid, 1, 0, 1, 1);
        panel.attach(&right_pane, 2, 0, 1, 1);
        panel
    }

    #[allow(dead_code)]
    pub fn make_picture_for(file_path: &str, opacity: f64, can_shrink: bool) -> gtk::Picture {
        let gtk_picture = gtk::Picture::new();
        gtk_picture.set_halign(Align::Center);
        gtk_picture.set_valign(Align::Center);
        gtk_picture.set_opacity(opacity);
        gtk_picture.set_can_shrink(can_shrink);
        gtk_picture.set_filename(Some(file_path));
        gtk_picture.set_visible(true);
        gtk_picture
    }

    #[allow(dead_code)]
    pub fn make_pane_with_label(symbol: &str) -> gtk::Label {
        let buttons_css_provider = CssProvider::new();
        buttons_css_provider.load_from_data(
            "
            label {
                color: gray;
                font-size: 12px;
                }
            text-button {
                background-color: black;
                }
        ",
        );
        let label = Label::new(Some(symbol));
        label.set_width_chars(10);
        label.style_context().add_provider(
            &buttons_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        label
    }

    pub fn make_cell_box() -> gtk::Box {
        gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .valign(Align::Center)
            .halign(Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build()
    }

    pub fn view_stack(application_window: &gtk::ApplicationWindow) -> gtk::Stack {
        application_window
            .first_child()
            .unwrap()
            .downcast::<gtk::Stack>()
            .unwrap()
    }

    pub fn visible_stack_child_scrolled_window(stack: &gtk::Stack) -> gtk::ScrolledWindow {
        stack
            .visible_child()
            .unwrap()
            .downcast::<gtk::ScrolledWindow>()
            .unwrap()
    }

    pub fn frame(window: &gtk::ScrolledWindow) -> gtk::Box {
        let child = window.first_child().unwrap().first_child().unwrap();
        child.downcast::<gtk::Box>().unwrap()
    }

    pub fn picture(cell_box: &gtk::Box) -> gtk::Picture {
        cell_box
            .first_child()
            .unwrap()
            .downcast::<gtk::Picture>()
            .unwrap()
    }

    pub fn single_view_picture(application_window: &gtk::ApplicationWindow) -> gtk::Picture {
        Self::picture(&Self::frame(&Self::visible_stack_child_scrolled_window(
            &Self::view_stack(application_window),
        )))
    }

    pub fn remove_children_from_box(cell_box: &gtk::Box) {
        while let Some(child) = cell_box.first_child() {
            cell_box.remove(&child)
        }
    }
    pub fn set_single_view_picture(
        application_window: &gtk::ApplicationWindow,
        controller: &Controller,
        picture: &gtk::Picture,
    ) {
        let frame = &Self::frame(&Self::visible_stack_child_scrolled_window(
            &Self::view_stack(application_window),
        ));
        while let Some(child) = frame.first_child() {
            frame.remove(&child)
        }
        let state = controller.state();
        if state.expand_on() {
            picture.set_valign(Align::Fill);
            picture.set_halign(Align::Fill);
        } else {
            picture.set_valign(Align::Center);
            picture.set_halign(Align::Center);
        };
        picture.set_can_shrink(!state.full_size_on());
        frame.append(picture);
    }

    #[allow(dead_code)]
    pub fn single_view_picture_label(application_window: &gtk::ApplicationWindow) -> gtk::Label {
        let picture = Self::picture(&Self::frame(&Self::visible_stack_child_scrolled_window(
            &Self::view_stack(application_window),
        )));
        picture
            .next_sibling()
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
    }

    pub fn panel_grid(window: &gtk::ScrolledWindow) -> gtk::Grid {
        let viewport: gtk::Viewport = window.child().unwrap().downcast::<gtk::Viewport>().unwrap();
        let panel = viewport.child().unwrap().downcast::<gtk::Grid>().unwrap();
        panel
    }

    pub fn multiple_view_scrolled_window(
        application_window: &gtk::ApplicationWindow,
    ) -> gtk::ScrolledWindow {
        let stack = Self::view_stack(application_window);
        stack
            .child_by_name("multiple_view")
            .unwrap()
            .downcast::<gtk::ScrolledWindow>()
            .unwrap()
    }

    pub fn single_view(application_window: &gtk::ApplicationWindow) -> bool {
        let child_name = Self::view_stack(application_window)
            .visible_child_name()
            .unwrap();
        child_name == "single_view"
    }

    pub fn toggle_view_stack(application_window: &gtk::ApplicationWindow) {
        let view_stack = Self::view_stack(application_window);
        if Self::single_view(application_window) {
            view_stack.set_visible_child_name("multiple_view")
        } else {
            view_stack.set_visible_child_name("single_view")
        }
    }

    #[allow(dead_code)]
    pub fn single_view_scrolled_window(
        application_window: &gtk::ApplicationWindow,
    ) -> gtk::ScrolledWindow {
        let stack = Self::view_stack(application_window);
        stack
            .child_by_name("single_view")
            .unwrap()
            .downcast::<gtk::ScrolledWindow>()
            .unwrap()
    }

    pub fn left_pane(application_window: &gtk::ApplicationWindow) -> gtk::Label {
        let panel_grid = Self::panel_grid(&Self::multiple_view_scrolled_window(application_window));
        panel_grid
            .child_at(0, 0)
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
    }

    pub fn right_pane(application_window: &gtk::ApplicationWindow) -> gtk::Label {
        let panel_grid = Self::panel_grid(&Self::multiple_view_scrolled_window(application_window));
        panel_grid
            .child_at(2, 0)
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
    }
    pub fn multiple_view_grid(application_window: &gtk::ApplicationWindow) -> gtk::Grid {
        let panel_grid = Self::panel_grid(&Self::multiple_view_scrolled_window(application_window));
        panel_grid
            .child_at(1, 0)
            .unwrap()
            .downcast::<gtk::Grid>()
            .unwrap()
    }
}
