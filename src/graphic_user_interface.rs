use crate::command_line_interface::Command::File;
use crate::command_line_interface::CommandLineInterface;
use crate::default_values::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use gtk::gdk::Key;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Orientation, Picture, ScrolledWindow, gdk};
use std::cell::RefCell;
use std::rc::Rc;

struct GraphicalUserInterface {
    application_window: gtk::ApplicationWindow,
    single_view_picture: gtk::Picture,
}

type RcRefCellGui = Rc<RefCell<GraphicalUserInterface>>;

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

fn set_picture_for_file_view(gui: &GraphicalUserInterface, cli: &CommandLineInterface) {
    let picture = &gui.single_view_picture;
    picture.set_valign(Align::Center);
    picture.set_halign(Align::Center);
    picture.set_opacity(1.00);
    if let Some(File { file_name }) = &cli.command {
        println!("{}", file_name);
        picture.set_filename(Some(file_name));
    } else {
        println!("no picture file to display")
    }
}
pub fn build_gui(application: &gtk::Application, cli: &CommandLineInterface) {
    let application_window = ApplicationWindow::builder()
        .application(application)
        .title("gsr2")
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
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

    let gui = GraphicalUserInterface {
        application_window,
        single_view_picture: picture,
    };
    let evk = gtk::EventControllerKey::new();
    let gui_rc = Rc::new(RefCell::new(gui));
    evk.connect_key_pressed(clone!(@strong gui_rc => move |_, key, _, _| {
        process_key(&gui_rc, key)
    }));
    if let Ok(gui) = gui_rc.try_borrow() {
        set_picture_for_file_view(&gui, cli);
        gui.application_window.add_controller(evk);
        gui.application_window.present()
    }
}

fn process_key(gui_rc: &RcRefCellGui, key: Key) -> gtk::Inhibit {
    if let Ok(gui) = gui_rc.try_borrow_mut() {
        if let Some(key_name) = key.name() {
            if key_name.as_str() == "q" {
                gui.application_window.close()
            };
        }
    };
    gtk::Inhibit(false)
}
pub fn launch_application(cli: CommandLineInterface) {
    println!("launching app…");
    let application = Application::builder()
        .application_id("org.example.gsr2")
        .build();
    application.connect_startup(|application| {
        startup_gui(application);
    });
    application
        .connect_activate(move |application: &gtk::Application| build_gui(application, &cli));
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
