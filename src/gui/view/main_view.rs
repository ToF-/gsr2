use std::cell::RefCell;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::default_values::FOCUS_SYMBOL_1;
use crate::env::default_values::QUARTER_OPACITY;
use crate::env::default_values::{FULL_OPACITY, HALF_OPACITY};
use crate::file::paths::check_path_exists;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::action::gio_action_type::GioActionType;
use crate::gui::control::Control;
use crate::gui::control::default_controls;
use crate::gui::controller::Controller;
use crate::gui::controller::RcController;
use crate::gui::direction::Direction;
use crate::gui::display::title_display;
use crate::gui::event::Event::{KeyPressed, NextSlideDelay, PaneClicked};
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use crate::gui::mode::Mode;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_picture_cell_box::GsrPictureCellBox;
use crate::gui::view::entry_window::EntryWindow;
use crate::gui::view::grid_view::GridView;
use crate::gui::view::picture_frame::PictureFrame;
use crate::gui::view::treelist_view::TreeListView;
use crate::model::catalog::Catalog;
use crate::model::change::Change;
use crate::model::picture::Picture;
use crate::model::rank::Rank;
use crate::model::thumbnail::no_thumbnail_picture;
use gtk::gdk::ModifierType;
use gtk::gio;
use gtk::gio::ActionEntry;
use gtk::gio::File as GtkFile;
use gtk::gio::prelude::*;
use gtk::gio::prelude::*;
use gtk::glib;
use gtk::glib::Variant;
use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use gtk::glib::timeout_add_local;
use gtk::glib::{ControlFlow, Propagation};
use gtk::prelude::ActionGroupExt;
use gtk::prelude::AdjustmentExt;
use gtk::prelude::BoxExt;
use gtk::prelude::ToVariant;
#[allow(deprecated)]
use gtk::prelude::{
    ApplicationExtManual, Cast, GestureSingleExt, GridExt, GtkApplicationExt, GtkWindowExt,
    WidgetExt,
};
use gtk::{ApplicationWindow, Grid, Label, Picture as GtkPicture, ScrolledWindow};
use palette_extract::Color;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

pub const LEFT_PANE: usize = 0;
pub const RIGHT_PANE: usize = 1;

#[derive(Clone, Debug)]
pub struct MainView {
    grid_view_rc: RefCell<GridView>,
    picture_frame: PictureFrame,
    application_window: gtk::ApplicationWindow,
    stack: gtk::Stack,
    frame_window: gtk::ScrolledWindow,
    controller_rc: RcController,
}

impl MainView {
    // pub fn new(application: &gtk::Application, args: &CommandLineArguments, controller_rc: &RcController) -> Self {
    //     // main_view_opt_rc.borrow().clone().unwrap()
    // }

    pub fn new_from_application(
        application: &GsrApplication,
        controller_rc: &RcController,
    ) -> Self {
        let application_window = application
            .active_window()
            .expect("can't get application active window")
            .downcast::<ApplicationWindow>()
            .expect("can't downcast application window");

        let stack = application_window
            .first_child()
            .expect("can't get stack")
            .downcast::<gtk::Stack>()
            .expect("can't downcast stack");

        let single_view_scrolled_window = stack
            .child_by_name("single_view")
            .expect("can't get single view scrolled window")
            .downcast::<gtk::ScrolledWindow>()
            .expect("can't downcast single view scrolled window");

        let multiple_view_scrolled_window = stack
            .child_by_name("multiple_view")
            .expect("can't get multiple view scrolled window")
            .downcast::<gtk::ScrolledWindow>()
            .expect("can't downcast multiple view scrolled window");

        let single_view_port = single_view_scrolled_window
            .first_child()
            .expect("can't get single view frame")
            .downcast::<gtk::Viewport>()
            .expect("can't donwcast view port");

        let frame = single_view_port
            .first_child()
            .expect("can't get view port first child")
            .downcast::<gtk::Box>()
            .expect("can't downcast frame as box");

        let multiple_view_port = multiple_view_scrolled_window
            .first_child()
            .expect("can't get multiple view frame")
            .downcast::<gtk::Viewport>()
            .expect("can't donwcast view port");

        let grid = multiple_view_port
            .first_child()
            .expect("can't get multiple view panel")
            .downcast::<gtk::Grid>()
            .expect("can't downcast panel to grid")
            .child_at(1, 0)
            .expect("can't find panel central child")
            .downcast::<gtk::Grid>()
            .expect("can't dowcast panel central child to grid");

        let grid_view = GridView::new_from_grid(&grid, controller_rc);
        let picture_frame = PictureFrame::new_from_frame(&frame);

        MainView {
            grid_view_rc: RefCell::new(grid_view),
            picture_frame: picture_frame.clone(),
            application_window: application_window.clone(),
            stack: stack.clone(),
            frame_window: single_view_scrolled_window.clone(),
            controller_rc: controller_rc.clone(),
        }
    }

