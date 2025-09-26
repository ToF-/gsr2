use crate::CommandLineInterface;
use crate::application_state::ApplicationState;
use crate::command::Command;
use crate::control::Control;
use crate::control::Controls;
use crate::control::default_controls;
use crate::database::Database;
use crate::default_values::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::editor::Editor;
use crate::environment::database_connection;
use crate::gallery::Gallery;
use crate::gui::components::{make_application, startup_gui};
use crate::gui::controller::gdk::Key;
use crate::gui::state::State;
use crate::gui::view::View;
use crate::navigator::Navigator;
use crate::order::Order;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{self};
use gtk::{Align, Application, ApplicationWindow, Text, gdk};
use gtk::{CssProvider, Grid, Label, Orientation, Picture, ScrolledWindow};
use std::cell::RefCell;
use std::io::Result as IOResult;
use std::rc::Rc;

#[derive(Debug)]
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
        let view = View::new(DEFAULT_HEIGHT, DEFAULT_WIDTH, pictures_per_row);
        database_connection().and_then(|connection_string| {
            match Database::from_connection(&connection_string) {
                Err(err) => Err(err),
                Ok(database) => {
                    println!("we have a database connection");
                    Ok(Controller {
                        args: cli,
                        gallery,
                        navigator: Navigator::new(0, pictures_per_row as usize),
                        controls: default_controls(),
                        database,
                        editor: Editor::new(),
                        state: State::new(pictures_per_row as usize),
                        view,
                    })
                }
            }
        })
    }

    pub fn view(&self) -> View {
        self.view.clone()
    }

    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn navigator(&self) -> Navigator {
        self.navigator.clone()
    }

    pub fn gallery(&self) -> &Gallery {
        &self.gallery
    }

    pub fn set_gallery(&mut self, gallery: Gallery) {
        self.gallery = gallery;
        self.navigator = Navigator::new(self.gallery.len(), self.state().pictures_per_row);
    }

    fn bind_components(controller_rc: &RcController) {}

    pub fn build_and_run_app(controller: Controller) -> IOResult<()> {
        println!("we have a controller");
        let controller_rc = Rc::new(RefCell::new(controller));
        match controller_rc.try_borrow_mut() {
            Ok(mut controller) => {
                let mut gallery = Gallery::new();
                let args = controller.args.clone();
                let result = match args.command {
                    Some(Command::File { file_path }) => gallery.load_from_file_path(&file_path),
                    Some(Command::Dir { directory }) => gallery.load_from_directory(&directory),
                    None => gallery.load_from_database(&controller.database),
                };
                if args.random {
                    gallery.sort_by(Order::Random)
                } else {
                    gallery.sort_by(Order::Name)
                };
                println!("{} pictures", &gallery.len());
                controller.set_gallery(gallery);
            }
            Err(err) => return Err(std::io::Error::other(err)),
        };
        let application: gtk::Application = make_application("org.example.gallsh");
        application.connect_startup(|application| startup_gui(application));
        application.connect_activate(
            clone!(@strong controller_rc => move |application: &gtk::Application| {
                View::build_gui(&application, &controller_rc)
            }),
        );
        let no_args: Vec<String> = vec![];
        application.run_with_args(&no_args);
        Ok(())
    }

    pub fn process_key(&mut self, key: Key) {
        match key.name() {
            None => {}
            Some(key_name) => match self.controls.get(&key_name.to_string()) {
                Some(control) => self.process(control),
                _ => {}
            },
        }
    }

    pub fn process(&self, control: &Control) {
        match control {
            Control::Quit => self.quit(),
            _ => {}
        }
    }
    pub fn quit(&self) {
        if let Ok(application_window) = self.view.application_window_rc().try_borrow_mut() {
            application_window.close()
        }
    }
}
