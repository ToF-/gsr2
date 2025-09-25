use crate::CommandLineInterface;
use crate::application_state::ApplicationState;
use crate::gui::view::View;
use gtk::glib::clone;
use crate::gui::components::make_application;
use gtk::gdk;
use gtk::Application;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Controller {
    args:  CommandLineInterface,
    state: ApplicationState,
    view:  View,
}

impl Controller {

pub fn startup_gui(&self, _application: &gtk::Application) {
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

pub fn build_and_run_application(&self, cli: CommandLineInterface) {
    let application: gtk::Application = make_application("example.org.gsr2");

    application.connect_startup(|application| {
        self.startup_gui(application);
    });
    let cli_rc = Rc::new(RefCell::new(cli));
    // clone! passes a strong reference to a variable in the closure that activates the application
    // move converts any variables captured by reference or mutable reference to variables captured by value.
    application.connect_activate(
        clone!(@strong cli_rc, => move |application: &gtk::Application| {
        activate(application, &cli_rc); }),
    );
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
}