    pub fn activate(
        application: &GsrApplication,
        clargs: &CommandLineArguments,
        controller_rc: &RcController,
        position: usize,
        main_controller: &MainController,
    ) {
        let (pictures_per_row, palette_on) = if let Ok(controller) = controller_rc.try_borrow() {
            (controller.state().pictures_per_row(),
            controller.state().palette_on())
        } else {
            panic!("can't borrow")
        };
        let grid_view = GridView::new(
            pictures_per_row.try_into().unwrap(),
            palette_on,
            controller_rc,
            main_controller,
        );
        let picture_frame = PictureFrame::new();
        let single_view_scrolled_window = make_scrolled_window();
        let multiple_view_scrolled_window = make_scrolled_window();
        let panel = make_panel(&grid_view.grid());
        let frame: gtk::Box = picture_frame.frame();
        single_view_scrolled_window.set_child(Some(&frame));
        multiple_view_scrolled_window.set_child(Some(&panel));
        let view_stack = make_stack();
        let _ = view_stack.add_named(&single_view_scrolled_window, Some("single_view"));
        let _ = view_stack.add_named(&multiple_view_scrolled_window, Some("multiple_view"));
        if pictures_per_row == 1 {
            view_stack.set_visible_child(&single_view_scrolled_window);
        } else {
            view_stack.set_visible_child(&multiple_view_scrolled_window);
        }
        let application_window = make_application_window(application, clargs);
        application_window.set_child(Some(&view_stack));
        {
            let main_view = MainView::new_from_application(application, controller_rc);
            if let Ok(controller) = controller_rc.try_borrow() {
                controller.set_main_view(main_view);
                let mut navigator = controller.navigator();
                if navigator.can_move(Direction::Index { value: position }) {
                    navigator.move_towards(Direction::Index { value: position });
                    navigator.set_page_changed();
                    controller.set_navigator(navigator);
                }
            }
            if let Ok(mut controller) = controller_rc.try_borrow() {
                controller.main_view().set_pictures(&mut controller);
                controller.main_view().set_title(&controller);
            }
        };
        attach_panel_event_handlers(&panel, controller_rc);
        add_actions(&application_window, controller_rc);
        // TESTING
        application_window
            .insert_action_group("main-controller", Some(&main_controller.gio_action_group()));
        application.set_accels_for_action("main-controller.toggle-thumbnails-view", &["<Ctrl>T"]);
        application.set_accels_for_action("application-group.close", &["<Ctrl>W"]);
        application.set_accels_for_action("main-controller.test::foobardelaw", &["<Ctrl>Z"]);
        attach_key_pressed_event_handlers(&application_window, controller_rc);
        if let Some(seconds) = clargs.slideshow {
            Self::attach_slideshow_event(seconds, controller_rc);
        }
        application_window.present();
    }

    pub fn run_application(application: GsrApplication, main_controller_rc: RcMainController) {
        let no_args: Vec<String> = vec![];
        application.run_with_args(&no_args);
    }

    pub fn application_window(&self) -> ApplicationWindow {
        self.application_window.clone()
    }

    pub fn picture_frame(&self) -> PictureFrame {
        self.picture_frame.clone()
    }

    pub fn frame_window(&self) -> gtk::ScrolledWindow {
        self.frame_window.clone()
    }

    pub fn stack(&self) -> gtk::Stack {
        self.stack.clone()
    }

