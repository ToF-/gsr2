use crate::cli::args::Args;
use crate::cli::command::Command;
use crate::cli::status::Status;
use crate::env::configuration::{Configuration, get_configuration};
use crate::env::environment::database_connection;
use crate::file::database::*;
use crate::file::{delete_picture, move_pictures};
use crate::file::operation::{execute, move_picture};
use crate::file::paths::{check_collectable, file_exists, parent_directory};
use crate::file::picture_file::{collect_data, create_missing_thumbnails};
use crate::gui::control::{Control, Controls, default_controls};
use crate::gui::direction::Direction;
use crate::gui::editor::Editor;
use crate::gui::entry_kind::EntryKind;
use crate::gui::event::Event;
use crate::gui::mode::Mode;
use crate::gui::navigator::Navigator;
use crate::gui::state::State;
use crate::gui::view::main_window::{LEFT_PANE, MainWindow};
use crate::model::action::Action;
use crate::model::gallery::Gallery;
use crate::model::order::Order;
use crate::model::picture::Picture;
use crate::model::rank::Rank;
use crate::model::repository::Repository;
use crate::model::selection::{Selection, ALL_TAGS, SOME_TAGS};
use gdk::{Key, ModifierType};
use gtk::prelude::*;
use gtk::{self, gdk};
use rand::Rng;
use rand::rng;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Error as IOError;
use std::io::Result as IOResult;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;

#[derive(Debug)]
pub struct Controller {
    repository: Repository,
    args: Args,
    gallery: Gallery,
    navigator: Navigator,
    controls: Controls,
    database: Database,
    state: State,
    main_window_opt: Option<MainWindow>,
    editor: Editor,
    last_action: Action,
    parent_dirs: HashMap<String, usize>,
}

pub type RcController = Rc<RefCell<Controller>>;

impl Controller {
    pub fn new(config: Configuration, cli: Args) -> IOResult<Self> {
        let gallery = Gallery::new();
        let pictures_per_row = cli.pictures_per_row();
        database_connection(config.clone()).and_then(|connection_string| {
            match Database::from_connection(&connection_string, false) {
                Err(err) => Err(err),
                Ok(mut database) => {
                    let mut repository = Repository::new(config, cli.clone());
                    repository.initialize();
                    database.retrieve_all_parent_dirs().and_then(|parent_dirs| {
                        database.retrieve_all_labels().and_then(|labels| {
                            Ok(Controller {
                                repository,
                                args: cli.clone(),
                                gallery,
                                editor: Editor::new(),
                                navigator: Navigator::new(0, pictures_per_row as usize),
                                controls: default_controls(),
                                database,
                                state: State::new(pictures_per_row as usize, cli.slideshow().is_some()),
                                main_window_opt: None,
                                last_action: Action::NoAction,
                                parent_dirs,
                            })
                        })
                    })
                },
            }
        })
    }

    pub fn args(&self) -> Args {
        self.args.clone()
    }

