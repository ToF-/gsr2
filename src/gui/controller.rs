use crate::gui::mode::EntryKind;
use crate::model::order::Order;
use crate::gui::mode::Mode;
use crate::Args;
use crate::MainWindow;
use crate::cli::command::Command;
use crate::env::environment::database_connection;
use crate::file::database::Database;
use crate::file::picture_file::create_missing_thumbnails;
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::direction::Direction;
use crate::gui::event::Event;
use crate::gui::navigator::Navigator;
use crate::gui::state::State;
use crate::gui::view::main_window::LEFT_PANE;
use crate::model::gallery::Gallery;
use crate::model::picture::Picture;
use gdk::{Key, ModifierType};
use gtk::prelude::*;
use gtk::{self, gdk};
use rand::Rng;
use rand::rng;
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
    main_window_opt: Option<MainWindow>,
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
                    main_window_opt: None,
                }),
            }
        })
    }

    pub fn args(&self) -> Args {
        self.args.clone()
    }

    pub fn main_window(&self) -> MainWindow {
        self.main_window_opt.clone().unwrap()
    }
    pub fn set_main_window(&mut self, main_window: MainWindow) {
        self.main_window_opt = Some(main_window)
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
        self.navigator = Navigator::new(self.gallery.len(), self.state().pictures_per_row());
        self.acknowledge_grid_size_change();
    }

    pub fn current_picture(&self) -> Picture {
        let navigator = &self.navigator;
        self.gallery.picture(navigator.position())
    }

    pub fn load_picture_data(&mut self) -> IOResult<usize> {
        let mut gallery = Gallery::new();
        let args = self.args.clone();
        let result = match args.command {
            Some(Command::File { file_path }) => gallery.load_from_file_path(&file_path),
            Some(Command::Dir { directory }) => gallery.load_from_directory(&directory),
            None => gallery.load_from_database(&self.database),
        };
        match result {
            Ok(0) => {
                println!("no pictures for this selection");
                Ok(0)
            }
            Ok(size) => {
                gallery.sort_by(args.order);
                println!("{} pictures", &gallery.len());
                if let Some(pictures_per_row) = self.args.create_missing_thumbnails {
                    create_missing_thumbnails(&gallery.clone(), pictures_per_row as usize);
                }
                self.set_gallery(gallery);
                self.navigator().set_page_changed();
                Ok(size)
            }
            Err(err) => Err(std::io::Error::other(err)),
        }
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
        self.main_window()
            .set_label_for_current_picture(self, false);
        if let Some(index) = self
            .navigator
            .position_from_coords(row as usize, col as usize)
            && self.navigator.can_move(Direction::Index { value: index })
        {
            self.navigator
                .move_towards(Direction::Index { value: index });
        }
        self.main_window().set_label_for_current_picture(self, true);
    }

    pub fn process_pane_clicked(&mut self, _button: usize, pane_number: usize) {
        self.process_control(if pane_number == LEFT_PANE {
            &Control::MovePrev
        } else {
            &Control::MoveNext
        });
        if self.navigator.has_moved() {
            self.main_window().set_pictures(self)
        }
    }

    pub fn process_key_event(
        &mut self,
        key: Key,
        _key_code: u32,
        _modifier_type: ModifierType,
        _controller_rc: &RcController,
    ) {
        let main_window = self.main_window();
        main_window.set_label_for_current_picture(self, false);
        let old_slideshow_on = self.state().slideshow_on();
        self.process_key(key);

        if self.state.slideshow_on() == old_slideshow_on {
            self.set_slideshow_off();
            if self.state().single_view() != self.main_window().single_view() {
                main_window.toggle_view_stack(self);
            };
            if self.navigator.page_changed() {
                self.main_window().set_pictures(self);
                self.navigator.set_page_unchanged();
            };
            self.main_window().set_label_for_current_picture(self, true);
            self.main_window().set_title(self);
        }
    }

    pub fn process_key(&mut self, key: Key) {
        let controls = self.controls.clone();
        match self.state().mode() {
            Mode::View => match key.name() {
                None => {}
                Some(key_name) => {
                    if let Some(control) = controls.get(&key_name.to_string()) {
                        self.process_control(control)
                    }
                }
            },
            Mode::Setting(setting) => { 
                match key.name() {
                    None => {},
                    Some(key_name) => {
                        let key_prefix: &str = match setting {
                            Control::SetDisplay => "D",
                            Control::SetOrder => "O",
                            _ => "no_control",
                        };
                        let key_sequence: String = key_prefix.to_owned() + &key_name.to_string();
                        if let Some(control) = controls.get(&key_sequence) {
                            self.set_setting(&setting, control);
                        }
                    },
                };
                self.state.set_mode(Mode::View)
            },
            Mode::Editing(kind) => {
                match key.name() {
                    None => {},
                    Some(key_name) => {
                        match controls.get(&key_name.to_string()) {
                                Some(Control::Cancel) => self.cancel(),
                                Some(Control::Enter) => self.enter(),
                                Some(Control::DeleteChar) => self.delete_editor_char(),
                                Some(_) |
                                None => self.process_editor_key(key, kind) 
                        }
                    },
                }
            }
        }
    }

    pub fn set_setting(&mut self, setting: &Control, choice: &Control) {
        match setting {
            Control::SetDisplay => match choice {
                    Control::DisplayDate |
                    Control::DisplaySize => self.process_control(choice),
                    _ => println!("?"),
                },
            Control::SetOrder => match choice {
                    Control::OrderByName |
                        Control::OrderByDate |
                        Control::OrderBySize |
                        Control::Randomize => self.process_control(choice),
                    _ => println!("?"),
            },
            _ => {},
        }
    }

    pub fn setting_display(&mut self) {
        println!("Setting display…");
        self.state.set_mode(Mode::Setting(Control::SetDisplay));
    }

    pub fn setting_order(&mut self) {
        self.state.set_mode(Mode::Setting(Control::SetOrder))
    }
    pub fn next_slide_delay(&mut self) {
        self.move_next();
        self.main_window().set_pictures(self)
    }

    pub fn process_control(&mut self, control: &Control) {
        match control {
            Control::Cancel => self.cancel(),
            Control::MoveNext => self.move_next(),
            Control::MovePrev => self.move_prev(),
            Control::MoveLast => self.move_last(),
            Control::MoveFirst => self.move_first(),
            Control::MoveStartPage => self.move_start(),
            Control::MoveRandom => self.move_random(),
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
            Control::Jump => self.jump(),
            Control::Label => self.label(),
            Control::GridTwo => self.change_grid_size(2),
            Control::GridThree => self.change_grid_size(3),
            Control::GridFour => self.change_grid_size(4),
            Control::GridFive => self.change_grid_size(5),
            Control::GridTen => self.change_grid_size(10),
            Control::SetDisplay => self.setting_display(),
            Control::SetOrder => self.setting_order(),
            Control::DisplayDate => self.toggle_display_date(),
            Control::DisplaySize => self.toggle_display_size(),
            Control::OrderByName => self.order_by(Order::Name),
            Control::OrderByDate => self.order_by(Order::Date),
            Control::OrderBySize => self.order_by(Order::Size),
            Control::Randomize => self.order_by(Order::Random),
            _ => {}
        }
    }

    pub fn delete_editor_char(&self) {
        self.main_window().entry_window().delete_char();
    }
    pub fn process_editor_key(&self, key: Key, kind: EntryKind) {
        if let Some(ch) = key.to_unicode() {
            let ch_is_ok = match kind {
                EntryKind::Number => ch.is_ascii_digit(),
                EntryKind::Label => matches!(ch,
                    'a'..='z' | '0'..='9' | '-' | '_'),
            };
            if ch_is_ok {
                self.main_window().entry_window().add_char(ch)
            }
        }
    }
    pub fn cancel(&mut self) {
        self.main_window().close_entry_window();
        self.state.set_mode(Mode::View)
    }

    pub fn enter(&mut self) {
        let content = self.main_window().entry_window().final_text();
        self.main_window().close_entry_window();
        if self.state.mode() == Mode::Editing(EntryKind::Number) {
            let index: usize = content.parse().unwrap();
            let direction = Direction::Index { value: index };
            if self.navigator().can_move(direction.clone()) {
                self.navigator.move_towards(direction)
            }
        };
        self.state.set_mode(Mode::View)
    }

    pub fn label(&mut self) {
        let mut main_window = self.main_window();
        main_window.popup_entry_window("Enter a label:", "");
        self.state.set_mode(Mode::Editing(EntryKind::Label));
        self.main_window_opt = Some(main_window);
    }

    pub fn jump(&mut self) {
        let mut main_window = self.main_window();
        main_window.popup_entry_window("Enter a number:", "");
        self.state.set_mode(Mode::Editing(EntryKind::Number));
        self.main_window_opt = Some(main_window);
    }

    pub fn quit(&self) {
        let application_window = self.main_window().application_window();
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
            navigator.set_pictures_per_row(self.state.pictures_per_row());
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

    pub fn toggle_display_date(&mut self) {
        self.state.toggle_display_date();
        self.main_window().set_title(self);
        println!("display date {}",
            if self.state().display_date_on() {
                String::from("on") 
            } else {
                String::from("off")
            });
    }

    pub fn toggle_display_size(&mut self) {
        self.state.toggle_display_size();
        self.main_window().set_title(self);
        println!("display size {}",
            if self.state().display_size_on() {
                String::from("on") 
            } else {
                String::from("off")
            });
    }

    pub fn toggle_full_size(&mut self) {
        if self.state.single_view() {
            self.state.toggle_full_size();
            let navigator = &mut self.navigator;
            navigator.set_page_changed();
        }
    }

    pub fn toggle_slideshow(&mut self) {
        if let Some(seconds) = self.args().slideshow() {
            self.state.toggle_slideshow();
            if self.state.slideshow_on() {
                self.main_window().reattach_slideshow_event(seconds);
                let navigator = &mut self.navigator;
                navigator.set_page_changed();
            }
        }
    }

    pub fn order_by(&mut self, order: Order) {
        let current_file_path = self.current_picture().file_path();
        self.gallery.sort_by(order);
        if let Some(index) = self.gallery().find_file_path(&current_file_path) {
            self.navigator.move_towards(Direction::Index{ value: index })
        } else {
            self.navigator.move_towards(Direction::First)
        };
        self.navigator.set_page_changed()
    }
    pub fn change_grid_size(&mut self, pictures_per_row: usize) {
        self.state.change_grid_size(pictures_per_row);
        self.main_window().change_grid_size(pictures_per_row);
        let navigator = &mut self.navigator;
        navigator.set_pictures_per_row(self.state.pictures_per_row());
        navigator.update_page_limits();
        navigator.set_page_changed();
        self.acknowledge_grid_size_change();
    }

    pub fn acknowledge_grid_size_change(&mut self) {
        self.state.acknowledge_grid_size_change();
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
        self.main_window().full_size_arrow_move(direction.clone())
    }

    pub fn move_next(&mut self) {
        let navigator = &mut self.navigator;
        if !self.state.full_size_on() {
            if self.state.single_view() {
                if navigator.can_move(Direction::Right) {
                    navigator.move_towards(Direction::Right);
                }
            } else if navigator.can_move_next_page() {
                navigator.move_next_page();
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
            } else if navigator.can_move_prev_page() {
                navigator.move_prev_page();
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

    pub fn move_random(&mut self) {
        let navigator = &mut self.navigator;
        let value: usize = rng().random_range(0..navigator.limit());
        if navigator.can_move(Direction::Index { value }) {
            navigator.move_towards(Direction::Index { value });
        }
    }
}