    pub fn set_title(&self, controller: &Controller) {
        let title = match controller.state().mode() {
            Mode::View => title_display(controller),
            Mode::Setting(choice) => match choice {
                Control::SetDisplay => {
                    String::from("Display… (d:date on | s:size on | f:focus change on)")
                }
                Control::SetOrder => String::from(
                    "Order… (c: by color count | d: by date | l: by label | p: by palette | n: by name | r: randomize | s: by size | v: by value)",
                ),
                Control::SetMark => String::from("Mark current picture… (a | b | c | d | e)"),
                Control::GotoMark => String::from("Destination picture… (a | b | c | d | e)"),
                _ => panic!("incorrect choice for setting: {:?}", choice),
            },
            Mode::Editing => String::from("Editing…"),
            Mode::Categorizing => String::from("Pick category to apply"),
            Mode::SelectingCategory => String::from("Pick category to view"),
            Mode::AddingCategory => String::from("Pick category to add the new category to"),
            Mode::MovingCategory => String::from("Pick category to move"),
            Mode::MovingToCategory => String::from("Pick category to move the category to"),
            Mode::RemovingCategory => String::from("Pick category to remove"),
            Mode::FindingSubCategory => String::from("Pick category to find"),
            Mode::SelectingSubCategory => String::from("Pick category to select"),
        };
        self.application_window().set_title(Some(&title));
    }

    pub fn set_pictures_for_multiple_view(&self, controller: &Controller, pictures_per_row: i32, palette_on: bool) {
        let navigator = controller.navigator();
        dbg!("set_pictures_for_multiple_view");
        dbg!(&navigator);
        if let Ok(gallery) = controller.repository().gallery_rc().try_borrow() {
            let grid_view = self.grid_view_rc.borrow();
            let grid = grid_view.grid();
            for col in 0..pictures_per_row {
                for row in 0..pictures_per_row {
                    let coords = (row as usize, col as usize);
                    let cell = match grid.child_at(col, row) {
                        Some(widget) => widget.downcast::<GsrPictureCellBox>().unwrap(),
                        None => GsrPictureCellBox::new(col, row, pictures_per_row, palette_on).into(),
                    };
                    if let Some(index) = navigator.position_from_coords(coords.0, coords.1) {
                        let picture = gallery.picture(index);
                        let has_focus = index == navigator.position();
                        grid_view.set_picture_at(col, row, &picture, has_focus);
                        let opacity: f64 = if let Some(position) =
                            navigator.position_from_coords(row as usize, col as usize)
                        {
                            if navigator.is_selected(position) {
                                HALF_OPACITY
                            } else if gallery.selection_criteria().is_empty()
                                || gallery.selection_criteria().matches(picture.tags())
                            {
                                FULL_OPACITY
                            } else {
                                QUARTER_OPACITY
                            }
                        } else {
                            FULL_OPACITY
                        };
                        grid_view.set_picture_opacity_at(col, row, opacity);
                        // self.grid_view.set_label_text_at(&picture, col, row, with_focus);
                    }
                }
            }
        } else {
            panic!("can't borrow");
        }
    }

    pub fn set_picture_for_single_view(&self, controller: &Controller) {
        let picture: Picture = controller.current_picture();
        let picture_file_path = picture.file_path();
        let gtk_picture =
            if let Ok(file_path) = check_path_exists(&PathBuf::from(picture_file_path.clone())) {
                gtk_picture_from_file_path(file_path)
            } else {
                no_thumbnail_picture()
            };
        let picture_frame = self.picture_frame();
        picture_frame.set_picture(controller, &gtk_picture);
        controller.increment_picture_score(&picture_file_path);
        self.set_title(controller);
    }

    pub fn set_pictures(&self, controller: &Controller) {
        if controller.state().single_view() {
            self.set_picture_for_single_view(controller)
        } else {
            let pictures_per_row = controller.state().pictures_per_row();
            let palette_on = controller.state().palette_on();
            self.set_pictures_for_multiple_view(controller, pictures_per_row as i32, palette_on)
        }
    }
    pub fn set_label_text_for_current_picture(
        &self,
        controller: &Controller,
        with_focus: Option<char>,
    ) {
        let navigator = controller.navigator();
        let position = navigator.position();
        let picture = controller.current_picture();
        if !controller.state().single_view()
            && let Some((row, col)) = navigator.coords_from_position(position)
        {
            let grid_view = self.grid_view_rc.borrow();
            grid_view.set_label_text_at(&picture, col as i32, row as i32, with_focus);
        }
    }

