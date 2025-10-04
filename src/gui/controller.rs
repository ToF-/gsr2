use crate::Args;
use crate::cli::command::Command;
use crate::env::default_values::APPLICATION_ID;
use crate::env::environment::database_connection;
use crate::file::database::Database;
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::controller::gdk::Key;
use crate::gui::controller::gdk::ModifierType;
use crate::gui::direction::Direction;
use crate::gui::event::Event;
use crate::gui::navigator::Navigator;
use crate::gui::state::State;
use crate::gui::view::LEFT_PANE;
use crate::gui::view::View;
use crate::gui::view::components::application::make_application;
use crate::model::gallery::Gallery;
use crate::model::picture::Picture;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{self};
use std::cell::RefCell;
use std::io::Result as IOResult;
use std::rc::Rc;

#[derive(Debug)]
pub struct Controller {
    args: Args,
    gallery: Gallery,
    navigator: Navigator,
    controls: Controls,
    database: Database,
    state: State,
    view_opt: Option<View>,
}

pub type RcController = Rc<RefCell<Controller>>;

impl Controller {
    pub fn new(cli: Args) -> IOResult<Self> {
        let gallery = Gallery::new();
        let pictures_per_row = cli.pictures_per_row();
        database_connection().and_then(|connection_string| {
            match Database::from_connection(&connection_string) {
                Err(err) => Err(err),
                Ok(database) => Ok(Controller {
                    args: cli.clone(),
                    gallery,
                    navigator: Navigator::new(0, pictures_per_row as usize),
                    controls: default_controls(),
                    database,
                    state: State::new(pictures_per_row as usize, cli.slideshow().is_some()),
                    view_opt: None,
                }),
            }
        })
    }

    pub fn args(&self) -> Args {
        self.args.clone()
    }

    pub fn view(&self) -> View {
        let view = self.view_opt.unwrap();
        view.clone()
    }

