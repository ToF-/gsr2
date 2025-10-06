use crate::gui::event::Event::NextSlideDelay;
use gtk::glib::ControlFlow;
use std::time::Duration;
 use crate::gui::view::timeout_add_local;
use gtk::glib::Propagation;
use crate::gui::event::Event::KeyPressed;
use crate::View;
use crate::Args;
use crate::gui::event::Event::PaneClicked;
use crate::gui::view::PictureFrame;
use crate::gui::view::PictureGrid;
use crate::gui::view::RcController;
use gtk::ApplicationWindow;
use gtk::CssProvider;
use gtk::Grid;
use gtk::Label;
use gtk::ScrolledWindow;
use gtk::glib::clone;
use gtk::prelude::ApplicationExtManual;
use gtk::prelude::Cast;
use gtk::prelude::GestureSingleExt;
use gtk::prelude::GridExt;
use gtk::prelude::GtkApplicationExt;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use std::cell::RefCell;
use std::rc::Rc;

pub const LEFT_PANE: usize = 0;
pub const RIGHT_PANE: usize = 1;

#[derive(Clone, Debug)]
pub struct MainWindow {
    picture_grid_ref: Rc<RefCell<PictureGrid>>,
    picture_frame_ref: Rc<RefCell<PictureFrame>>,
    application_window_ref: Rc<RefCell<gtk::ApplicationWindow>>,
    stack_ref: Rc<RefCell<gtk::Stack>>,
    frame_window_ref: Rc<RefCell<gtk::ScrolledWindow>>,
}

impl MainWindow {
    // pub fn new(application: &gtk::Application, args: &Args, controller_rc: &RcController) -> Self {
    //     // main_window_opt_rc.borrow().clone().unwrap()
    // }

    pub fn new_from_application(
        application: &gtk::Application,
        args: &Args,
        controller_rc: &RcController,
    ) -> Self {
        let pictures_per_row: i32 = args.pictures_per_row() as i32;
        let application_window = application
            .active_window()
            .expect("can't get application active window")
            .downcast::<gtk::ApplicationWindow>()
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

        let picture_grid = PictureGrid::new_from_grid(&grid, pictures_per_row, controller_rc);
        let picture_frame = PictureFrame::new_from_frame(&frame);

        MainWindow {
            picture_grid_ref: Rc::new(RefCell::new(picture_grid.clone())),
            picture_frame_ref: Rc::new(RefCell::new(picture_frame.clone())),
            application_window_ref: Rc::new(RefCell::new(application_window.clone())),
            stack_ref: Rc::new(RefCell::new(stack.clone())),
            frame_window_ref: Rc::new(RefCell::new(single_view_scrolled_window.clone())),
        }
    }

    pub fn activate(application: &gtk::Application, args: &Args, controller_rc: &RcController) {
        let pictures_per_row = args.pictures_per_row();
        let picture_grid = PictureGrid::new(pictures_per_row, controller_rc);
        let picture_frame = PictureFrame::new();
        let single_view_scrolled_window = make_scrolled_window();
        let multiple_view_scrolled_window = make_scrolled_window();
        let panel = make_panel(&picture_grid.grid_ref().borrow());
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
        let application_window = make_application_window(application, args);
        application_window.set_child(Some(&view_stack));
        {
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                let main_window = MainWindow::new_from_application(application, args, controller_rc);
                let view = View::new(&main_window);
                controller.set_view(view);
                controller.view().set_pictures(&controller);
                controller.view().set_title(&controller);
            }
        }
        attach_panel_event_handlers(&panel, controller_rc);
        attach_key_pressed_event_handlers(&application_window, controller_rc);
        if let Some(seconds) = args.slideshow {
            attach_slideshow_event(seconds, controller_rc);
        }
        application_window.present();
    }

    pub fn run_application(application: gtk::Application) {
        let no_args: Vec<String> = vec![];
        application.run_with_args(&no_args);
    }

    pub fn application_window(&self) -> gtk::ApplicationWindow {
        self.application_window_ref.borrow().clone()
    }

    pub fn picture_grid(&self) -> PictureGrid {
        self.picture_grid_ref.borrow().clone()
    }

    pub fn picture_grid_ref(&self) -> Rc<RefCell<PictureGrid>> {
        self.picture_grid_ref.clone()
    }

    pub fn picture_frame(&self) -> PictureFrame {
        self.picture_frame_ref.borrow().clone()
    }

    pub fn frame_window(&self) -> gtk::ScrolledWindow {
        self.frame_window_ref.borrow().clone()
    }

    pub fn stack(&self) -> gtk::Stack {
        self.stack_ref.borrow().clone()
    }
}

fn make_application_window(application: &gtk::Application, args: &Args) -> gtk::ApplicationWindow {
    ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(args.width.unwrap())
        .default_height(args.height.unwrap())
        .build()
}
#[allow(deprecated)]
fn make_panel(view_grid: &gtk::Grid) -> gtk::Grid {
    let panel = Grid::new();
    panel.set_hexpand(true);
    panel.set_vexpand(true);
    panel.set_row_homogeneous(true);
    panel.set_column_homogeneous(false);
    let buttons_css_provider = CssProvider::new();
    buttons_css_provider.load_from_string(
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
                controller.process_event(
                    PaneClicked {
                        button,
                        pane_number,
                    },
                    &controller_rc,
                );
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

fn attach_key_pressed_event_handlers(application_window: &gtk::ApplicationWindow, controller_rc: &RcController) {
    let event_controller_key = gtk::EventControllerKey::new();
    event_controller_key.connect_key_pressed(clone!(
        #[strong] controller_rc,
        move |_, key, key_code, modifier_type| {
            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                controller.process_event(
                    KeyPressed {
                        key,
                        key_code,
                        modifier_type,
                    },
                    &controller_rc);
            };
            Propagation::Stop
        }));
    application_window.add_controller(event_controller_key);
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
                        controller.process_event(NextSlideDelay, &controller_rc)
                    };
                    if controller.state().slideshow_on() {
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
