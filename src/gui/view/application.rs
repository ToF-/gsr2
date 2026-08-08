use crate::gui::gsr_application::GsrApplication;
use crate::gui::main_controller::MainController;
use gtk::Application;
use gtk::gdk::Display;
use gtk::prelude::ApplicationExt;

pub fn make_gsr_application(
    application_id: &str,
    main_controller: &MainController,
) -> GsrApplication {
    let application = GsrApplication::new(application_id, main_controller);
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
        }
        listview.catalog {
            background-color:black;
            }
        listview.catalog treeexpander expander {
            color: white;
        }
        ",
    );
    gtk::style_context_add_provider_for_display(&Display::default().unwrap(), &css_provider, 1000);
}