    pub fn set_focus_for_current_picture(
        &self,
        controller: &Controller,
    ) {
        let navigator = controller.navigator();
        let position = navigator.position();
        if !controller.state().single_view() 
            && let Some((row, col)) = navigator.coords_from_position(position)
        {
            let grid_view = self.grid_view_rc.borrow();
            grid_view.set_focus_at(col as i32, row as i32)
        }
    }

    pub fn set_opacity_for_current_picture(&self, controller: &Controller, opacity: f64) {
        let navigator = controller.navigator();
        let position = navigator.position();
        if !controller.state().single_view()
            && let Some((row, col)) = navigator.coords_from_position(position)
        {
            let grid_view = self.grid_view_rc.borrow();
            grid_view.set_picture_opacity_at(col as i32, row as i32, opacity);
        }
    }

    pub fn single_view(&self) -> bool {
        let stack = self.stack();
        let child_name = stack.visible_child_name().unwrap();
        child_name == "single_view"
    }

    pub fn popup_entry_window(&self, prompt: &str, text: &str) -> EntryWindow {
        let entry_window = EntryWindow::new(
            &self.application_window(),
            prompt,
            text,
            &self.controller_rc,
        );
        entry_window.popup();
        entry_window
    }

    pub fn popup_treelist_view(&self, prompt: &str, catalog: &Catalog) -> TreeListView {
        let treelist_view = TreeListView::new(
            &self.application_window(),
            prompt,
            "",
            catalog,
            &self.controller_rc,
        );
        treelist_view.popup();
        treelist_view
    }
    pub fn change_grid_size(&mut self, pictures_per_row: usize, palette_on: bool, main_controller: &MainController) {
        let grid_view = self.grid_view_rc.borrow();
        grid_view.change_dimension(pictures_per_row as i32, palette_on, main_controller)
    }

    pub fn toggle_view_stack(&self, controller: &Controller) {
        let stack = self.stack();
        if controller.state().single_view() {
            stack.set_visible_child_name("single_view")
        } else {
            stack.set_visible_child_name("multiple_view")
        }
    }
    pub fn full_size_arrow_move(&self, direction: Direction) {
        let step: f64 = 100.0;
        let window = self.frame_window();
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

    pub fn reattach_slideshow_event(&self, seconds: i32) {
        Self::attach_slideshow_event(seconds, &self.controller_rc);
    }

    pub fn attach_slideshow_event(seconds: i32, controller_rc: &RcController) {
        let delay: u64 = seconds.try_into().unwrap();
        println!("setting slideshow delay to {} seconds", delay);
        timeout_add_local(
            Duration::new(delay, 0),
            clone!(
                #[strong]
                controller_rc,
                move || {
                    if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                        if controller.state().slideshow_on() {
                            controller.process_event(NextSlideDelay);
                            ControlFlow::Continue
                        } else {
                            ControlFlow::Break
                        }
                    } else {
                        panic!("can't borrow mut controller")
                    }
                }
            ),
        );
    }
}

fn make_application_window(
    application: &GsrApplication,
    clargs: &CommandLineArguments,
) -> ApplicationWindow {
    let gsrWindow = ApplicationWindow::new(application);
    gsrWindow.set_title(Some("gsr2"));
    gsrWindow.set_default_width(clargs.width.unwrap());
    gsrWindow.set_default_height(clargs.height.unwrap());
    gsrWindow
    /*
    ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(args.width.unwrap())
        .default_height(args.height.unwrap())
        .build()
    */
}
#[allow(deprecated)]
fn make_panel(view_grid: &gtk::Grid) -> gtk::Grid {
    let panel = Grid::new();
    panel.set_hexpand(true);
    panel.set_vexpand(true);
    panel.set_row_homogeneous(true);
    panel.set_column_homogeneous(false);
    let left_pane = Label::new(Some("←"));
    let right_pane = Label::new(Some("→"));
    left_pane.set_width_chars(5);
    left_pane.add_css_class("pane");
    right_pane.set_width_chars(5);
    right_pane.add_css_class("pane");
    panel.attach(&left_pane, 0, 0, 1, 1);
    panel.attach(view_grid, 1, 0, 1, 1);
    panel.attach(&right_pane, 2, 0, 1, 1);
    panel
}

