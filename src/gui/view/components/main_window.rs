#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::GridExt;
use gtk::prelude::WidgetExt;
use gtk::prelude::GtkWindowExt;
use crate::gui::view::PictureFrame;
use crate::gui::view::PictureGrid;
use crate::gui::view::RcController;
use gtk::ApplicationWindow;
use gtk::CssProvider;
use gtk::Grid;
use gtk::Label;
use gtk::ScrolledWindow;
use std::cell::RefCell;

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
        let frame: gtk::Box = *picture_frame.frame_ref().borrow();
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
        MainWindow {
            picture_grid_ref: RefCell::new(picture_grid),
            picture_frame_ref: RefCell::new(picture_frame),
            application_window_ref: RefCell::new(application_window),
            stack_ref: RefCell::new(view_stack),
            frame_window_ref: RefCell::new(single_view_scrolled_window),
            grid_window_ref: RefCell::new(single_view_scrolled_window),
        }
    }

    pub fn application_window(&self) -> gtk::ApplicationWindow {
        *self.application_window_ref.borrow()
    }

    pub fn picture_grid(&self) -> PictureGrid {
        *self.picture_grid_ref.borrow()
    }

    pub fn picture_frame(&self) -> PictureFrame {
        *self.picture_frame_ref.borrow()
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
