use crate::cli::command_line_arguments::CommandLineArguments;
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use crate::gui::view::main_window::MainWindow;
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
                    MainWindow::activate(
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
