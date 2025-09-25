use gtk::prelude::*;
use gtk::{self};
use gtk::{ Align, Application, ApplicationWindow, Text, gdk };
use crate::gallery::Gallery;
use crate::environment::database_connection;
use crate::control::default_controls;
use crate::navigator::Navigator;
use crate::control::Controls;
use crate::database::Database;
use crate::editor::Editor;
use crate::gui::state::State;
use crate::CommandLineInterface;
use crate::application_state::ApplicationState;
use crate::gui::view::View;
use gtk::glib::clone;
use crate::gui::components::make_application;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Result as IOResult;
use crate::default_values::{ DEFAULT_HEIGHT, DEFAULT_WIDTH };
use gtk::prelude::{GtkApplicationExt, GtkWindowExt};

pub struct Controller {
    args:  CommandLineInterface,
    gallery: Gallery,
    navigator: Navigator,
    controls: Controls,
    database: Database,
    editor: Editor,
    state: State,
    view:  View,
}

impl Controller {

    pub fn new(cli: CommandLineInterface) -> IOResult<Self> {
        let gallery = Gallery::new();
        let pictures_per_row = cli.pictures_per_row();
        let view = View::make_view(DEFAULT_HEIGHT, DEFAULT_WIDTH, pictures_per_row); 
        database_connection()
            .and_then(|connection_string| {
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
                        view
                    }),

        }
            })
    }

    pub fn build_and_run_app(&self) {
    }
}

// pub fn startup_gui(&self, _application: &gtk::Application) {
//     let css_provider = gtk::CssProvider::new();
//     css_provider.load_from_data(
//         "window { background-color:black;} image { margin:1em ; } label { color:white; }",
//     );
//     gtk::style_context_add_provider_for_display(
//         &gdk::Display::default().unwrap(),
//         &css_provider,
//         1000,
//     );
// }
// 
// pub fn build_and_run_application(&self, cli: CommandLineInterface) {
//     let application: gtk::Application = make_application("example.org.gsr2");
// 
//     application.connect_startup(|application| { self.startup_gui(application); });
//     let cli_rc = Rc::new(RefCell::new(cli));
//     // clone! passes a strong reference to a variable in the closure that activates the application
//     // move converts any variables captured by reference or mutable reference to variables captured by value.
//     application.connect_activate(
//         clone!(@strong cli_rc, => move |application: &gtk::Application| {
//         application.activate(&cli_rc); }),
//     );
//     let no_args: Vec<String> = vec![];
//     application.run_with_args(&no_args);
// }

