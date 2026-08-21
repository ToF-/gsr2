use crate::env::configuration::Configuration;
use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_application::CONFIGURATION;
use crate::gui::objects::gsr_application::CommandLineArguments;
use crate::gui::objects::gsr_application::Controller;
use crate::gui::objects::gsr_application::MainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::view::View;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::{glib, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::rc::Rc;

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
    // stored for sharing: command line args, view state, navigator and gallery
    pub fn set_state(&self, clargs: CommandLineArguments, controller: &Controller) {
        dbg!("set_state");
        *self.command_line_arguments.borrow_mut() = Some(Rc::new(RefCell::new(clargs.clone())));

        let mut view = View::default();
        let _current_picture_file_path = &CONFIGURATION.get().unwrap().current_picture;
        view.set_pictures_per_row(clargs.pictures_per_row());
        *self.view.borrow_mut() = Some(Rc::new(RefCell::new(view.clone())));

        let gallery = &controller.repository().gallery_rc().borrow().clone();
        *self.gallery.borrow_mut() = Some(Rc::new(RefCell::new(gallery.clone())));

        let navigator = Navigator::new(gallery.len(), view.pictures_per_row() as usize);
        *self.navigator.borrow_mut() = Some(Rc::new(RefCell::new(navigator)));
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
        let app = self.obj();
        let gsr_application_window = GsrApplicationWindow::new(&app);
        gsr_application_window.initialize();
        gsr_application_window.present();
    }
}

impl GtkApplicationImpl for GsrApplication {}
