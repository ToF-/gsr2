use crate::env::configuration::Configuration;
use crate::gui::direction::Direction;
use crate::gui::objects::gsr_application::CONFIGURATION;
use crate::gui::objects::gsr_application::CommandLineArguments;
use crate::gui::objects::gsr_application::MainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::view_state::ViewState;
use crate::gui::view_state::navigator::Navigator;
use crate::model::gallery::Gallery;
use crate::model::repository::Repository;
use crate::model::shared::Shared;
use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Default)]
pub struct GsrApplication {
    pub command_line_arguments: Shared<CommandLineArguments>,
    pub configuration: Shared<Configuration>,
    pub repository: Shared<Option<Repository>>,
    pub main_controller: Shared<MainController>,
    pub view_state: Shared<ViewState>,
}

// GSR_APPLICATION
impl GsrApplication {
    // stored for sharing: command line args, view state, navigator and gallery
    pub fn set_state(
        &self,
        clargs: CommandLineArguments,
        gallery: &Gallery,
        repository: &Repository,
    ) {
        // store clargs
        *self.command_line_arguments.borrow_mut() = clargs.clone();

        // store repository
        *self.repository.borrow_mut() = Some(repository.clone());
        let pictures_per_row = {
            // no grid or thumbnails option, try cfg
            if clargs.grid.is_none()
                && !clargs.thumbnails
                && let Some(pictures_per_row) = CONFIGURATION
                    .get()
                    .expect("Configuration not set")
                    .current_pictures_per_row
            {
                pictures_per_row as i32
            } else {
                clargs.pictures_per_row()
            }
        };
        let gallery = gallery.clone();
        let mut navigator = Navigator::new(gallery.len(), pictures_per_row as usize);
        let mut view_state = self.view_state.borrow_mut();
        view_state.settings.set_pictures_per_row(pictures_per_row);
        navigator.move_towards(&Direction::Index {
            value: gallery.current_picture_index(),
        });
        view_state.gallery = gallery.clone();
        view_state.navigator = navigator.clone();
        if let Some((row, col)) = navigator.coords_from_position(navigator.position()) {
            view_state.focus_at_coords = (col as i32, row as i32);
        }
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
