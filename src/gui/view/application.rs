use gtk::Application;
use gtk::gdk::Display;
use gtk::prelude::ApplicationExt;

pub fn make_application(application_id: &str) -> gtk::Application {
    let application = Application::builder()
        .application_id(application_id)
        .build();
    application.connect_startup(|_| startup_gui());
    application
}
//
// basic settings when starting up gtk application
pub fn startup_gui() {
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_string(
        "window { background-color:black;} 
        image { margin:1em ; } 
        label { color:white; 
                font-family:sans-serif;
                font-size:12px;}
        label.pane {
            color: gray;
            font-size: 22px;
            background-color:black;
        }
        label.entry {
            padding: 10px;
            font-size: 32px;
        }"
    );
    gtk::style_context_add_provider_for_display(&Display::default().unwrap(), &css_provider, 1000);
}