    pub fn database(&self) -> Database {
        self.database.clone()
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

    pub fn labels(&self) -> HashSet<String> {
        self.repository.all_labels()
    }

    pub fn current_picture(&self) -> Picture {
        let navigator = &self.navigator;
        self.gallery.picture(navigator.position())
    }

    fn load_gallery(&mut self) -> IOResult<usize> {
        let mut gallery = Gallery::new();
        let args = self.args.clone();
        let result = match args.command {
            Some(Command::File { file_path }) => match self.repository.picture_from_file_path(&file_path) {
                Ok(gallery) => {
                    println!("{} pictures", gallery.len());
                    self.set_gallery(gallery.clone());
                    Ok(gallery.len())
                },
                Err(e) => Err(e),
            },
            Some(Command::Directory { directory }) => match self.repository.pictures_in_directory(&directory) {
                Ok(gallery) => {
                    println!("{} pictures", gallery.len());
                    self.set_gallery(gallery.clone());
                    Ok(gallery.len())
                },
                Err(e) => Err(e),
            },
            None => match self.repository.gallery_rc().try_borrow() {
                Ok(gallery) => {
                    println!("{} pictures", gallery.len());
                    self.set_gallery(gallery.clone());
                    Ok(gallery.len())
                },
                Err(e) => Err(IOError::other(format!("{}",e))),
            },
            _ => Ok(0),
        };
        match result {
            Ok(0) => {
                println!("no pictures\nquitting");
                self.quit();
                Ok(0)
            },
            other => other
        }
    }

    pub fn execute_command(&mut self) -> IOResult<Status> {
        let mut gallery = Gallery::new();
        let args = self.args.clone();
        match args.command {
            Some(Command::File { file_path }) => match gallery.load_from_file_path(&file_path) {
                Err(e) => Err(e),
                Ok(_) => Ok(Status::Ready),
            },
            Some(Command::Directory { directory }) => match gallery.load_from_directory(&directory)
            {
                Err(e) => Err(e),
                Ok(0) => {
                    println!("no pictures for this selection");
                    Ok(Status::Exit)
                }
                Ok(n) => {
                    println!("{} pictures", &gallery.len());
                    gallery.sort_by(args.order);
                    self.set_gallery(gallery);
                    if let Some(index) = args.index
                        && self.navigator().can_move(Direction::Index { value: index })
                    {
                        self.navigator
                            .move_towards(Direction::Index { value: index })
                    };
                    self.navigator().set_page_changed();
                    Ok(Status::Ready)
                }
            },
            Some(Command::Collect { directory }) => {
                println!("collecting data for picture files in the database…");
                let path: PathBuf = PathBuf::from(directory);
                match check_collectable(&path) {
                    Ok(directory) => {
                        gallery.load_from_directory(&directory.display().to_string());
                        match collect_data(&gallery, &self.database()) {
                            Ok(_) => Ok(Status::Done),
                            Err(err) => Err(err),
                        }
                    }
                    Err(err) => Err(err),
                }
            }
            Some(Command::Thumbnails { pictures_per_row }) => {
                gallery.load_from_database(&self.database, &args);
                create_missing_thumbnails(&gallery, pictures_per_row as usize);
                Ok(Status::Done)
            }
            Some(Command::List { ref directory }) => {
                match directory {
                    Some(path) => {
                        gallery.load_from_directory(&path);
                    }
                    None => {
                        gallery.load_from_database(&self.database, &args);
                    }
                };
                gallery.print();
                if !self.parent_dirs.is_empty() {
                    println!("----- directories:{} -----", self.parent_dirs.len());
                    let mut dirs: Vec<String> = vec![];
                    for dir in self.parent_dirs.keys() {
                        dirs.push(dir.to_string());
                    }
                    dirs.sort();
                    for dir in dirs {
                        let count = self.parent_dirs.get(&dir).unwrap();
                        println!("{}:  {}", dir, count)
                    }
                }
                Ok(Status::Done)
            }
            Some(Command::Check) => {
                gallery.load_from_database(&self.database, &args);
                for picture in gallery.pictures() {
                    if !file_exists(&picture.file_path()) {
                        println!("{}", picture.file_path());
                    }
                }
                Ok(Status::Done)
            }
            Some(Command::Clean) => {
                gallery.load_from_database(&self.database, &args);
                for picture in gallery.pictures() {
                    if !file_exists(&picture.file_path()) {
                        self.database
                            .delete_picture_with_file_path(&picture.file_path());
                        println!("deleted from database: {}", picture.file_path());
                    }
                }
                Ok(Status::Done)
            }
            Some(Command::Move { source, target }) => {
                let selection: Selection = if let Some(labels) = &args.select {
                    Selection::from(&labels, SOME_TAGS)
                } else if let Some(labels) = &args.restrict {
                    Selection::from(&labels, ALL_TAGS)
                } else {
                    Selection::empty()
                };
                match move_pictures(&self.database, &selection, &source, &target) {
                    Ok(n) => {
                        println!("{} pictures moved from {} to {}", n, source, target);
                        Ok(Status::Exit)
                    }
                    Err(err) => Err(err),
                }
            }
            Some(Command::Initialize) => {
                let config = match get_configuration() {
                    Ok(config) => config,
                    Err(e) => return Err(e),
                };
                println!("initializing database");
                if !file_exists(&config.database_file) {
                    println!("creating new database file {}", config.database_file);
                    match Database::from_connection(&config.database_file, true) {
                        Ok(database) => match database.rusqlite_create_schema() {
                            Ok(_) => Ok(Status::Done),
                            Err(e) => Err(IOError::other(e)),
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    Err(IOError::other(format!(
                        "{} already exists",
                        &config.database_file
                    )))
                }
            }

            None => match gallery.load_from_database(&self.database, &args) {
                Err(e) => Err(e),
                Ok(0) => {
                    println!("no pictures for this selection");
                    Ok(Status::Exit)
                }
                Ok(n) => {
                    println!("{} pictures", &gallery.len());
                    gallery.sort_by(args.order);
                    self.set_gallery(gallery);
                    if let Some(index) = args.index
                        && self.navigator().can_move(Direction::Index { value: index })
                    {
                        self.navigator
                            .move_towards(Direction::Index { value: index })
                    };
                    self.navigator().set_page_changed();
                    Ok(Status::Ready)
                }
            },
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
            .set_label_text_for_current_picture(self, None);
        if let Some(index) = self
            .navigator
            .position_from_coords(row as usize, col as usize)
            && self.navigator.can_move(Direction::Index { value: index })
        {
            self.navigator
                .move_towards(Direction::Index { value: index });
        }
        self.set_label_text_for_current_picture()
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
        main_window.set_label_text_for_current_picture(self, None);
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
            self.set_label_text_for_current_picture();
            self.main_window().set_title(self);
        }
    }

    pub fn set_label_text_for_current_picture(&mut self) {
        if self.state.change_focus_symbol_on() {
            self.state.toggle_focus_symbol()
        };
        self.main_window()
            .set_label_text_for_current_picture(self, Some(self.state().focus_symbol()))
    }

    pub fn set_opacity_for_current_picture(&mut self, opacity: f64) {
        self.main_window()
            .set_opacity_for_current_picture(&self, opacity)
    }

    pub fn process_key(&mut self, key: Key) {
        let controls = self.controls.clone();
        match self.state().mode() {
            Mode::View => match key.name() {
                None => {}
                Some(key_name) => {
                    if let Some(control) = controls.get(&(key_name.to_string(), Mode::View)) {
                        self.process_control(control)
                    }
                }
            },
            Mode::Setting(setting) => {
                match key.name() {
                    None => {}
                    Some(key_name) => {
                        if let Some(control) =
                            controls.get(&(key_name.to_string(), Mode::Setting(setting)))
                        {
                            self.set_setting(&setting, control);
                        }
                    }
                };
                self.state.set_mode(Mode::View)
            }
            Mode::Editing => {
                self.editor.process(key);
                if !self.editor.editing() {
                    self.state.set_mode(Mode::View);
                    match self.editor.entry_kind() {
                        EntryKind::Label => {
                            if !self.editor.input().is_empty() {
                                self.label_selected_pictures(&self.editor.input())
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::AddTag => {
                            if !self.editor.input().is_empty() {
                                self.tag_selected_pictures(&self.editor.input())
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::RemoveTag => {
                            if !self.editor.input().is_empty() {
                                self.untag_selected_pictures(&self.editor.input())
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::Number => {
                            self.move_towards_index(self.editor.input().parse().unwrap())
                        }
                        EntryKind::Order => self.set_order(&self.editor.input()),
                        EntryKind::DeleteConfirmation => {
                            if &self.editor.input() == "yes" {
                                self.confirm_delete_picture()
                            } else {
                                self.cancel_delete_picture()
                            }
                        }
                        EntryKind::MoveConfirmation => {
                            if &self.editor.input() == "yes" {
                                self.confirm_move_picture()
                            } else {
                                self.cancel_move_picture()
                            }
                        }
                        EntryKind::Find => {
                            if !self.editor.input().is_empty() {
                                self.find_pattern(&self.editor.input())
                            };
                        }
                        EntryKind::FindLabel => {
                            if !self.editor.input().is_empty() {
                                self.find_pattern_in_label(&self.editor.input())
                            };
                        }
                        EntryKind::Information => {}
                        EntryKind::SetSelection => {
                            if !self.editor.input().is_empty() {
                                self.apply_selection(&self.editor.input())
                            }
                        }
                        EntryKind::SetRestriction => {
                            if !self.editor.input().is_empty() {
                                self.apply_restriction(&self.editor.input())
                            }
                        }
                    }
                }
            }
        }
    }

    fn set_order(&mut self, input: &str) {
        let choice: Control = match input {
            "ColorCount" => Control::OrderByColorCount,
            "Date" => Control::OrderByDate,
            "Label" => Control::OrderByLabel,
            "Name" => Control::OrderByName,
            "Palette" => Control::OrderByPalette,
            "Random" => Control::Randomize,
            "Size" => Control::OrderBySize,
            "Value" => Control::OrderByValue,
            &_ => todo!(),
        };
        self.process_control(&choice)
    }

    fn label_picture_at_index(&mut self, index: usize, label: &str) {
        let mut picture = self.gallery.picture(index);
        picture.set_label(label);
        self.gallery.set_picture(index, picture.clone());
        if self.args.on_database() {
            match self.database.update_picture(&picture) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        };
        self.last_action = Action::Label(label.to_string());
    }

    pub fn label_selected_pictures(&mut self, label: &str) {
        self.repository.add_label(label);
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.label_picture_at_index(index, label);
                }
            }
            self.navigator.unselect_all();
        } else {
            self.label_picture_at_index(self.navigator().position(), label)
        };
        self.navigator.set_page_changed()
    }

    pub fn unlabel_selected_pictures(&mut self) {
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.label_picture_at_index(index, "");
                }
            }
            self.navigator.unselect_all();
        } else {
            self.label_picture_at_index(self.navigator().position(), "")
        };
        self.navigator.set_page_changed();
        self.last_action = Action::Unlabel;
    }

    fn tag_picture_at_index(&mut self, index: usize, label: &str) {
        let mut picture = self.gallery.picture(index);
        picture.add_tag(label);
        self.gallery.set_picture(index, picture.clone());
        if self.args.on_database() {
            match self.database.update_picture(&picture) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }

    fn untag_picture_at_index(&mut self, index: usize, label: &str) {
        let mut picture = self.gallery.picture(index);
        picture.remove_tag(label);
        self.gallery.set_picture(index, picture.clone());
        if self.args.on_database() {
            match self.database.update_picture(&picture) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }

    pub fn tag_selected_pictures(&mut self, label: &str) {
        self.repository.add_label(label);
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.tag_picture_at_index(index, label);
                }
            }
            self.navigator.unselect_all();
        } else {
            self.tag_picture_at_index(self.navigator().position(), label)
        };
        self.navigator.set_page_changed();
        self.last_action = Action::AddTag(label.to_string());
    }

    pub fn untag_selected_pictures(&mut self, label: &str) {
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.untag_picture_at_index(index, label);
                }
            }
            self.navigator.unselect_all();
        } else {
            self.untag_picture_at_index(self.navigator().position(), label)
        };
        self.navigator.set_page_changed();
        self.last_action = Action::RemoveTag(label.to_string());
    }

    pub fn move_towards_index(&mut self, index: usize) {
        let direction = Direction::Index { value: index };
        if self.navigator().can_move(direction.clone()) {
            self.navigator.move_towards(direction)
        }
    }

    pub fn set_setting(&mut self, setting: &Control, choice: &Control) {
        match setting {
            Control::SetDisplay => match choice {
                Control::DisplayDate | Control::DisplaySize => self.process_control(choice),
                Control::DisplayFocus => self.toggle_display_focus_symbol_change(),
                _ => println!("?"),
            },
            Control::SetOrder => match choice {
                Control::OrderByName
                | Control::OrderByDate
                | Control::OrderBySize
                | Control::OrderByValue
                | Control::OrderByLabel
                | Control::OrderByColorCount
                | Control::OrderByPalette
                | Control::Randomize => self.process_control(choice),
                _ => println!("?"),
            },
            _ => {}
        }
    }

    pub fn setting_display(&mut self) {
        println!("Setting display…");
        self.state.set_mode(Mode::Setting(Control::SetDisplay));
    }

    pub fn setting_order(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::Order, None);
        self.state.set_mode(Mode::Editing);
    }

    pub fn next_slide_delay(&mut self) {
        self.move_next();
        self.main_window().set_pictures(self)
    }

    pub fn process_control(&mut self, control: &Control) {
        match control {
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
            Control::ToggleCover => self.toggle_cover(),
            Control::ToggleExpand => self.toggle_expand(),
            Control::ToggleFullSize => self.toggle_full_size(),
            Control::ToggleSlideShow => self.toggle_slideshow(),
            Control::TogglePalette => self.toggle_palette(),
            Control::Jump => self.jump(),
            Control::Find => self.find(),
            Control::FindLabel => self.find_label(),
            Control::Information => self.information(),
            Control::ToggleInformation => self.toggle_information(),
            Control::AddTag => self.add_tag(),
            Control::RemoveTag => self.remove_tag(),
            Control::Label => self.label(),
            Control::Unlabel => self.unlabel_selected_pictures(),
            Control::GridTwo => self.change_grid_size(2),
            Control::GridThree => self.change_grid_size(3),
            Control::GridFour => self.change_grid_size(4),
            Control::GridFive => self.change_grid_size(5),
            Control::GridTen => self.change_grid_size(10),
            Control::SetDisplay => self.setting_display(),
            Control::SetOrder => self.setting_order(),
            Control::SetSelection => self.set_selection(),
            Control::SetRestriction => self.set_restriction(),
            Control::CancelSelection => self.cancel_selection(),
            Control::DisplayDate => self.toggle_display_date(),
            Control::DisplaySize => self.toggle_display_size(),
            Control::OrderByName => self.order_by(Order::Name),
            Control::OrderByDate => self.order_by(Order::Date),
            Control::OrderBySize => self.order_by(Order::Size),
            Control::OrderByValue => self.order_by(Order::Value),
            Control::OrderByLabel => self.order_by(Order::Label),
            Control::OrderByColorCount => self.order_by(Order::ColorCount),
            Control::OrderByPalette => self.order_by(Order::Palette),
            Control::Randomize => self.order_by(Order::Random),
            Control::SetRange => self.set_range(),
            Control::SetRangeAll => self.set_range_all(),
            Control::RepeatRange => self.repeat_range(),
            Control::ToggleSelected => self.toggle_selected(),
            Control::CancelRange => self.cancel_range(),
            Control::DeletePicture => self.delete_picture(),
            Control::MovePicture => self.move_picture(),
            Control::RankNoStar => self.rank_selected_pictures(Rank::NoStar),
            Control::RankOneStar => self.rank_selected_pictures(Rank::OneStar),
            Control::RankTwoStars => self.rank_selected_pictures(Rank::TwoStars),
            Control::RankThreeStars => self.rank_selected_pictures(Rank::ThreeStars),
            Control::RepeatLastAction => self.repeat_last_action(),
            Control::GotoDirectory => self.go_to_directory(),
            Control::BackFromDirectory => self.back_from_directory(),
            _ => {}
        }
    }

    pub fn go_to_directory(&mut self) {
        if let Some(directory) = parent_directory(&self.current_picture().file_path())
            && Some(directory.clone()) != self.args.directory
            && !self.state.single_view()
        {
            self.args.index = Some(self.navigator.position());
            let args = self.args.clone();
            self.state.push_current_args(args.clone());
            let new_args = Args {
                directory: Some(directory),
                cover: false,
                ..args.clone()
            };
            self.args = new_args;
            self.reload();
            self.navigator.set_page_changed()
        }
    }

    pub fn back_from_directory(&mut self) {
        if let Some((pictures_per_row, single_view, old_args)) = self.state.pop_saved_args() {
            self.args = old_args;
            self.state.set_single_view(single_view);
            self.change_grid_size(pictures_per_row);
            self.reload();
            if let Some(index) = self.args.index
                && self.navigator.can_move(Direction::Index { value: index })
            {
                self.navigator
                    .move_towards(Direction::Index { value: index })
            };
            self.navigator.set_page_changed()
        }
    }

    pub fn current_dir_count(&self) -> usize {
        if let Some(directory) = parent_directory(&self.current_picture().file_path()) {
            if let Some(count) = self.parent_dirs.get(&directory) {
                *count
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn toggle_cover(&mut self) {
        let index = self.navigator().position();
        let mut picture = self.gallery.picture(index);
        let dir_count = self.current_dir_count();
        picture.toggle_cover(dir_count);
        self.gallery.set_picture(index, picture.clone());
        match self.database.update_picture(&picture) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
            }
        }
        self.navigator.set_page_changed()
    }
    pub fn set_selection(&mut self) {
        self.editor.begin(
            &self.main_window(),
            EntryKind::SetSelection,
            Some(self.repository.all_labels()),
        );
        self.state.set_mode(Mode::Editing);
    }

    pub fn set_restriction(&mut self) {
        self.editor.begin(
            &self.main_window(),
            EntryKind::SetRestriction,
            Some(self.repository.all_labels()),
        );
        self.state.set_mode(Mode::Editing);
    }

    pub fn cancel_selection(&mut self) {
        let current_file_path = self.current_picture().file_path();
        self.gallery.set_selection(Selection::empty());
        if let Some(index) = self.gallery().find_file_path(&current_file_path) {
            self.navigator
                .move_towards(Direction::Index { value: index })
        } else {
            self.navigator.move_towards(Direction::First)
        };
        self.navigator.set_page_changed();
    }

    pub fn apply_selection(&mut self, selection_str: &str) {
        self.gallery
            .set_selection(Selection::from(selection_str, false));
        self.navigator.move_towards(Direction::First);
        self.navigator.set_page_changed();
    }

    pub fn apply_restriction(&mut self, selection_str: &str) {
        self.gallery
            .set_selection(Selection::from(selection_str, true));
        self.navigator.move_towards(Direction::First);
        self.navigator.set_page_changed();
    }

    pub fn add_tag(&mut self) {
        self.set_opacity_for_current_picture(0.25);
        self.editor
            .begin(&self.main_window(), EntryKind::AddTag, Some(self.repository.all_labels()));
        self.state.set_mode(Mode::Editing);
    }

    pub fn remove_tag(&mut self) {
        self.set_opacity_for_current_picture(0.25);
        self.editor.begin(
            &self.main_window(),
            EntryKind::RemoveTag,
            Some(self.current_picture().tags()),
        );
        self.state.set_mode(Mode::Editing);
    }

    pub fn label(&mut self) {
        self.set_opacity_for_current_picture(0.25);
        self.editor
            .begin(&self.main_window(), EntryKind::Label, Some(self.repository.all_labels()));
        self.state.set_mode(Mode::Editing);
    }

    fn rank_picture_at_index(&mut self, index: usize, rank: Rank) {
        let mut picture = self.gallery.picture(index);
        picture.set_rank(rank);
        self.gallery.set_picture(index, picture.clone());
        if self.args.on_database() {
            match self.database.update_picture(&picture) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }

    pub fn rank_selected_pictures(&mut self, rank: Rank) {
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.rank_picture_at_index(index, rank);
                }
            }
            self.navigator.unselect_all();
        } else {
            self.rank_picture_at_index(self.navigator().position(), rank)
        };
        self.navigator.set_page_changed();
        self.last_action = Action::Rank(rank);
    }

    pub fn jump(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::Number, None);
        self.state.set_mode(Mode::Editing);
    }

    pub fn find(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::Find, None);
        self.state.set_mode(Mode::Editing);
    }

    pub fn find_label(&mut self) {
        self.editor.begin(
            &self.main_window(),
            EntryKind::FindLabel,
            Some(self.gallery.all_labels()),
        );
        self.state.set_mode(Mode::Editing);
    }

    pub fn information(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::Information, None);
        self.editor
            .set_input(&format!("{}", self.current_picture().file_path()));
        self.state.set_mode(Mode::Editing);
    }

    pub fn toggle_information(&mut self) {
        self.state.toggle_display_information_on();
        let navigator = &mut self.navigator;
        navigator.set_page_changed()
    }

    pub fn find_pattern(&mut self, pattern: &str) {
        if let Some(index) = self
            .gallery
            .pictures()
            .iter()
            .position(|picture| picture.file_path().contains(pattern))
        {
            let navigator = &mut self.navigator;
            navigator.move_towards(Direction::Index { value: index });
            navigator.set_page_changed()
        }
    }

    pub fn find_pattern_in_label(&mut self, pattern: &str) {
        if let Some(index) = self
            .gallery
            .pictures()
            .iter()
            .position(|picture| picture.label().contains(pattern))
        {
            let navigator = &mut self.navigator;
            navigator.move_towards(Direction::Index { value: index });
            navigator.set_page_changed()
        }
    }

    pub fn quit(&mut self) {
        if self.state.has_saved_args() {
            self.back_from_directory()
        } else {
            let application_window = self.main_window().application_window();
            application_window.close()
        }
    }

    pub fn reload(&mut self) {
        match self.load_gallery() {
            Ok(_) => {
                self.move_first();
                self.navigator.set_page_changed();
            }
            Err(e) => {
                eprintln!("{}", e);
                self.quit()
            }
        }
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
        println!(
            "display date {}",
            if self.state().display_date_on() {
                String::from("on")
            } else {
                String::from("off")
            }
        );
    }

    pub fn toggle_display_focus_symbol_change(&mut self) {
        self.state.toggle_change_focus_symbol()
    }

    pub fn toggle_display_size(&mut self) {
        self.state.toggle_display_size();
        self.main_window().set_title(self);
        println!(
            "display size {}",
            if self.state().display_size_on() {
                String::from("on")
            } else {
                String::from("off")
            }
        );
    }

    pub fn toggle_full_size(&mut self) {
        if self.state.single_view() {
            self.state.toggle_full_size();
            let navigator = &mut self.navigator;
            navigator.set_page_changed();
        }
    }

    pub fn toggle_palette(&mut self) {
        self.state.toggle_palette();
        let navigator = &mut self.navigator;
        navigator.set_page_changed()
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
            self.navigator
                .move_towards(Direction::Index { value: index })
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

    pub fn set_range(&mut self) {
        let position = self.navigator.position();
        let navigator = &mut self.navigator;
        navigator.set_range(position);
        self.navigator.set_page_changed()
    }

    pub fn set_range_all(&mut self) {
        let navigator = &mut self.navigator;
        navigator.set_range_all();
        self.navigator.set_page_changed()
    }

    pub fn repeat_range(&mut self) {
        let navigator = &mut self.navigator;
        navigator.repeat_range();
        self.navigator.set_page_changed()
    }

    pub fn toggle_selected(&mut self) {
        let position = self.navigator.position();
        let navigator = &mut self.navigator;
        if navigator.is_selected(position) {
            navigator.unselect(position)
        } else {
            navigator.select(position)
        }
        self.navigator.set_page_changed()
    }

    pub fn cancel_range(&mut self) {
        let navigator = &mut self.navigator;
        navigator.cancel_range();
        self.navigator.set_page_changed()
    }

    fn delete_selected_pictures(&mut self) {
        for index in self.navigator.selection() {
            let picture = &self.gallery.picture(index);
            match delete_picture(&self.database, &picture.file_path()) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }

    fn move_selected_pictures(&mut self) {
        if let Some(target_dir) = &self.args.r#move {
            let mut picture_count = 0;
            let mut operation_count = 0;
            for index in self.navigator.selection() {
                let picture = &self.gallery.picture(index);
                let operations = move_picture(&picture.file_path(), &target_dir);
                if operations.is_empty() {
                    println!(
                        "no operation for move of {} to {}",
                        picture.file_path(),
                        target_dir
                    );
                } else {
                    picture_count += 1;
                    operation_count += operations.len();
                    match execute(&self.database, &operations) {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                }
            }
            println!(
                "{} pictures moved to {}\n{} operations\nexiting gsr",
                picture_count, target_dir, operation_count
            );
            self.reload()
        }
    }
    pub fn cancel_delete_picture(&mut self) {
        let navigator = &mut self.navigator;
        navigator.cancel_range();
        self.navigator.set_page_changed()
    }

    pub fn confirm_delete_picture(&mut self) {
        self.delete_selected_pictures();
        self.reload();
        self.navigator.set_page_changed()
    }

    pub fn confirm_move_picture(&mut self) {
        if let Some(target_dir) = &self.args.r#move {
            self.move_selected_pictures()
        }
    }

    pub fn cancel_move_picture(&mut self) {
        let navigator = &mut self.navigator;
        navigator.cancel_range();
        self.navigator.set_page_changed()
    }

    pub fn delete_picture(&mut self) {
        if self.navigator.has_selected() {
            self.editor
                .begin(&self.main_window(), EntryKind::DeleteConfirmation, None);
            self.state.set_mode(Mode::Editing);
        }
    }

    pub fn move_picture(&mut self) {
        if let Some(target_dir) = &self.args.r#move {
            self.editor
                .begin(&self.main_window(), EntryKind::MoveConfirmation, None);
            self.editor
                .set_prompt(&format!("move these pictures to {} ?", target_dir));
            self.state.set_mode(Mode::Editing);
        }
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

    pub fn repeat_last_action(&mut self) {
        let action = self.last_action.clone();
        match action {
            Action::NoAction => {}
            Action::Label(label) => self.label_selected_pictures(&label),
            Action::Unlabel => self.unlabel_selected_pictures(),
            Action::AddTag(label) => self.tag_selected_pictures(&label),
            Action::RemoveTag(label) => self.untag_selected_pictures(&label),
            Action::Rank(rank) => self.rank_selected_pictures(rank),
        }
    }
}