fn make_scrolled_window() -> gtk::ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build()
}

fn make_stack() -> gtk::Stack {
    gtk::Stack::builder().hexpand(true).vexpand(true).build()
}

pub fn left_pane(panel_grid: &gtk::Grid) -> gtk::Label {
    panel_grid
        .child_at(0, 0)
        .unwrap()
        .downcast::<gtk::Label>()
        .unwrap()
}

pub fn right_pane(panel_grid: &gtk::Grid) -> gtk::Label {
    panel_grid
        .child_at(2, 0)
        .unwrap()
        .downcast::<gtk::Label>()
        .unwrap()
}

fn pane_gesture_click(
    button: usize,
    pane_number: usize,
    controller_rc: &RcController,
) -> gtk::GestureClick {
    let gesture_click = gtk::GestureClick::new();
    gesture_click.set_button(1);
    gesture_click.connect_pressed(clone!(
        #[strong]
        controller_rc,
        move |_, _, _, _| {
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                controller.process_event(PaneClicked {
                    button,
                    pane_number,
                });
            } else {
                panic!("can't borrow mut controller");
            }
        }
    ));
    gesture_click
}

fn attach_panel_event_handlers(panel: &gtk::Grid, controller_rc: &RcController) {
    left_pane(panel).add_controller(pane_gesture_click(1, LEFT_PANE, controller_rc));
    right_pane(panel).add_controller(pane_gesture_click(1, RIGHT_PANE, controller_rc));
}

fn add_actions(application_window: &ApplicationWindow, controller_rc: &RcController) {
    let action_close = ActionEntry::builder("close")
        .activate(clone!(
            #[weak]
            controller_rc,
            move |_obj, _simple_action, _variant_opt| {
                if let Ok(controller) = controller_rc.try_borrow() {
                    controller.quit();
                }
            }
        ))
        .build();
    let actions = gtk::gio::SimpleActionGroup::new();
    actions.add_action_entries([action_close]);
    application_window.insert_action_group("application-group", Some(&actions));
}

fn attach_key_pressed_event_handlers(
    application_window: &ApplicationWindow,
    controller_rc: &RcController,
) {
    let event_controller_key = gtk::EventControllerKey::new();
    event_controller_key.connect_key_pressed(clone!(
        #[strong]
        application_window,
        #[strong]
        controller_rc,
        move |_, key, _key_code, _modifier_type| {
            if let Some(key_name) = key.name() {
                let mode = if let Ok(controller) = controller_rc.try_borrow() {
                    controller.state().mode()
                } else {
                    panic!("can't borrow controller");
                };
                if let Some(control) =
                    default_controls().get(&(key_name.clone().into(), mode.clone()))
                {
                    let action = Action::from_control(control);
                    let gio_action = GioAction::from(action);
                    let (name, variant) = gio_action.to_simple_action_call();
                    match gtk::prelude::WidgetExt::activate_action(
                        &application_window,
                        &name,
                        variant.as_ref(),
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            dbg!("{}:{}", e, name);
                        }
                    }
                } else {
                    println!("no control on key {key_name:?}");
                };
            }
            Propagation::Stop
        }
    ));
    application_window.add_controller(event_controller_key);
}

pub fn gtk_picture_from_file_path(file_path: &Path) -> gtk::Picture {
    GtkPicture::builder()
        .file(&GtkFile::for_path(file_path))
        .hexpand(true)
        .vexpand(true)
        .build()
}

pub fn remove_children_from_box(cell_box: &gtk::Box) {
    while let Some(child) = cell_box.first_child() {
        cell_box.remove(&child)
    }
}

#[allow(dead_code)]
pub fn make_entry_window(application_window: &ApplicationWindow, prompt: &str) -> gtk::Window {
    let window: gtk::Window = gtk::Window::builder()
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
