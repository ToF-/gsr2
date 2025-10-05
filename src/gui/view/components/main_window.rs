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
use gtk::prelude::Cast;
use gtk::prelude::GestureSingleExt;
use gtk::prelude::GridExt;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use std::cell::RefCell;

pub const LEFT_PANE: usize = 0;
pub const RIGHT_PANE: usize = 1;

#[derive(Clone, Debug)]
pub struct MainWindow {
    picture_grid_ref: RefCell<PictureGrid>,
    picture_frame_ref: RefCell<PictureFrame>,
    application_window_ref: RefCell<gtk::ApplicationWindow>,
    stack_ref: RefCell<gtk::Stack>,
    frame_window_ref: RefCell<gtk::ScrolledWindow>,
    grid_window_ref: RefCell<gtk::ScrolledWindow>,
}

impl MainWindow {
    pub fn new(application: &gtk::Application, controller_rc: &RcController) -> Self {
        let controller = controller_rc.borrow();
        let args = controller.args();
        let pictures_per_row = args.pictures_per_row();
        let picture_grid = PictureGrid::new(pictures_per_row, controller_rc);
        let picture_frame = PictureFrame::new(controller_rc);
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
        let application_window = make_application_window(application, controller_rc);
        application_window.set_child(Some(&view_stack));
        attach_panel_event_handlers(&panel, controller_rc);

        MainWindow {
            picture_grid_ref: RefCell::new(picture_grid),
            picture_frame_ref: RefCell::new(picture_frame),
            application_window_ref: RefCell::new(application_window),
            stack_ref: RefCell::new(view_stack),
            frame_window_ref: RefCell::new(single_view_scrolled_window.clone()),
            grid_window_ref: RefCell::new(single_view_scrolled_window.clone()),
        }
    }

    pub fn application_window(&self) -> gtk::ApplicationWindow {
        self.application_window_ref.borrow().clone()
    }

    pub fn picture_grid(&self) -> PictureGrid {
        self.picture_grid_ref.borrow().clone()
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

fn make_application_window(
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
                        button: button,
                        pane_number: pane_number,
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