    pub fn set_view(&mut self, view: View) {
        self.view_opt = Some(view)
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

    pub fn load_picture_data(&mut self) -> IOResult<usize> {
        let mut gallery = Gallery::new();
        let args = self.args.clone();
        let load_result = match args.command {
            Some(Command::File { file_path }) => gallery.load_from_file_path(&file_path),
            Some(Command::Dir { directory }) => gallery.load_from_directory(&directory),
            None => gallery.load_from_database(&self.database),
        };
        match load_result {
            Ok(_) => {
                let len = gallery.len();
                println!("{} pictures", len);
                Ok(len)
            }
            Err(err) => Err(err),
        }
    }

    pub fn create_view(&mut self, controller_rc: &RcController) {
        let pictures_per_row: i32 = self.navigator().pictures_per_row() as i32;
        let application = make_application(APPLICATION_ID, controller_rc);
        let view = View::new(&application, controller_rc);
        self.set_view(view)
    }

    pub fn run_application(self) {
        let application = self.view().application();
        let no_args: Vec<String> = vec![];
        application.run_with_args(&no_args);
    }

    pub fn process_event(&mut self, event: Event, controller_rc: &RcController) {
        match event {
            Event::KeyPressed {
                key,
                key_code,
                modifier_type,
            } => {
                self.process_key_event(key, key_code, modifier_type, controller_rc);
            }
            Event::NextSlideDelay => self.next_slide_delay(),
            Event::PaneClicked {
                button,
                pane_number,
            } => {
                self.process_pane_clicked(button, pane_number);
                self.set_slideshow_off()
            }
            Event::PictureClicked { button, col, row } if button == 1 => {
                self.process_picture_clicked(button, col, row);
                self.set_slideshow_off()
            }
            _ => println!("{:?}", event),
        }
    }

    pub fn set_slideshow_off(&mut self) {
        if self.state().slideshow_on() {
            println!("setting slideshow off…");
            self.state.set_slideshow_off();
        }
    }
    pub fn process_picture_clicked(&mut self, _button: u32, col: i32, row: i32) {
        let view = self.view();
        view.set_label_for_current_picture(self, false);
        if let Some(index) = self
            .navigator
            .position_from_coords(row as usize, col as usize)
        {
            if self.navigator.can_move(Direction::Index { value: index }) {
                self.navigator
                    .move_towards(Direction::Index { value: index });
            }
        }
        view.set_label_for_current_picture(self, true);
    }

    pub fn process_pane_clicked(&mut self, _button: usize, pane_number: usize) {
        self.process_control(if pane_number == LEFT_PANE {
            &Control::MovePrev
        } else {
            &Control::MoveNext
        });
        if self.navigator.has_moved() {
            View::set_pictures(self)
        }
    }

    pub fn process_key_event(
        &mut self,
        key: Key,
        _key_code: u32,
        _modifier_type: ModifierType,
        controller_rc: &RcController,
    ) {
        let view = self.view();
        view.set_label_for_current_picture(self, false);
        let old_slideshow_on = self.state().slideshow_on();
        self.process_key(key);
        if self.state.slideshow_on() != old_slideshow_on {
            if let Some(seconds) = self.args().slideshow() {
                View::attach_slideshow_event(seconds, controller_rc);
            }
        } else {
            self.set_slideshow_off();
            if self.state().dimension_changed() {
                self.view()
                    .change_dimension(controller_rc, self.state().pictures_per_row());
                self.acknowledge_dimension();
            }
            if self.state().single_view() != self.view().single_view() {
                view.toggle_view_stack(self);
                View::set_pictures(self)
            } else if self.navigator.page_changed() {
                View::set_pictures(self)
            };
            View::set_label_for_current_picture(self, true);
        }
    }

    pub fn process_key(&mut self, key: Key) {
        let controls = self.controls.clone();
        match key.name() {
            None => {}
            Some(key_name) => match controls.get(&key_name.to_string()) {
                Some(control) => self.process_control(control),
                _ => println!("?"),
            },
        }
    }

    pub fn next_slide_delay(&mut self) {
        self.move_next();
        View::set_pictures(self)
    }

    pub fn process_control(&mut self, control: &Control) {
        match control {
            Control::MoveNext => self.move_next(),
            Control::MovePrev => self.move_prev(),
            Control::MoveLast => self.move_last(),
            Control::MoveFirst => self.move_first(),
            Control::MoveStartPage => self.move_start(),
            Control::MoveEndPage => self.move_end(),
            Control::Left => self.arrow_move(Direction::Left),
            Control::Right => self.arrow_move(Direction::Right),
            Control::Up => self.arrow_move(Direction::Up),
            Control::Down => self.arrow_move(Direction::Down),
            Control::Quit => self.quit(),
            Control::ToggleSingleView => self.toggle_single_view(),
            Control::ToggleExpand => self.toggle_expand(),
            Control::ToggleFullSize => self.toggle_full_size(),
            Control::ToggleSlideShow => self.toggle_slideshow(),
            Control::Label => self.label(),
            Control::GridTwo => self.switch_grid(2),
            Control::GridThree => self.switch_grid(3),
            Control::GridFour => self.switch_grid(4),
            Control::GridFive => self.switch_grid(5),
            Control::GridTen => self.switch_grid(10),
            _ => {}
        }
    }

    pub fn label(&self) {
        // if let Ok(application_window) = self.view().application_window_rc().try_borrow_mut() {
        //     View::make_entry_window(&application_window, "Enter a label");
        // }
    }
    pub fn quit(&self) {
        let application_window = self.view().application_window();
        application_window.close()
    }

    pub fn toggle_single_view(&mut self) {
        self.state.toggle_single_view();
        if self.state.full_size_on() {
            self.state.toggle_full_size()
        }
        let navigator = &mut self.navigator;
        if self.state.single_view() {
            navigator.set_pictures_per_row(1);
        } else {
            navigator.set_pictures_per_row(self.state.pictures_per_row);
        }
        navigator.set_page_changed();
    }

    pub fn toggle_expand(&mut self) {
        if self.state.single_view() {
            self.state.toggle_expand();
            let navigator = &mut self.navigator;
            navigator.set_page_changed();
        }
    }

    pub fn toggle_full_size(&mut self) {
        if self.state.single_view() {
            self.state.toggle_full_size();
            let navigator = &mut self.navigator;
            navigator.set_page_changed();
        }
    }

    pub fn toggle_slideshow(&mut self) {
        if self.args().slideshow().is_some() {
            self.state.toggle_slideshow();
            if self.state.slideshow_on() {
                let navigator = &mut self.navigator;
                navigator.set_page_changed();
            }
        }
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

    pub fn arrow_move(&mut self, direction: Direction) {
        if self.state().single_view() && self.state().full_size_on() {
            self.full_size_arrow_move(direction)
        } else {
            let navigator = &mut (self.navigator);
            if navigator.can_move(direction.clone()) {
                navigator.move_towards(direction)
            }
        }
    }

    pub fn full_size_arrow_move(&self, direction: Direction) {
        self.view().full_size_arrow_move(direction.clone())
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
