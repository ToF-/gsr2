use gtk::prelude::*;
use gtk::gdk;
use gtk::Application;
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
pub fn launch_application(cli: CommandLineInterface) {
    println!("launching app…");
    let application = Application::builder()
        .application_id("org.example.gsr2")
        .build();
    application.connect_startup(|application| {
        startup_gui(application);
    });
    let no_args: Vec<String> = vec![];
    application.run_with_args(&no_args);
}
