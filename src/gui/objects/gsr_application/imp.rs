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
        gsr_application_window.initialize();
        gsr_application_window.present();
    }
}

impl GtkApplicationImpl for GsrApplication {
}


