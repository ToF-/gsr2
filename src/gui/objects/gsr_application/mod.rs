use crate::cli::command_line_arguments::CommandLineArguments;
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use crate::gui::view::main_view::MainView;
use gtk::Application;
use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::prelude::ApplicationExt;
use gtk::subclass::prelude::*;
mod imp;

use glib::Object;
use gtk::gio;
use gtk::glib;

glib::wrapper! {
    pub struct GsrApplication(ObjectSubclass<imp::GsrApplication>)
        @extends gio::Application, gtk::Application,
        @implements gtk::gio::ActionGroup, gtk::gio::ActionMap;
}

impl GsrApplication {
    pub fn new(application_id: &str, main_controller: &MainController) -> Self {
        let obj: Self = Object::builder()
            .property("application-id", application_id)
            .build();
        obj.set_main_controller_rc(main_controller);
        obj
    }

    pub fn make_gsr_application(application_id: &str, main_controller: &MainController) -> Self {
        let application = GsrApplication::new(application_id, main_controller);
        application.connect_startup(|_| startup_gui());
        application
    }
    pub fn main_controller_rc(&self) -> RcMainController {
        self.imp().main_controller_rc()
    }

    pub fn set_main_controller_rc(&self, main_controller: &MainController) {
        self.imp().set_main_controller_rc(main_controller)
    }

    pub fn connect_activation(&self, clargs: CommandLineArguments, position: usize) {
        let main_controller = self.main_controller_rc().borrow().clone();
        let controller_rc_opt = main_controller.controlller_rc_opt().clone();
        if let Some(controller_rc) = controller_rc_opt {
            self.connect_activate(clone!(
                #[strong]
                clargs,
                #[strong]
                controller_rc,
                move |gsr_application: &GsrApplication| {
                    MainView::activate(
                        gsr_application,
                        &clargs,
                        &controller_rc,
                        position,
                        &main_controller,
                    )
                }
            ));
        } else {
            panic!("controller_rc is not set")
        }
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
