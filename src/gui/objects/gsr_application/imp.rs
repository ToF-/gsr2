use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::model::gallery::Gallery;
use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_application::MainController;
use crate::env::configuration::Configuration;
use crate::gui::objects::gsr_application::CommandLineArguments;
use crate::gui::view::View;
use crate::model::shared::Shared;
use std::cell::RefCell;
use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Default)]
pub struct GsrApplication {
    pub command_line_arguments: RefCell<Option<Shared<CommandLineArguments>>>,
    pub configuration: RefCell<Option<Shared<Configuration>>>,
    pub main_controller: RefCell<Option<Shared<MainController>>>,
    pub view: RefCell<Option<Shared<View>>>,
    pub navigator: RefCell<Option<Shared<Navigator>>>,
    pub gallery: RefCell<Option<Shared<Gallery>>>,
}

impl GsrApplication {
    pub fn shared_view(&self) -> Shared<View> {
        (*self.view.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_navigator(&self) -> Shared<Navigator> {
        (*self.navigator.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_gallery(&self) -> Shared<Gallery> {
        (*self.gallery.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_command_line_arguments(&self) -> Shared<CommandLineArguments> {
        (*self.command_line_arguments.borrow()).as_ref().unwrap().clone()
    }
    pub fn shared_configuration(&self) ->  Shared<Configuration> {
        (*self.configuration.borrow()).as_ref().unwrap().clone()
    }
    pub fn shared_main_controller(&self) -> Shared<MainController> {
        (*self.main_controller.borrow()).as_ref().unwrap().clone()
    }

}
#[glib::object_subclass]
impl ObjectSubclass for GsrApplication {
    const NAME: &'static str = "GsrApplication";

    type Type = super::GsrApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for GsrApplication {}
impl ApplicationImpl for GsrApplication {

    fn activate(&self) {
        dbg!("imp activate");
        let app = self.obj();
        let gsr_application_window = GsrApplicationWindow::new(&app);
        dbg!("foo");
        gsr_application_window.imp().set_state(
            self.shared_command_line_arguments(),
            self.shared_configuration(),
            self.shared_main_controller(),
            self.shared_view(),
            self.shared_navigator(),
            self.shared_gallery(),
            );
        gsr_application_window.initialize();
        gsr_application_window.present();
    }
}

impl GtkApplicationImpl for GsrApplication {
}


