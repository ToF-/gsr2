use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::Configuration;
use crate::env::default_values::APPLICATION_NAME;
use crate::env::default_values::FRAME_WINDOW_NAME;
use crate::env::default_values::GRID_WINDOW_NAME;
use crate::gui::controller::Controller;
use crate::gui::direction::Direction;
use crate::gui::main_controller::MainController;
use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_picture_frame::GsrPictureFrame;
use crate::gui::objects::gsr_picture_grid::GsrPictureGrid;
use crate::gui::view::View;
use crate::gui::view::picture_frame::PictureFrame;
use crate::gui::view::treelist_view::TreeListView;
use crate::model::catalog::Catalog;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

use gtk::subclass::prelude::*;
use std::cell::RefCell;

pub const LEFT_PANE: usize = 0;
pub const RIGHT_PANE: usize = 1;

use super::*;

#[derive(Default)]
pub struct GsrApplicationWindow {
    pub command_line_arguments: RefCell<Option<Shared<CommandLineArguments>>>,
    pub configuration: RefCell<Option<Shared<Configuration>>>,
    pub main_controller: RefCell<Option<Shared<MainController>>>,
    pub view: RefCell<Option<Rc<RefCell<View>>>>,
    pub navigator: RefCell<Option<Shared<Navigator>>>,
    pub gallery: RefCell<Option<Shared<Gallery>>>,
}

impl GsrApplicationWindow {
    pub fn set_state(
        &self,
        command_line_arguments: Shared<CommandLineArguments>,
        configuration: Shared<Configuration>,
        main_controller: Shared<MainController>,
        view: Shared<View>,
        navigator: Shared<Navigator>,
        gallery: Shared<Gallery>,
    ) {
        *self.command_line_arguments.borrow_mut() = Some(command_line_arguments);
        *self.configuration.borrow_mut() = Some(configuration);
        *self.main_controller.borrow_mut() = Some(main_controller);
        *self.view.borrow_mut() = Some(view);
        *self.navigator.borrow_mut() = Some(navigator);
        *self.gallery.borrow_mut() = Some(gallery);
    }
    pub fn shared_view(&self) -> Rc<RefCell<View>> {
        self.view.borrow().as_ref().unwrap().clone()
    }

    pub fn shared_gallery(&self) -> Rc<RefCell<Gallery>> {
        self.gallery.borrow().as_ref().unwrap().clone()
    }
}
#[gtk::glib::object_subclass]
impl ObjectSubclass for GsrApplicationWindow {
    const NAME: &'static str = "GsrApplicationWindow";
    type Type = super::GsrApplicationWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for GsrApplicationWindow {}

impl WidgetImpl for GsrApplicationWindow {}

impl WindowImpl for GsrApplicationWindow {}

impl ApplicationWindowImpl for GsrApplicationWindow {}
