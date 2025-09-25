use crate::CommandLineInterface;
use crate::application_state::ApplicationState;
use crate::control::Controls;
use crate::control::default_controls;
use crate::database::Database;
use crate::default_values::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::editor::Editor;
use crate::environment::database_connection;
use crate::gallery::Gallery;
use crate::gui::components::make_application;
use crate::gui::state::State;
use crate::gui::view::View;
use crate::navigator::Navigator;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Text, gdk};
use gtk::{self};
use std::cell::RefCell;
use std::io::Result as IOResult;
use std::rc::Rc;


pub struct Controller {
    args: CommandLineInterface,
    gallery: Gallery,
    navigator: Navigator,
    controls: Controls,
    database: Database,
    editor: Editor,
    state: State,
    view: View,
}

pub type RcController = Rc<RefCell<Controller>>;

impl Controller {
    pub fn new(cli: CommandLineInterface) -> IOResult<Self> {
        let gallery = Gallery::new();
        let pictures_per_row = cli.pictures_per_row();
        let view = View::make_view(DEFAULT_HEIGHT, DEFAULT_WIDTH, pictures_per_row);
        database_connection().and_then(|connection_string| {
            match Database::from_connection(&connection_string) {
                Err(err) => Err(err),
                Ok(database) => Ok(Controller {
                    args: cli,
                    gallery,
                    navigator: Navigator::new(0, pictures_per_row as usize),
                    controls: default_controls(),
                    database,
                    editor: Editor::new(),
                    state: State::new(pictures_per_row as usize),
                    view,
                }),
            }
        })
    }

    fn startup_gui() {
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data("window { background-color:black;} image { margin:1em ; } label { color:white; }");
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &css_provider,
            1000,
        );

    }

    fn bind_components(controller_rc: &RcController) {
    }

    pub fn build_and_run_app(controller: Controller) {
        let controller_rc = Rc::new(RefCell::new(controller));
        {
            if let Ok(controller) = controller_rc.try_borrow() {
                let application = controller.view.application.clone();
                application.connect_startup(|application|
                    { Self::startup_gui() });
                application.connect_activate(
                    clone!(@strong controller_rc, => move | application: &gtk::Application| {
                        Self::bind_components(&controller_rc)
                    }));
                let no_args: Vec<String> = vec![];
                application.run_with_args(&no_args);
            }
        }
    }
}
