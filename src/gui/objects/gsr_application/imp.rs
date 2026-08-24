use crate::env::configuration::Configuration;
use crate::gui::objects::gsr_application::CONFIGURATION;
use crate::gui::objects::gsr_application::CommandLineArguments;
use crate::gui::objects::gsr_application::Controller;
use crate::gui::objects::gsr_application::MainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::view::View;
use crate::gui::view_state::navigator::Navigator;
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

// GSR_APPLICATION
impl GsrApplication {
    // stored for sharing: command line args, view state, navigator and gallery
    pub fn set_state(&self, clargs: CommandLineArguments, gallery: &Gallery) {
        // store clargs
        *self.command_line_arguments.borrow_mut() = Some(Rc::new(RefCell::new(clargs.clone())));

        let mut view = View::from_command_line_arguments(&clargs);
        // no grid or thumbnails option, try cfg
        if clargs.grid.is_none()
            && !clargs.thumbnails
            && let Some(pictures_per_row) = CONFIGURATION
                .get()
                .expect("Configuration not set")
                .current_pictures_per_row
        {
            view.set_pictures_per_row(pictures_per_row as i32)
        };
        dbg!(&view);
        let _current_picture_file_path = &CONFIGURATION.get().unwrap().current_picture;
        *self.view.borrow_mut() = Some(Rc::new(RefCell::new(view.clone())));

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

// ACTIVATE
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
