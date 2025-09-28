use crate::CommandLineInterface;
use crate::command::Command;
use crate::control::Control;
use crate::control::Controls;
use crate::control::default_controls;
use crate::database::Database;
use crate::default_values::{DEFAULT_HEIGHT, DEFAULT_WIDTH};
use crate::direction::Direction;
use crate::environment::database_connection;
use crate::file_system::create_missing_thumbnails;
use crate::gallery::Gallery;
use crate::gui::components::attach_cells;
use crate::gui::components::multiple_view_grid;
use crate::gui::components::remove_cells;
use crate::gui::components::single_view;
use crate::gui::components::toggle_view_stack;
use crate::gui::components::{make_application, startup_gui};
use crate::gui::controller::gdk::Key;
use crate::gui::controller::gdk::ModifierType;
use crate::gui::event::Event;
use crate::gui::event::Event::KeyPressed;
use crate::gui::event::Event::PaneClicked;
use crate::gui::event::Event::PictureClicked;
use crate::gui::navigator::Navigator;
use crate::gui::state::State;
use crate::gui::view::LEFT_PANE;
use crate::gui::view::View;
use crate::order::Order;
use crate::picture::Picture;
use gtk::gdk;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{self};
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
                Ok(database) => Ok(Controller {
                    args: cli,
                    gallery,
                    navigator: Navigator::new(0, pictures_per_row as usize),
                    controls: default_controls(),
                    database,
                    state: State::new(pictures_per_row as usize),
                    view,
                }),
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
        self.acknowledge_dimension();
    }

    pub fn current_picture(&self) -> Picture {
        let navigator = &self.navigator;
        self.gallery.picture(navigator.position())
    }

    pub fn build_and_run_app(controller: Controller) -> IOResult<()> {
        let controller_rc = Rc::new(RefCell::new(controller));
        match controller_rc.try_borrow_mut() {
            Ok(mut controller) => {
                let mut gallery = Gallery::new();
                let args = controller.args.clone();
                let _ = match args.command {
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
                if gallery.is_empty() {
                    return Ok(())
                }
                if controller.args.create_missing_thumbnails {
                    create_missing_thumbnails(&gallery.clone());
                }
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

    pub fn process_event(
        &mut self,
        event: Event,
        view: &View,
        application_window: &gtk::ApplicationWindow,
        controller_rc: &RcController,
    ) {
        match event {
            KeyPressed {
                key,
                key_code,
                modifier_type,
            } => self.process_key_event(
                key,
                key_code,
                modifier_type,
                view,
                application_window,
                controller_rc,
            ),
            PaneClicked {
                button,
                pane_number,
            } => self.process_pane_clicked(button, pane_number, view, application_window),
            PictureClicked { button, col, row } if button == 1 => {
                self.process_picture_cliked(button, col, row, view, application_window)
            }
            _ => println!("{:?}", event),
        }
    }

    pub fn process_picture_cliked(
        &mut self,
        _button: u32,
        col: i32,
        row: i32,
        view: &View,
        window: &gtk::ApplicationWindow,
    ) {
        view.set_label_for_current_picture(&window, self, false);
        if let Some(index) = self
            .navigator
            .position_from_coords(row as usize, col as usize)
        {
            if self.navigator.can_move(Direction::Index { value: index }) {
                self.navigator
                    .move_towards(Direction::Index { value: index });
            }
        }
        view.set_label_for_current_picture(&window, self, true);
    }

    pub fn process_pane_clicked(
        &mut self,
        _button: usize,
        pane_number: usize,
        view: &View,
        window: &gtk::ApplicationWindow,
    ) {
        self.process(if pane_number == LEFT_PANE {
            &Control::MovePrev
        } else {
            &Control::MoveNext
        });
        if self.navigator.has_moved() {
            view.set_pictures(window, self)
        }
    }

    pub fn process_key_event(
        &mut self,
        key: Key,
        _key_code: u32,
        _modifier_type: ModifierType,
        view: &View,
        window: &gtk::ApplicationWindow,
        controller_rc: &RcController,
    ) {
        view.set_label_for_current_picture(&window, self, false);
        self.process_key(key);
        if self.state().dimension_changed() {
            let grid = multiple_view_grid(&window);
            remove_cells(&grid, self.state().old_pictures_per_row() as i32);
            attach_cells(&grid, self.state().pictures_per_row() as i32);
            view.attach_grid_picture_events(
                self.state().pictures_per_row() as i32,
                window,
                controller_rc,
            );
            self.acknowledge_dimension();
        }
        if self.state().single_view() != single_view(&window) {
            toggle_view_stack(&window);
            view.set_pictures(&window, self)
        } else if self.navigator.page_changed() {
            view.set_pictures(&window, self)
        };
        view.set_label_for_current_picture(&window, self, true);
    }

    pub fn process_key(&mut self, key: Key) {
        let controls = self.controls.clone();
        match key.name() {
            None => {}
            Some(key_name) => match controls.get(&key_name.to_string()) {
                Some(control) => self.process(control),
                _ => {}
            },
        }
    }

    pub fn process(&mut self, control: &Control) {
        match control {
            Control::MoveNext => self.move_next(),
            Control::MovePrev => self.move_prev(),
            Control::MoveLast => self.move_last(),
            Control::MoveFirst => self.move_first(),
            Control::MoveStartPage => self.move_start(),
            Control::MoveEndPage => self.move_end(),
            Control::Left => self.move_left(),
            Control::Right => self.move_right(),
            Control::Up => self.move_up(),
            Control::Down => self.move_down(),
            Control::Quit => self.quit(),
            Control::ToggleSingleView => self.toggle_single_view(),
            Control::GridTwo => self.switch_grid(2),
            Control::GridThree => self.switch_grid(3),
            Control::GridFour => self.switch_grid(4),
            Control::GridFive => self.switch_grid(5),
            Control::GridTen => self.switch_grid(10),
            _ => {}
        }
    }
    pub fn quit(&self) {
        if let Ok(application_window) = self.view.application_window_rc().try_borrow_mut() {
            application_window.close()
        };
    }

    pub fn toggle_single_view(&mut self) {
        self.state.toggle_single_view();
        let navigator = &mut self.navigator;
        if self.state.single_view() {
            navigator.set_pictures_per_row(1);
        } else {
            navigator.set_pictures_per_row(self.state.pictures_per_row);
        }
        navigator.set_page_changed();
    }

    pub fn switch_grid(&mut self, pictures_per_row: usize) {
        self.state.switch_grid(pictures_per_row);
        let navigator = &mut self.navigator;
        navigator.set_pictures_per_row(self.state.pictures_per_row);
        navigator.update_page_limits();
        navigator.set_page_changed();
    }

    pub fn acknowledge_dimension(&mut self) {
        self.state.acknowledge_dimension();
    }

    pub fn move_start(&mut self) {
        let navigator = &mut self.navigator;
        if navigator.can_move(Direction::PageStart) {
            navigator.move_towards(Direction::PageStart);
        }
    }

    pub fn move_end(&mut self) {
        let navigator = &mut self.navigator;
        if navigator.can_move(Direction::PageEnd) {
            navigator.move_towards(Direction::PageEnd);
        }
    }
    pub fn move_right(&mut self) {
        let navigator = &mut self.navigator;
        if navigator.can_move(Direction::Right) {
            navigator.move_towards(Direction::Right);
        }
    }

    pub fn move_left(&mut self) {
        let navigator = &mut self.navigator;
        if navigator.can_move(Direction::Left) {
            navigator.move_towards(Direction::Left);
        }
    }

    pub fn move_up(&mut self) {
        let navigator = &mut self.navigator;
        if navigator.can_move(Direction::Up) {
            navigator.move_towards(Direction::Up);
        }
    }

    pub fn move_down(&mut self) {
        let navigator = &mut self.navigator;
        if navigator.can_move(Direction::Down) {
            navigator.move_towards(Direction::Down);
        }
    }

    pub fn move_next(&mut self) {
        let navigator = &mut self.navigator;
        if !self.state.full_size_on() {
            if self.state.single_view() {
                if navigator.can_move(Direction::Right) {
                    navigator.move_towards(Direction::Right);
                }
            } else {
                if navigator.can_move_next_page() {
                    navigator.move_next_page();
                }
            }
        }
    }

    pub fn move_prev(&mut self) {
        let navigator = &mut self.navigator;
        if !self.state.full_size_on() {
            if self.state.single_view() {
                if navigator.can_move(Direction::Left) {
                    navigator.move_towards(Direction::Left);
                }
            } else {
                if navigator.can_move_prev_page() {
                    navigator.move_prev_page();
                }
            }
        }
    }

    pub fn move_first(&mut self) {
        let navigator = &mut self.navigator;
        if !self.state.full_size_on() {
            navigator.move_towards(Direction::First);
        }
    }

    pub fn move_last(&mut self) {
        let navigator = &mut self.navigator;
        if !self.state.full_size_on() {
            navigator.move_towards(Direction::Last);
        }
    }
}
