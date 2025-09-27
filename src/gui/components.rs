use crate::default_values::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use gtk::prelude::*;
use gtk::{self};
use gtk::{Align, Application, ApplicationWindow, Grid, gdk};
use gtk::{CssProvider, Label, Orientation, Picture, ScrolledWindow};

pub fn startup_gui(application: &gtk::Application) {
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
pub fn make_application(application_id: &str) -> gtk::Application {
    Application::builder()
        .application_id(application_id)
        .build()
}

pub fn make_application_window(application: &gtk::Application) -> gtk::ApplicationWindow {
    ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .build()
}
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
    Picture::builder().hexpand(true).vexpand(true).build()
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
    for col in 0..cells_per_row {
        for row in 0..cells_per_row {
            let cell_box = make_cell_box();
            grid.attach(&cell_box, col, row, 1, 1);
        }
    }
    grid
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
    picture(&frame(&visible_stack_child_scrolled_window(&view_stack(
        application_window,
    ))))
}

pub fn panel_grid(window: &gtk::ScrolledWindow) -> gtk::Grid {
    let viewport: gtk::Viewport = window.child().unwrap().downcast::<gtk::Viewport>().unwrap();
    let panel = viewport.child().unwrap().downcast::<gtk::Grid>().unwrap();
    panel
}

pub fn multiple_view_scrolled_window(application_window: &gtk::ApplicationWindow) -> gtk::ScrolledWindow {
    let stack = view_stack(application_window);
    stack.child_by_name("multiple_view").unwrap().downcast::<gtk::ScrolledWindow>().unwrap()
}

pub fn single_view_scrolled_window(application_window: &gtk::ApplicationWindow) -> gtk::ScrolledWindow {
    let stack = view_stack(application_window);
    stack.child_by_name("single_view").unwrap().downcast::<gtk::ScrolledWindow>().unwrap()
}

pub fn left_pane(application_window: &gtk::ApplicationWindow) -> gtk::Label {
    let panel_grid = panel_grid(
        &multiple_view_scrolled_window(application_window));
    panel_grid.child_at(0,0).unwrap().downcast::<gtk::Label>().unwrap()
}

pub fn right_pane(application_window: &gtk::ApplicationWindow) -> gtk::Label {
    let panel_grid = panel_grid(
        &multiple_view_scrolled_window(application_window));
    panel_grid.child_at(2,0).unwrap().downcast::<gtk::Label>().unwrap()
}
pub fn multiple_view_grid(application_window: &gtk::ApplicationWindow) -> gtk::Grid {
    let panel_grid = panel_grid(
        &multiple_view_scrolled_window(application_window));
    panel_grid.child_at(1,0).unwrap().downcast::<gtk::Grid>().unwrap()
}
