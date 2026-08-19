use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::gui::main_controller::MainController;
use gtk::Application;
use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::prelude::ApplicationExt;

pub type GsrApplication = gtk::Application;

pub fn make_gsr_application(
    application_id: &str,
    main_controller: MainController,
    clargs: CommandLineArguments,
    position: usize,
) -> GsrApplication {
    let gsr_application = Application::builder()
        .application_id(application_id)
        .build();
    gsr_application.connect_startup(|_| startup_gui());
    connect_activate_application(&gsr_application, clargs, position, main_controller);
    gsr_application
}

fn connect_activate_application(
    gsr_application: &GsrApplication,
    clargs: CommandLineArguments,
    position: usize,
    main_controller: MainController,
) {
    let controller_rc_opt = main_controller.controller_rc_opt().clone();
    if let Some(controller_rc) = controller_rc_opt {
        gsr_application.connect_activate(clone!(
            #[strong]
            clargs,
            #[strong]
            controller_rc,
            move |gsr_application: &GsrApplication| {
                GsrApplicationWindow::new(gsr_application, &clargs);
            }
        ));
    } else {
        panic!("controller_rc is not set")
    }
}

fn startup_gui() {
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
