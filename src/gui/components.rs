use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, CssProvider, Label, Orientation, Picture,
    ScrolledWindow, Text, gdk,
};

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
pub fn make_view_box() -> gtk::Box {
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

pub fn make_view_stack() -> gtk::Stack {
    gtk::Stack::builder().hexpand(true).vexpand(true).build()
}

pub fn make_multiple_view_scrolled_window() -> gtk::ScrolledWindow {
    ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build()
}

pub fn make_multiple_view_grid() -> gtk::Grid {
    let grid = gtk::Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .hexpand(true)
        .vexpand(true)
        .build();
    grid
}

pub fn make_multiple_view_panel() -> gtk::Grid {
    gtk::Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(false)
        .hexpand(true)
        .vexpand(true)
        .build()
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

pub fn make_label(symbol: &str) -> gtk::Label {
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
