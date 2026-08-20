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
    pub fn set_state(&self, clargs: CommandLineArguments, controller: Controller) {
        dbg!("set_state");
        *self.command_line_arguments.borrow_mut() = Some(Rc::new(RefCell::new(clargs.clone())));

        let main_controller = MainController::new(Some(Rc::new(RefCell::new(controller))));
        *self.main_controller.borrow_mut() = Some(Rc::new(RefCell::new(main_controller)));

        let configuration = CONFIGURATION.get().unwrap();
        *self.configuration.borrow_mut() = Some(Rc::new(RefCell::new(configuration.clone())));

        let mut view = View::default();
        let current_picture_file_path = &CONFIGURATION.get().unwrap().current_picture;
        view.set_pictures_per_row(clargs.pictures_per_row());
        *self.view.borrow_mut() = Some(Rc::new(RefCell::new(view)));

        let navigator = Navigator::default();
        *self.navigator.borrow_mut() = Some(Rc::new(RefCell::new(navigator)));

        let controller_rc = self.shared_main_controller()
            .borrow()
            .controller_opt_rc
            .borrow()
            .clone()
            .expect("controller not initialized");
        let controller = controller_rc.borrow();
        let gallery = &controller.repository().gallery_rc().borrow().clone();
        *self.gallery.borrow_mut() = Some(Rc::new(RefCell::new(gallery.clone())));
    }
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
        (*self.command_line_arguments.borrow())
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn shared_configuration(&self) -> Shared<Configuration> {
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
        let app = self.obj();
        let gsr_application_window = GsrApplicationWindow::new(&app);
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

impl GtkApplicationImpl for GsrApplication {}
