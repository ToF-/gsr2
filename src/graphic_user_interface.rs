use gtk::prelude::*;
use gtk::gdk;
use gtk::{Align, Application, ApplicationWindow, Orientation, Picture, ScrolledWindow};
use crate::command_line_interface::CommandLineInterface;

pub fn startup_gui(_application: &gtk::Application) {
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data("window { background-color:black;} image { margin:1em ; } label { color:white; }");
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css_provider,
        1000,
    );
}

pub fn build_gui(application: &gtk::Application) {
    let application_window = ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(400)
        .default_height(400)
        .build();
    let single_view_scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .name("view")
        .build();
    let view_box = gtk::Box::new(Orientation::Vertical, 0);
    view_box.set_valign(Align::Fill);
    view_box.set_halign(Align::Fill);
    view_box.set_hexpand(true);
    view_box.set_vexpand(true);
    view_box.set_homogeneous(false);

    let picture = Picture::new();
    picture.set_hexpand(true);
    picture.set_vexpand(true);

    view_box.append(&picture);
    single_view_scrolled_window.set_child(Some(&view_box));

    let view_stack = gtk::Stack::new();
    view_stack.set_hexpand(true);
    view_stack.set_vexpand(true);
    let _ = view_stack.add_child(&single_view_scrolled_window);
    view_stack.set_visible_child(&single_view_scrolled_window);
    application_window.set_child(Some(&view_stack));
    application_window.present()
}

pub fn launch_application(cli: CommandLineInterface) {
    println!("launching app…");
    let application = Application::builder()
        .application_id("org.example.gsr2")
        .build();
    application.connect_startup(|application| { startup_gui(application); });
    application.connect_activate(move |application: &gtk::Application| {
        build_gui(application)
    });
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
