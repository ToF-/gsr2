use crate::env::configuration::CONFIGURATION;
use crate::gui::view::View;
use crate::gui::controller::Controller;
use std::rc::Rc;
use crate::env::configuration::Configuration;
use crate::env::default_values::APPLICATION_ID;
use gtk::gdk::Display;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::gui::main_controller::MainController;
use std::cell::RefCell;
use gtk::glib::clone;
mod imp;

use gtk::glib;
use gtk::gio;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct GsrApplication(ObjectSubclass<imp::GsrApplication>)
        @extends gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}


impl Default for GsrApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APPLICATION_ID)
            .build()
    }
}
impl GsrApplication { 
    pub fn set_state(&self,
        clargs: CommandLineArguments,
        controller: Controller,
        ) {
        *self.imp().command_line_arguments.borrow_mut() = Some(Rc::new(RefCell::new(clargs.clone())));
        let main_controller = MainController::new(Some(Rc::new(RefCell::new(controller))));
        *self.imp().main_controller.borrow_mut() = Some(Rc::new(RefCell::new(main_controller)));
        let mut view = View::default();
        let current_picture_file_path = &CONFIGURATION.get().unwrap().current_picture;
        view.set_pictures_per_row(clargs.pictures_per_row());
        *self.imp().view.borrow_mut() = Some(Rc::new(RefCell::new(view)));
    }
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
