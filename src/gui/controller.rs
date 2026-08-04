use crate::model::label::Label;
use crate::gui::enter_label::enter_label;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::cli::command::Command;
use crate::env::configuration::Configuration;
use crate::file::paths::check_path_exists;
use crate::file::paths::grand_parent_directory;
use crate::file::paths::parent_directory;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::control::{Control, Controls, default_controls, help_on_controls};
use crate::gui::direction::Direction;
use crate::gui::display_information::display_information;
use crate::gui::editor::Editor;
use crate::gui::editor::entry_editor::EntryEditor;
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;
use crate::gui::event::Event;
use crate::gui::mode::Mode;
use crate::gui::navigator::Navigator;
use crate::gui::selector::Selector;
use crate::gui::state::State;
use crate::gui::validator::Validator;
use crate::gui::view::entry_view::EntryView;
use crate::gui::view::main_window::{LEFT_PANE, MainWindow};
use crate::model::action::Action;
use crate::model::catalog::Catalog;
use crate::model::category::Category;
use crate::model::category::category_from_string;
use crate::model::change::Change;
use crate::model::find::Find;
use crate::model::finder::Predicate;
use crate::model::finder::predicate;
use crate::model::order::Order;
use crate::model::picture::Picture;
use crate::model::rank::Rank;
use crate::model::repository::Repository;
use crate::model::selection_criteria::SelectionCriteria;
use crate::model::tags::Tags;
use crate::model::tags::tags_from_str;
use gdk::{Key, ModifierType};
use gtk::prelude::*;
use gtk::{self, gdk};
use rand::Rng;
use rand::rng;
use std::cell::RefCell;
use std::io::Error;
use std::io::Result as IOResult;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

pub mod main_controller;
#[derive(Debug)]
pub struct Controller {
    configuration: Configuration,
    repository: Repository,
    command_line_arguments: CommandLineArguments,
    navigator: Navigator,
    controls: Controls,
    state: State,
    main_window_opt: Option<MainWindow>,
    editor: Editor,
    selector: Selector,
    last_action: Action,
}

pub type RcController = Rc<RefCell<Controller>>;

impl Controller {
    pub fn new(config: Configuration, command_line_arguments: CommandLineArguments) -> IOResult<Self> {
        let pictures_per_row = if let Some(grid) = command_line_arguments.grid {
            grid
        } else {
            match command_line_arguments.pictures_per_row() {
                1 => config.current_pictures_per_row.unwrap_or(1),
                n => n.try_into().unwrap(),
            }
            .try_into()
            .unwrap()
        };
        let mut cli = command_line_arguments.clone();

        if cli.order.is_none() {
            if let Some(order) = config.current_order {
                cli.order = Some(order)
            } else {
                cli.order = Some(Order::Name)
            }
        };
        if config.cover {
            cli.cover = !command_line_arguments.all;
        }
        let mut repository = Repository::new(config.clone(), cli.clone(), false);
        match repository.initialize(None) {
            Ok(_) => {}
            Err(e) => panic!("can't initialize repository: {}", e),
        };
        println!("{} pictures", repository.len());
        let catalog: Catalog = match Catalog::from_file(&config.catalog_filepath) {
            Ok(cat) => cat,
            Err(e) => {
                return Err(Error::other(format!(
                    "cannot log catalog file {} {}",
                    config.catalog_filepath, e
                )));
            }
        };
        let controller = Controller {
            configuration: config.clone(),
            repository: repository.clone(),
            command_line_arguments: cli.clone(),
            editor: Editor::new(),
            selector: Selector::new(&catalog),
            navigator: Navigator::new(repository.len(), pictures_per_row as usize),
            controls: default_controls(),
            state: State::new(pictures_per_row as usize, cli.slideshow().is_some()),
            main_window_opt: None,
            last_action: Action::Nothing,
        };
        Ok(controller)
    }

    pub fn command_line_arguments(&self) -> CommandLineArguments {
        self.command_line_arguments.clone()
    }

    pub fn repository(&self) -> Repository {
        self.repository.clone()
    }
    pub fn selector(&self) -> Selector {
        self.selector.clone()
    }

    pub fn set_selected(&mut self, selected: &str) {
        self.selector.set_selected(selected);
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

    pub fn set_navigator(&mut self, navigator: Navigator) {
        self.navigator = navigator;
    }

    pub fn current_picture(&self) -> Picture {
        self.repository.picture_at(self.navigator.position())
    }

    fn load_repository(&mut self) -> IOResult<usize> {
        println!("loading directory");
        let args = self.command_line_arguments.clone();
        let result = match args.command {
            Some(Command::File { file_path }) => {
                match self.repository.picture_from_file_path(&file_path) {
                    Ok(gallery) => Ok(gallery.len()),
                    Err(e) => Err(e),
                }
            }
            Some(Command::Directory { directory }) => {
                match self.repository.pictures_in_directory(&directory) {
                    Ok(gallery) => Ok(gallery.len()),
                    Err(e) => Err(e),
                }
            }
            None => Ok(self.repository.len()),
            _ => Ok(0),
        };
        match result {
            Ok(0) => {
                println!("no pictures\n");
                Ok(0)
            }
            Err(e) => Err(e),
            Ok(count) => {
                println!("{} pictures", count);
                self.navigator =
                    Navigator::new(self.repository.len(), self.state.pictures_per_row());
                Ok(count)
            }
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
            Event::PictureClicked { button, col, row } => {
                self.process_picture_clicked(button, col, row);
                self.set_slideshow_off()
            }
            Event::PictureDoubleClicked { button, col, row } => {
                self.process_picture_double_clicked(button, col, row);
                self.set_slideshow_off()
            }
        }
    }

    fn set_slideshow_off(&mut self) {
        if self.state().slideshow_on() {
            println!("setting slideshow off…");
            self.state.set_slideshow_off();
        }
    }
    fn process_picture_clicked(&mut self, button: u32, col: i32, row: i32) {
        self.main_window()
            .set_label_text_for_current_picture(self, None);
        if let Some(index) = self
            .navigator
            .position_from_coords(row as usize, col as usize)
            && self.navigator.can_move(Direction::Index { value: index })
        {
            self.navigator
                .move_towards(Direction::Index { value: index });
            if button == 3 {
                self.toggle_selected();
                self.main_window().set_pictures(self);
                self.main_window().set_title(self);
            }
        }
        self.set_label_text_for_current_picture();
    }

    fn process_picture_double_clicked(&mut self, button: u32, col: i32, row: i32) {
        self.main_window()
            .set_label_text_for_current_picture(self, None);
        if let Some(index) = self
            .navigator
            .position_from_coords(row as usize, col as usize)
            && self.navigator.can_move(Direction::Index { value: index })
        {
            self.navigator
                .move_towards(Direction::Index { value: index });
            if button == 1 {
                let main_window = self.main_window();
                main_window.set_label_text_for_current_picture(self, None);
                let old_slideshow_on = self.state().slideshow_on();
                self.process_control(&Control::SetSelectionRange);
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
            } else if button == 3 {
                self.toggle_selected();
                self.main_window().set_pictures(self);
                self.main_window().set_title(self);
            }
        }
        self.set_label_text_for_current_picture();
    }

    fn process_pane_clicked(&mut self, _button: usize, pane_number: usize) {
        self.process_control(if pane_number == LEFT_PANE {
            &Control::MovePrev
        } else {
            &Control::MoveNext
        });
        if self.navigator.has_moved() {
            self.main_window().set_pictures(self)
        }
    }

    fn process_key_event(
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

    fn set_opacity_for_current_picture(&mut self, opacity: f64) {
        self.main_window()
            .set_opacity_for_current_picture(self, opacity)
    }

    fn process_key(&mut self, key: Key) {
        const SHIFT_L: &str = "Shift_L";
        const SHIFT_R: &str = "Shift_R";
        if let Some(name) = key.name()
            && (name == SHIFT_L || name == SHIFT_R)
        {
            return;
        }
        let controls = self.controls.clone();
        match self.state().mode() {
            Mode::MovingToCategory => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        self.move_sub_category_to_category(
                            &self.selector.prev_selected(),
                            &self.selector.selected(),
                        )
                    }
                }
            }
            Mode::MovingCategory => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        self.move_to_category(&self.selector.selected())
                    }
                }
            }
            Mode::AddingCategory => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        self.add_category(&self.editor.input(), &self.selector.selected())
                    }
                }
            }
            Mode::RemovingCategory => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        self.remove_category(&self.selector.selected())
                    }
                }
            }
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
            Mode::Categorizing => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        let category: Category = category_from_string(&self.selector.selected());
                        self.categorize_selected_pictures(category)
                    }
                    self.set_opacity_for_current_picture(1.00);
                }
            }
            Mode::SelectingCategory => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        self.find_first(&self.editor.input(), Find::SubCategory);
                    }
                }
            }
            Mode::FindingSubCategory => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        let category_name = self.selector.selected();
                        self.find_first(&category_name, Find::SubCategory)
                    }
                }
            }
            Mode::SelectingSubCategory => {
                self.selector.process(key);
                if !self.selector.selecting() {
                    self.state.set_mode(Mode::View);
                    if !self.selector.selected().is_empty() {
                        let category_name = self.selector.selected();
                        self.select(&category_name, Find::SubCategory)
                    }
                }
            }
            Mode::Editing => {
                self.editor.process(key);
                if !self.editor.editing() {
                    self.state.set_mode(Mode::View);
                    match self.editor.entry_kind() {
                        EntryKind::AddCategory => {
                            if !self.editor.input().is_empty() {
                                self.adding_category(&self.editor.input())
                            }
                        }
                        EntryKind::MoveCategory => {}
                        EntryKind::RemoveCategory => {}
                        EntryKind::Catalog => {
                            if !self.editor.input().is_empty() {
                                match Change::from_str(&self.editor.input()) {
                                    Ok(Change::AddCategory) => self.enter_add_category(),
                                    Ok(Change::MoveCategory) => self.enter_move_category(),
                                    Ok(Change::RemoveCategory) => self.enter_remove_category(),
                                    _ => {}
                                }
                            };
                        }
                        EntryKind::Change => {
                            if !self.editor.input().is_empty() {
                                match Change::from_str(&self.editor.input()) {
                                    Ok(Change::AddTag) => self.add_tag(),
                                    Ok(Change::Catalog) => self.enter_change_catalog(),
                                    Ok(Change::Category) => self.categorize(),
                                    Ok(Change::Cover) => self.toggle_cover(),
                                    Ok(Change::Label) => self.label(),
                                    Ok(Change::Name) => self.rename(),
                                    Ok(Change::RemoveTag) => self.remove_tag(),
                                    Ok(Change::Unlabel) => self.unlabel_selected_pictures(),
                                    _ => {}
                                }
                            };
                        }
                        EntryKind::Rename => {
                            if !self.editor.input().is_empty() {
                                self.rename_selected_picture(&self.editor.input())
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::Categorize => {
                            if !self.editor.input().is_empty() {
                                self.categorize_selected_pictures(Some(self.editor.input()))
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::Select => {
                            if !self.editor.input().is_empty() {
                                match Find::from_str(&self.editor.input()) {
                                    Ok(Find::Label) => self.enter_select_label(),
                                    Ok(Find::Name) => self.enter_select_name(),
                                    Ok(Find::Category) => self.enter_select_category(),
                                    Ok(Find::SubCategory) => self.enter_select_sub_category(),
                                    Ok(Find::SomeTags) => self.enter_select_tags(false),
                                    Ok(Find::AllTags) => self.enter_select_tags(true),
                                    _ => {}
                                };
                            }
                        }
                        EntryKind::Find => {
                            if !self.editor.input().is_empty() {
                                match Find::from_str(&self.editor.input()) {
                                    Ok(Find::Label) => self.enter_find_label(),
                                    Ok(Find::Name) => self.enter_find_name(),
                                    Ok(Find::Category) => self.enter_find_category(),
                                    Ok(Find::SubCategory) => self.enter_find_sub_category(),
                                    Ok(Find::SomeTags) => self.enter_find_tags(false),
                                    Ok(Find::AllTags) => self.enter_find_tags(true),
                                    _ => {}
                                };
                            }
                        }
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
                            if !self.editor.input().is_empty() {
                                self.move_towards_index(self.editor.input().parse().unwrap())
                            };
                        }
                        EntryKind::Order => self.set_order(&self.editor.input()),
                        EntryKind::Rank => self.confirm_rank(&self.editor.input()),
                        EntryKind::View => self.confirm_view(&self.editor.input()),
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
                        EntryKind::MoveToLabelConfirmation(ref target) => {
                            if &self.editor.input() == "yes" {
                                self.confirm_move_picture_to_label(target)
                            } else {
                                self.cancel_move_picture()
                            }
                        }
                        EntryKind::FindName => {
                            if !self.editor.input().is_empty() {
                                self.find_first(&self.editor.input(), Find::Name);
                            };
                        }
                        EntryKind::FindLabel => {
                            if !self.editor.input().is_empty() {
                                self.find_first(&self.editor.input(), Find::Label);
                            };
                        }
                        EntryKind::FindCategory => {
                            if !self.editor.input().is_empty() {
                                self.find_first(&self.editor.input(), Find::Category);
                            };
                        }
                        EntryKind::FindSubCategory => {
                            if !self.editor.input().is_empty() {
                                self.find_first(&self.editor.input(), Find::SubCategory);
                            };
                        }
                        EntryKind::FindSomeTags => {
                            if !self.editor.input().is_empty() {
                                self.find_first(&self.editor.input(), Find::AllTags)
                            }
                        }
                        EntryKind::FindAllTags => {
                            if !self.editor.input().is_empty() {
                                self.find_first(&self.editor.input(), Find::AllTags)
                            };
                        }
                        EntryKind::SelectName => {
                            if !self.editor.input().is_empty() {
                                self.select(&self.editor.input(), Find::Name);
                            };
                        }
                        EntryKind::SelectLabel => {
                            if !self.editor.input().is_empty() {
                                self.select(&self.editor.input(), Find::Label);
                            };
                        }
                        EntryKind::SelectCategory => {
                            if !self.editor.input().is_empty() {
                                self.select(&self.editor.input(), Find::Category);
                            };
                        }
                        EntryKind::SelectSubCategory => {
                            if !self.editor.input().is_empty() {
                                self.select(&self.editor.input(), Find::SubCategory);
                            };
                        }
                        EntryKind::SelectSomeTags => {
                            if !self.editor.input().is_empty() {
                                self.select(&self.editor.input(), Find::AllTags)
                            }
                        }
                        EntryKind::SelectAllTags => {
                            if !self.editor.input().is_empty() {
                                self.find_first(&self.editor.input(), Find::AllTags)
                            };
                        }
                        EntryKind::Information => {}
                        EntryKind::Help => {}
                    }
                }
            }
        }
    }

    fn set_order(&mut self, input: &str) {
        let choice: Control = match input {
            "Category" => Control::OrderByCategory,
            "ColorCount" => Control::OrderByColorCount,
            "Date" => Control::OrderByDate,
            "Label" => Control::OrderByLabel,
            "Cover" => Control::OrderByCover,
            "Name" => Control::OrderByName,
            "Palette" => Control::OrderByPalette,
            "Random" => Control::Randomize,
            "Size" => Control::OrderBySize,
            "Score" => Control::OrderByScore,
            "Value" => Control::OrderByValue,
            &_ => Control::CancelEdition,
        };
        self.process_control(&choice)
    }

    fn rename_selected_picture(&mut self, name: &str) {
        for index in self.navigator.selection() {
            match self.repository.rename_picture_at_index(index, name) {
                Ok(count) => {
                    println!("{} picture renamed", count);
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        match self.repository.initialize_for_args(&self.command_line_arguments, None) {
            Ok(()) => {
                let _ = self.reload();
                self.navigator.set_page_changed();
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    fn label_picture_at_index(&mut self, index: usize, label: &str) {
        let mut picture = self.repository.picture_at(index);
        picture.set_label(label);
        self.repository.set_picture_at(index, &picture);
        self.last_action = Action::Label(label.to_string());
    }

    fn label_selected_pictures(&mut self, label: &str) {
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

    fn unlabel_selected_pictures(&mut self) {
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

    fn tag_picture_at_index(&mut self, index: usize, input: &str) {
        let labels: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
        let mut picture = self.repository.picture_at(index);
        labels.iter().for_each(|label| {
            self.repository.add_label(label);
            picture.add_tag(label);
        });
        self.repository.set_picture_at(index, &picture);
    }

    fn untag_picture_at_index(&mut self, index: usize, label: &str) {
        let mut picture = self.repository.picture_at(index);
        picture.remove_tag(label);
        self.repository.set_picture_at(index, &picture);
    }

    fn tag_selected_pictures(&mut self, labels: &str) {
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.tag_picture_at_index(index, labels);
                }
            }
            self.navigator.unselect_all();
        } else {
            self.tag_picture_at_index(self.navigator().position(), labels)
        };
        self.navigator.set_page_changed();
        self.last_action = Action::AddTag(labels.to_string());
    }

    fn untag_selected_pictures(&mut self, label: &str) {
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

    fn move_next(&mut self) {
        if self.state().search_in_progress() {
            self.find_next()
        } else {
            self.move_towards(Direction::NextPage)
        };
    }
    fn move_towards_index(&mut self, index: usize) {
        let direction = Direction::Index { value: index };
        if self.navigator().can_move(direction.clone()) {
            self.navigator.move_towards(direction)
        }
    }

    fn set_setting(&mut self, setting: &Control, choice: &Control) {
        match setting {
            Control::SetMark => match choice {
                Control::SetMarkChar(_) => self.process_control(choice),
                _ => println!("?"),
            },
            Control::GotoMark => match choice {
                Control::JumpMarkChar(_) => self.process_control(choice),
                _ => println!("?"),
            },
            Control::SetDisplay => match choice {
                Control::DisplayDate | Control::DisplaySize => self.process_control(choice),
                Control::DisplayFocus => self.toggle_display_focus_symbol_change(),
                _ => println!("?"),
            },
            Control::SetOrder => match choice {
                Control::OrderByCategory
                | Control::OrderByName
                | Control::OrderByDate
                | Control::OrderBySize
                | Control::OrderByValue
                | Control::OrderByLabel
                | Control::OrderByColorCount
                | Control::OrderByPalette
                | Control::OrderByScore
                | Control::Randomize => self.process_control(choice),
                _ => println!("?"),
            },
            _ => {}
        }
    }

    fn setting_display(&mut self) {
        println!("Setting display…");
        self.state.set_mode(Mode::Setting(Control::SetDisplay));
    }

    fn enter_change(&mut self) {
        self.enter_editing(EntryKind::Change, None)
    }

    fn enter_change_catalog(&mut self) {
        self.enter_editing(EntryKind::Catalog, None)
    }

    fn enter_add_category(&mut self) {
        self.enter_editing(EntryKind::AddCategory, None)
    }

    fn adding_category(&mut self, category_name: &str) {
        if !self.repository.catalog().contains(category_name) {
            self.selector.begin(
                &self.main_window(),
                &format!("select a category to add {} to ", category_name),
                &self.repository.catalog(),
            );
            self.state.set_mode(Mode::AddingCategory);
        } else {
            display_information(
                &self.main_window().application_window(),
                &format!("the category {} already exists", category_name),
            );
        }
    }
    fn enter_move_category(&mut self) {
        self.selector.begin(
            &self.main_window(),
            "select the category to move",
            &self.repository.catalog(),
        );
        self.state.set_mode(Mode::MovingCategory);
    }

    fn move_to_category(&mut self, moving_category_name: &str) {
        let mut pruned_catalog = self.repository.catalog();
        let _ = pruned_catalog.remove_category(moving_category_name, true);
        self.selector.set_prev_selected(moving_category_name);
        self.selector.begin(
            &self.main_window(),
            &format!("select the category where to move {}", moving_category_name),
            &pruned_catalog,
        );
        self.state.set_mode(Mode::MovingToCategory);
    }

    fn enter_remove_category(&mut self) {
        self.selector.begin(
            &self.main_window(),
            "select a category to remove",
            &self.repository.catalog(),
        );
        self.state.set_mode(Mode::RemovingCategory);
    }

    fn enter_find(&mut self) {
        self.enter_editing(EntryKind::Find, None)
    }

    fn enter_select(&mut self) {
        if !self.state().has_saved_command_line_arguments() {
            self.enter_editing(EntryKind::Select, None)
        } else {
            display_information(
                &self.main_window().application_window(),
                "selection not allowed while in a directory or a selection",
            )
        }
    }

    fn enter_editing(&mut self, entry_kind: EntryKind, choice_opt: Option<Tags>) {
        self.editor
            .begin(&self.main_window(), entry_kind, choice_opt);
        self.state.set_mode(Mode::Editing);
    }

    fn enter_find_label(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::FindLabel, None);
        self.state.set_mode(Mode::Editing);
    }

    fn enter_find_name(&mut self) {
        println!("enter_find_name");
        self.editor
            .begin(&self.main_window(), EntryKind::FindName, None);
        self.state.set_mode(Mode::Editing);
    }

    fn enter_find_category(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::FindCategory, None);
        self.state.set_mode(Mode::Editing);
    }

    fn enter_find_tags(&mut self, all_match: bool) {
        let kind = if all_match {
            EntryKind::FindAllTags
        } else {
            EntryKind::FindSomeTags
        };
        self.editor.begin(&self.main_window(), kind, None);
        self.state.set_mode(Mode::Editing);
    }
    fn enter_find_sub_category(&mut self) {
        self.set_category_selection();
        self.state.set_mode(Mode::FindingSubCategory);
    }

    fn enter_select_label(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::SelectLabel, None);
        self.state.set_mode(Mode::Editing);
    }

    fn enter_select_name(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::SelectName, None);
        self.state.set_mode(Mode::Editing);
    }

    fn enter_select_category(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::SelectCategory, None);
        self.state.set_mode(Mode::Editing);
    }

    fn enter_select_tags(&mut self, all_match: bool) {
        let kind = if all_match {
            EntryKind::SelectAllTags
        } else {
            EntryKind::SelectSomeTags
        };
        self.editor.begin(&self.main_window(), kind, None);
        self.state.set_mode(Mode::Editing);
    }
    fn enter_select_sub_category(&mut self) {
        self.set_category_selection();
        self.state.set_mode(Mode::SelectingSubCategory);
    }
    fn setting_mark(&mut self) {
        println!("Setting mark…");
        self.state.set_mode(Mode::Setting(Control::SetMark));
    }

    fn jumping_mark(&mut self) {
        println!("Jumping to mark…");
        self.state.set_mode(Mode::Setting(Control::GotoMark));
    }

    fn set_mark(&mut self, mark: char) {
        let file_path = self.current_picture().file_path();
        let _ = self.configuration.marked.insert(mark, file_path.clone());
        println!("{}={}", mark, file_path);
        let _ = self.configuration.save();
    }
    fn setting_order(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::Order, None);
        self.state.set_mode(Mode::Editing);
    }

    fn next_slide_delay(&mut self) {
        self.move_towards(Direction::NextPage);
        self.main_window().set_pictures(self)
    }

    fn process_control(&mut self, control: &Control) {
        match control {
            Control::Test => self.test(),
            Control::AddTag => self.add_tag(),
            Control::BackFromDirectory => self.back_from_directory(),
            Control::CancelRange => self.cancel_range(),
            Control::CancelSelection => self.cancel_selection_criteria(),
            Control::Categorize => self.categorize(),
            Control::CopyTemp => self.copy_to_temp(),
            Control::DeletePicture => self.delete_picture(),
            Control::DisplayDate => self.toggle_display_date(),
            Control::DisplaySize => self.toggle_display_size(),
            Control::Down => self.arrow_move(Direction::Down),
            Control::EnterChange => self.enter_change(),
            Control::EnterFind => self.enter_find(),
            Control::EnterSelect => self.enter_select(),
            Control::SetView => self.enter_editing(EntryKind::View, None),
            Control::EnterRank => self.enter_editing(EntryKind::Rank, None),
            Control::ExtractFileNames => self.extract_filenames(),
            Control::FindName => self.enter_find_name(),
            Control::FindNext => self.find_next(),
            Control::GotoDirectory => self.go_to_directory(),
            Control::GotoMark => self.jumping_mark(),
            Control::Help => self.help(),
            Control::Information => self.information(),
            Control::Jump => self.jump(),
            Control::JumpMarkChar(ch) => self.find_mark(*ch),
            Control::Label => self.label(),
            Control::Left => self.arrow_move(Direction::Left),
            Control::MoveEndPage => self.move_towards(Direction::PageEnd),
            Control::MoveFirst => self.move_towards(Direction::First),
            Control::MoveLast => self.move_towards(Direction::Last),
            Control::MoveNext => self.move_next(),
            Control::MovePicture => self.move_picture(),
            Control::MovePictureToLabel => self.move_picture_to_label(),
            Control::MovePrev => self.move_towards(Direction::PrevPage),
            Control::MoveRandom => self.move_towards(Direction::Index {
                value: rng().random_range(0..self.navigator.limit()),
            }),
            Control::MoveStartPage => self.move_towards(Direction::PageStart),
            Control::OrderByCategory => self.order_by(Order::Category),
            Control::OrderByColorCount => self.order_by(Order::ColorCount),
            Control::OrderByCover => self.order_by(Order::Cover),
            Control::OrderByDate => self.order_by(Order::Date),
            Control::OrderByLabel => self.order_by(Order::Label),
            Control::OrderByName => self.order_by(Order::Name),
            Control::OrderByPalette => self.order_by(Order::Palette),
            Control::OrderByScore => self.order_by(Order::Score),
            Control::OrderBySize => self.order_by(Order::Size),
            Control::OrderByValue => self.order_by(Order::Value),
            Control::Quit => self.quit(),
            Control::Randomize => self.order_by(Order::Random),
            Control::RankNoStar => self.rank_selected_pictures(Rank::NoStar),
            Control::RankOneStar => self.rank_selected_pictures(Rank::OneStar),
            Control::RankThreeStars => self.rank_selected_pictures(Rank::ThreeStars),
            Control::RankTwoStars => self.rank_selected_pictures(Rank::TwoStars),
            Control::RemoveTag => self.remove_tag(),
            Control::Rename => {
                if self.navigator.has_selected() && self.navigator.selected_picture_count() == 1 {
                    self.enter_editing(EntryKind::Rename, None)
                } else {
                    eprintln!("select the picture to rename first")
                }
            }
            Control::RepeatLastAction => self.repeat_last_action(),
            Control::RepeatRange => self.repeat_range(),
            Control::Right => self.arrow_move(Direction::Right),
            Control::SelectCategory => self.set_category_selection(),
            Control::SetDisplay => self.setting_display(),
            Control::SetMark => self.setting_mark(),
            Control::SetMarkChar(ch) => self.set_mark(*ch),
            Control::SetOrder => self.setting_order(),
            Control::SetSelectionRange => self.set_selection_range(),
            Control::SetSelectionRangeAll => self.set_selection_range_all(),
            Control::SetSelectionRangePage => self.set_selection_range_page(),
            Control::ToggleCover => self.toggle_cover(),
            Control::ToggleThumbView => self.toggle_thumbview(),
            Control::ToggleCoverSelection => self.toggle_cover_selection(),
            Control::ToggleExpand => self.toggle_expand(),
            Control::ToggleFullSize => self.toggle_full_size(),
            Control::DisplayPath => self.toggle_display_path(),
            Control::TogglePalette => self.toggle_palette(),
            Control::ToggleSelected => self.toggle_selected(),
            Control::ToggleSingleView => self.toggle_single_view(),
            Control::ToggleSlideShow => self.toggle_slideshow(),
            Control::Uncategorize => self.uncategorize_selected_pictures(),
            Control::Unlabel => self.unlabel_selected_pictures(),
            Control::Up => self.arrow_move(Direction::Up),
            _ => {}
        }
    }

    fn go_to_directory(&mut self) {
        if self.command_line_arguments.cover
            && let Some(directory) = parent_directory(&self.current_picture().file_path())
            && Some(directory.clone()) != self.command_line_arguments.directory
            && !self.state.single_view()
        {
            self.command_line_arguments.index = Some(self.navigator.position());
            let clargs = self.command_line_arguments.clone();
            self.state.push_saved_command_line_arguments(clargs.clone(), &directory);
            let new_clargs = CommandLineArguments {
                directory: Some(directory),
                cover: false,
                ..clargs.clone()
            };
            self.command_line_arguments = new_clargs.clone();
            match self.repository.initialize_for_args(&new_clargs, None) {
                Ok(()) => {
                    let _ = self.reload();
                    self.navigator.set_page_changed();
                }
                Err(e) => eprintln!("{}", e),
            }
        } else {
            display_information(
                &self.main_window().application_window(),
                "cannot go to a directory when not in covers view",
            )
        }
    }

    fn go_to_selection(&mut self, selection: &str, predicate: Predicate) -> Option<String> {
        self.command_line_arguments.index = Some(self.navigator.position());
        let clargs = self.command_line_arguments.clone();
        self.state.push_saved_command_line_arguments(clargs.clone(), selection);
        let new_clargs = CommandLineArguments {
            directory: None,
            cover: false,
            ..clargs.clone()
        };
        self.command_line_arguments = new_clargs.clone();
        match self
            .repository
            .initialize_for_args(&new_clargs, Some(predicate))
        {
            Ok(()) => match self.reload() {
                Ok(0) => {
                    self.back_from_directory();
                    self.navigator.set_page_changed();
                    Some(format!("no picture found with: {}", selection))
                }
                Ok(_) => {
                    self.navigator.set_page_changed();
                    None
                }
                Err(e) => {
                    eprintln!("error:{}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    fn back_from_directory(&mut self) {
        if let Some((pictures_per_row, old_clargs)) = self.state.pop_saved_command_line_arguments() {
            self.command_line_arguments = old_clargs.clone();
            match self.repository.initialize_for_args(&old_clargs, None) {
                Ok(()) => {
                    self.change_grid_size(pictures_per_row);
                    let _ = self.reload();
                    if let Some(index) = self.command_line_arguments.index
                        && self.navigator.can_move(Direction::Index { value: index })
                    {
                        self.navigator
                            .move_towards(Direction::Index { value: index })
                    };
                    self.navigator.set_page_changed()
                }
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    fn toggle_single_view(&mut self) {
        let state = &mut self.state;
        state.toggle_single_view();
        if state.full_size_on() {
            state.toggle_full_size()
        }
        self.change_grid_size(self.state().pictures_per_row());
    }

    fn toggle_thumbview(&mut self) {
        let mut state = self.state();
        if state.pictures_per_row() != 10 {
            state.change_grid_size(10)
        } else {
            state.toggle_back_grid_size()
        };
        self.state = state;
        self.change_grid_size(self.state().pictures_per_row());
    }
    fn toggle_cover(&mut self) {
        let index = self.navigator().position();
        let counts = self.repository.directory_count_at_index(index);
        let mut picture = self.repository.picture_at(index);
        picture.toggle_cover(counts.0);
        self.repository.set_picture_at(index, &picture);
        self.navigator.set_page_changed()
    }

    fn rank_picture_at_index(&mut self, index: usize, rank: Rank) {
        let mut picture = self.repository.picture_at(index);
        picture.set_rank(rank);
        self.repository.set_picture_at(index, &picture);
    }

    fn categorize_picture_at_index(&mut self, index: usize, category_opt: Category) {
        let mut picture = self.repository.picture_at(index);
        picture.set_category(category_opt.clone());
        self.repository.set_picture_at(index, &picture);
    }

    fn toggle_cover_selection(&mut self) {
        println!("toggle cover selection");
        if !self.state.has_saved_command_line_arguments() {
            if !self.command_line_arguments.cover && self.repository.covers() > 0 {
                let new_clargs = CommandLineArguments {
                    cover: true,
                    ..self.command_line_arguments.clone()
                };
                self.command_line_arguments = new_clargs;
                match self.repository.initialize_for_args(&self.command_line_arguments, None) {
                    Ok(_) => match self.reload() {
                        Ok(0) => {
                            self.toggle_cover_selection();
                        }
                        Ok(_) => {}
                        Err(e) => panic!("{}", e),
                    },
                    Err(e) => panic!("{}", e),
                }
            } else if self.command_line_arguments.cover {
                let new_clargs = CommandLineArguments {
                    cover: false,
                    ..self.command_line_arguments.clone()
                };
                self.command_line_arguments = new_clargs;
                match self.repository.initialize_for_args(&self.command_line_arguments, None) {
                    Ok(_) => match self.reload() {
                        Ok(0) => {
                            self.toggle_cover_selection();
                        }
                        Ok(_) => {}
                        Err(e) => panic!("{}", e),
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        } else {
            display_information(
                &self.main_window().application_window(),
                "cannot toggle cover selection while in a directory",
            )
        }
    }

    fn cancel_selection_criteria(&mut self) {
        let current_file_path = self.current_picture().file_path();
        self.repository
            .set_selection_criteria(SelectionCriteria::empty());
        if let Some(index) = self.repository.find_index_for_file_path(&current_file_path) {
            self.navigator
                .move_towards(Direction::Index { value: index })
        } else {
            self.navigator.move_towards(Direction::First)
        };
        self.navigator.set_page_changed();
    }

    fn add_category(&mut self, new_category_name: &str, target_category_name: &str) {
        match self
            .repository
            .add_category(new_category_name, target_category_name)
        {
            Ok(_) => {}
            Err(e) => {
                display_information(&self.main_window().application_window(), &format!("{}", e))
            }
        }
    }

    fn move_sub_category_to_category(
        &mut self,
        moving_category_name: &str,
        target_category_name: &str,
    ) {
        match self
            .repository
            .move_category(moving_category_name, target_category_name)
        {
            Ok(_) => {}
            Err(e) => {
                display_information(&self.main_window().application_window(), &format!("{}", e))
            }
        }
    }

    fn remove_category(&mut self, input: &str) {
        let _ = self.repository.retrieve_all_categories();
        if !self.repository.all_categories().contains(input) {
            match self.repository.remove_category(input) {
                Ok(_) => {}
                Err(e) => {
                    display_information(&self.main_window().application_window(), &format!("{}", e))
                }
            }
        } else {
            display_information(
                &self.main_window().application_window(),
                &format!("category {} is being used and cannot be removed", input),
            )
        }
    }

    fn add_tag(&mut self) {
        self.set_opacity_for_current_picture(0.25);
        self.editor.begin(
            &self.main_window(),
            EntryKind::AddTag,
            Some(self.repository.all_labels()),
        );
        self.state.set_mode(Mode::Editing);
    }

    fn remove_tag(&mut self) {
        self.set_opacity_for_current_picture(0.25);
        self.editor.begin(
            &self.main_window(),
            EntryKind::RemoveTag,
            Some(self.current_picture().tags()),
        );
        self.state.set_mode(Mode::Editing);
    }

    fn label(&mut self) {
        self.set_opacity_for_current_picture(0.25);
        self.editor.begin(
            &self.main_window(),
            EntryKind::Label,
            Some(self.repository.all_labels()),
        );
        self.state.set_mode(Mode::Editing);
    }

    fn label_(&self) {
        let application_window = self.main_window().application_window();
        enter_label(&application_window, &self.repository);
    }

    fn rename(&mut self) {
        if self.navigator.has_selected() && self.navigator.selected_picture_count() == 1 {
            self.set_opacity_for_current_picture(0.25);
            self.editor
                .begin(&self.main_window(), EntryKind::Rename, None);
            self.state.set_mode(Mode::Editing);
        } else {
            self.editor
                .begin(&self.main_window(), EntryKind::Information, None);
            self.editor
                .set_input("Select the picture you want to rename first");
            self.state.set_mode(Mode::Editing);
        }
    }

    fn categorize_selected_pictures(&mut self, category: Category) {
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.categorize_picture_at_index(index, category.clone());
                }
            }
            self.navigator.unselect_all();
        } else {
            self.categorize_picture_at_index(self.navigator().position(), category.clone());
        };
        self.navigator.set_page_changed();
        self.last_action = Action::Categorize(category);
    }

    fn categorize(&mut self) {
        self.set_opacity_for_current_picture(0.25);
        self.selector.begin(
            &self.main_window(),
            "select a category to apply",
            &self.repository.catalog(),
        );
        self.state.set_mode(Mode::Categorizing);
    }

    fn set_category_selection(&mut self) {
        self.selector.begin(
            &self.main_window(),
            "select a category to find",
            &self.repository.catalog(),
        );
        self.state.set_mode(Mode::SelectingCategory);
    }

    fn uncategorize_selected_pictures(&mut self) {
        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    self.categorize_picture_at_index(index, None);
                }
            }
            self.navigator.unselect_all();
        } else {
            self.categorize_picture_at_index(self.navigator().position(), None)
        };
        self.navigator.set_page_changed();
        self.last_action = Action::Unlabel;
    }
    fn rank_selected_pictures(&mut self, rank: Rank) {
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

    fn jump(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::Number, None);
        self.state.set_mode(Mode::Editing);
    }

    fn help(&mut self) {
        display_information(
            &self.main_window().application_window(),
            &help_on_controls(),
        );
    }

    fn information(&mut self) {
        self.editor
            .begin(&self.main_window(), EntryKind::Information, None);
        self.editor
            .set_input(&self.current_picture().file_path().to_string());
        self.state.set_mode(Mode::Editing);
    }

    fn toggle_display_path(&mut self) {
        self.state.toggle_display_path();
        let navigator = &mut self.navigator;
        navigator.set_page_changed()
    }

    fn find_mark(&mut self, mark: char) {
        if let Some(file_path) = self.configuration.marked.get(&mark) {
            if let Ok(gallery) = self.repository.gallery_rc().try_borrow() {
                if let Some(index) = gallery
                    .pictures()
                    .iter()
                    .position(|picture| picture.file_path() == *file_path)
                {
                    let navigator = &mut self.navigator;
                    navigator.move_towards(Direction::Index { value: index });
                    navigator.set_page_changed()
                } else {
                    // display_information(&self.main_window().application_window(),&format!("mark: {} not found", mark));
                }
            } else {
                panic!("can't borrow")
            }
        } else {
            display_information(
                &self.main_window().application_window(),
                &format!("no picture with mark {}", mark),
            );
        }
    }

    fn quit(&mut self) {
        if self.state.has_saved_command_line_arguments() {
            self.back_from_directory()
        } else {
            self.configuration.current_picture = Some(self.current_picture().file_path());
            self.configuration.cover = self.command_line_arguments.cover;
            self.configuration.current_pictures_per_row = if self.state.single_view() {
                Some(1)
            } else {
                Some(self.state.pictures_per_row())
            };
            self.configuration.current_order = Some(self.repository.order());
            let _ = self.configuration.save();
            let application_window = self.main_window().application_window();
            self.repository.update_picture_scores(self.state().scores());
            application_window.close()
        }
    }

    fn reload(&mut self) -> Result<usize, Error> {
        match self.load_repository() {
            Ok(0) => Ok(0),
            Ok(n) => {
                self.move_towards(Direction::First);
                self.navigator.set_page_changed();
                Ok(n)
            }
            Err(e) => {
                eprintln!("{}", e);
                self.quit();
                Err(e)
            }
        }
    }

    fn toggle_expand(&mut self) {
        if self.state.single_view() {
            self.state.toggle_expand();
            let navigator = &mut self.navigator;
            navigator.set_page_changed();
        }
    }

    fn toggle_display_date(&mut self) {
        if self.state.display_path_on() {
            self.state.toggle_display_path();
        };
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

    fn toggle_display_focus_symbol_change(&mut self) {
        self.state.toggle_change_focus_symbol()
    }

    fn toggle_display_size(&mut self) {
        if self.state.display_path_on() {
            self.state.toggle_display_path();
        };
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

    fn toggle_full_size(&mut self) {
        if self.state.single_view() {
            self.state.toggle_full_size();
            let navigator = &mut self.navigator;
            navigator.set_page_changed();
        }
    }

    fn toggle_palette(&mut self) {
        self.state.toggle_palette();
        let navigator = &mut self.navigator;
        navigator.set_page_changed()
    }

    fn toggle_slideshow(&mut self) {
        if let Some(seconds) = self.command_line_arguments().slideshow() {
            self.state.toggle_slideshow();
            if self.state.slideshow_on() {
                self.main_window().reattach_slideshow_event(seconds);
                let navigator = &mut self.navigator;
                navigator.set_page_changed();
            }
        }
    }

    fn order_by(&mut self, order: Order) {
        let new_position: Option<usize>;
        let current_file_path = self.current_picture().file_path();
        if let Ok(mut gallery) = self.repository.gallery_rc().try_borrow_mut() {
            gallery.sort_by(order);
            new_position = gallery.find_file_path(&current_file_path);
            self.command_line_arguments.order = Some(order);
        } else {
            panic!("can't borrow mut")
        };
        if let Some(index) = new_position {
            self.navigator
                .move_towards(Direction::Index { value: index })
        } else {
            self.navigator.move_towards(Direction::First)
        };
        self.navigator.set_page_changed()
    }

    fn change_grid_size(&mut self, pictures_per_row: usize) {
        self.state.change_grid_size(pictures_per_row);
        self.apply_grid_size_change();
    }

    fn apply_grid_size_change(&mut self) {
        let pictures_per_row = self.state.pictures_per_row();
        self.navigator.set_pictures_per_row(pictures_per_row);
        self.navigator.update_page_limits();
        self.navigator.set_page_changed();
        self.main_window().change_grid_size(pictures_per_row);
    }

    fn set_selection_range(&mut self) {
        let position = self.navigator.position();
        self.navigator.set_selection_range(position);
        self.navigator.set_page_changed()
    }

    fn set_selection_range_all(&mut self) {
        let navigator = &mut self.navigator;
        navigator.set_selection_range_all();
        self.navigator.set_page_changed()
    }

    fn set_selection_range_page(&mut self) {
        let navigator = &mut self.navigator;
        navigator.set_selection_range_page();
        self.navigator.set_page_changed()
    }
    fn repeat_range(&mut self) {
        let navigator = &mut self.navigator;
        navigator.repeat_range();
        self.navigator.set_page_changed()
    }

    fn toggle_selected(&mut self) {
        let position = self.navigator.position();
        let navigator = &mut self.navigator;
        if navigator.is_selected(position) {
            navigator.unselect(position)
        } else {
            navigator.select(position)
        }
        self.navigator.set_page_changed()
    }

    fn cancel_range(&mut self) {
        let navigator = &mut self.navigator;
        navigator.cancel_range();
        self.navigator.set_page_changed()
    }

    fn delete_selected_pictures(&mut self) {
        for index in self.navigator.selection() {
            match self.repository.delete_picture_at_index(index) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }

    fn move_selected_pictures_to_target(&mut self, target_dir: &str) {
        let mut picture_count = 0;
        let mut operation_count = 0;
        for index in self.navigator.selection() {
            match self.repository.move_picture_at_index(index, target_dir) {
                Ok(count) => {
                    picture_count += 1;
                    operation_count += count;
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        println!(
            "{} pictures moved to {}\n{} operations\nexiting gsr",
            picture_count, target_dir, operation_count
        );
        let _ = self.reload();
        self.navigator.set_page_changed();
    }
    fn move_selected_pictures(&mut self) {
        if let Some(target_dir) = &self.command_line_arguments.clone().r#move {
            self.move_selected_pictures_to_target(target_dir);
        }
    }
    fn cancel_delete_picture(&mut self) {
        let navigator = &mut self.navigator;
        navigator.cancel_range();
        self.navigator.set_page_changed()
    }

    fn confirm_delete_picture(&mut self) {
        self.delete_selected_pictures();
        let _ = self.reload();
        self.navigator.set_page_changed()
    }

    fn confirm_move_picture(&mut self) {
        self.move_selected_pictures()
    }

    fn confirm_move_picture_to_label(&mut self, directory: &str) {
        self.move_selected_pictures_to_target(directory);
    }

    fn cancel_move_picture(&mut self) {
        let navigator = &mut self.navigator;
        navigator.cancel_range();
        self.navigator.set_page_changed()
    }

    fn copy_to_temp(&mut self) {
        match self
            .repository
            .copy_picture_at_index_to_temp_dir(self.navigator.position())
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    fn extract_filenames(&mut self) {
        if self.navigator.has_selected() {
            let _ = self
                .repository
                .extract_file_names(&self.navigator.selection());
        }
    }

    fn delete_picture(&mut self) {
        if self.navigator.has_selected() {
            self.editor
                .begin(&self.main_window(), EntryKind::DeleteConfirmation, None);
            self.state.set_mode(Mode::Editing);
        }
    }

    fn move_picture(&mut self) {
        if let Some(target_dir) = &self.command_line_arguments.r#move {
            self.editor
                .begin(&self.main_window(), EntryKind::MoveConfirmation, None);
            self.editor
                .set_prompt(&format!("move these pictures to {} ?", target_dir));
            self.state.set_mode(Mode::Editing);
        }
    }
    fn check_move_destination_label(&self) -> Option<String> {
        let mut label: Option<String> = None;
        let mut grand_parent: Option<String> = None;

        if self.navigator.has_selected() {
            for index in 0..self.navigator.limit() {
                if self.navigator.is_selected(index) {
                    let picture = self.repository.picture_at(index);
                    let this_label = picture.label();
                    if let Some(directory) = grand_parent_directory(&picture.file_path()) {
                        if label.is_none() {
                            label = Some(this_label);
                            grand_parent = Some(directory);
                        } else if this_label != label.clone().unwrap()
                            || directory != grand_parent.clone().unwrap()
                        {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
            let path = PathBuf::from(grand_parent.unwrap());
            let addendum = PathBuf::from(label.clone().unwrap());
            let candidate = path.join(addendum.clone());
            let result = check_path_exists(&candidate);
            match result {
                Ok(valid_path) => Some(valid_path.to_str().unwrap().to_string()),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    fn move_picture_to_label(&mut self) {
        if let Some(target_dir) = self.check_move_destination_label() {
            self.editor.begin(
                &self.main_window(),
                EntryKind::MoveToLabelConfirmation(target_dir),
                None,
            );
            self.state.set_mode(Mode::Editing);
        }
    }

    fn arrow_move(&mut self, direction: Direction) {
        if self.state().single_view() && self.state().full_size_on() {
            self.full_size_arrow_move(direction)
        } else {
            let navigator = &mut (self.navigator);
            if navigator.can_move(direction.clone()) {
                navigator.move_towards(direction)
            }
        }
    }

    fn full_size_arrow_move(&self, direction: Direction) {
        self.main_window().full_size_arrow_move(direction.clone())
    }

    fn can_move(&mut self, direction: Direction) -> bool {
        !self.state.full_size_on() && self.navigator.can_move(direction)
    }

    fn move_towards(&mut self, direction: Direction) {
        match direction {
            Direction::NextPage if self.state.single_view() => self.move_towards(Direction::Right),
            Direction::PrevPage if self.state.single_view() => self.move_towards(Direction::Left),
            ref other => {
                if self.can_move(other.clone()) {
                    self.navigator.move_towards(other.clone());
                }
            }
        }
    }

    fn repeat_last_action(&mut self) {
        let action = self.last_action.clone();
        match action {
            Action::Nothing => {}
            Action::Label(label) => self.label_selected_pictures(&label),
            Action::Categorize(category) => self.categorize_selected_pictures(category),
            Action::Unlabel => self.unlabel_selected_pictures(),
            Action::AddTag(label) => self.tag_selected_pictures(&label),
            Action::RemoveTag(label) => self.untag_selected_pictures(&label),
            Action::Rank(rank) => self.rank_selected_pictures(rank),
        }
    }

    pub fn increment_picture_score(&mut self, file_path: &str) {
        if let Some(score) = self.state().scores().get_mut(file_path) {
            *score += 1;
        } else {
            _ = self.state().scores_mut().insert(file_path.to_string(), 1);
        };
    }

    fn confirm_view(&mut self, input: &str) {
        if !input.is_empty() {
            match input {
                "1" | "2" | "3" | "4" | "5" => self.change_grid_size(input.parse().unwrap()),
                "Thumbs" => self.change_grid_size(10),
                "Covers" => self.toggle_cover_selection(),
                "Date" => self.process_control(&Control::DisplayDate),
                "Path" => self.process_control(&Control::DisplayPath),
                "Size" => self.process_control(&Control::DisplaySize),
                _ => {
                    eprintln!("not implemented");
                }
            }
        }
    }

    fn confirm_rank(&mut self, input: &str) {
        if !input.is_empty() {
            match input.parse() {
                Ok(level) => match level {
                    0 => self.rank_selected_pictures(Rank::NoStar),
                    1 => self.rank_selected_pictures(Rank::OneStar),
                    2 => self.rank_selected_pictures(Rank::TwoStars),
                    3 => self.rank_selected_pictures(Rank::ThreeStars),
                    _ => {}
                },
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }

    fn find_first(&mut self, pattern: &str, find: Find) {
        self.apply_search(pattern, find)
    }

    fn select(&mut self, pattern: &str, find: Find) {
        let information_opt = match predicate(pattern, find, self.repository.catalog().clone()) {
            Ok(predicate) => {
                let selection = format!("{:?} {}", find, pattern);
                self.go_to_selection(&selection, predicate)
            }
            Err(e) => {
                eprintln!("error in select: {}", e);
                None
            }
        };
        if let Some(information) = information_opt {
            display_information(&self.main_window().application_window(), &information)
        }
    }

    fn apply_search(&mut self, pattern: &str, find: Find) {
        let information_opt = match predicate(pattern, find, self.repository.catalog().clone()) {
            Ok(predicate) => {
                if let Ok(mut gallery) = self.repository.gallery_rc().try_borrow_mut() {
                    let finder = &mut gallery.finder;
                    if let Some(index) = finder.find_first(predicate) {
                        let navigator = &mut self.navigator;
                        navigator.move_towards(Direction::Index { value: index });
                        navigator.set_page_changed();
                        self.state.set_search_in_progress(true);
                        None
                    } else {
                        Some(format!("{} [{}] not found", find, pattern))
                    }
                } else {
                    panic!("can't borrow");
                }
            }
            Err(e) => Some(format!("{}", e)),
        };
        if let Some(information) = information_opt {
            display_information(&self.main_window().application_window(), &information)
        }
    }

    fn find_next(&mut self) {
        let information_opt = if let Ok(mut gallery) = self.repository.gallery_rc().try_borrow_mut()
        {
            if let Some(index) = gallery.finder.find_next() {
                let navigator = &mut self.navigator;
                navigator.move_towards(Direction::Index { value: index });
                navigator.set_page_changed();
                None
            } else {
                self.state.set_search_in_progress(false);
                Some("end of search")
            }
        } else {
            panic!("can't borrow");
        };
        if let Some(information) = information_opt {
            display_information(&self.main_window().application_window(), information)
        }
    }

    fn test(&mut self) {
        println!("test");
        let application_window = self.main_window().application_window();

        // first create a view with the app_window, a prompt, an initial entry
        let entry_view =
            EntryView::new_with(&application_window, &entry_prompt(EntryKind::FindLabel), "");

        // create a refcell to it for the editor to have
        let entry_view_rc = RefCell::new(entry_view);

        // create a validator for this editor to have
        let validator = Validator::new(EntryKind::FindLabel);

        // create the editor managing the control between view and rest of the app
        let entry_editor = EntryEditor::new_with(
            entry_view_rc.clone(),
            validator,
            CompletionDispenser::new_with(tags_from_str("foo,fog,bar,qux,law")),
        );
        let entry_editor_rc = RefCell::new(entry_editor);

        // when view receives a key, it sends a signal to its editor
        entry_view_rc
            .borrow()
            .attach_key_pressed_editor(&entry_editor_rc, true);

        // when the editor is sent a key signal it reacts
        // maybe closing if Escape whas pressed
        entry_editor_rc
            .borrow()
            .connect_key_pressed(|editor, key_name| editor.edit_entry(key_name));

        // when the editor is sent a close signal it does things, mainly closing its view
        entry_editor_rc.borrow().connect_closed(|editor| {
            if let Some(view) = editor.view() {
                view.set_input("#");
                view.close()
            }
        });
        self.state.set_mode(Mode::Editing);
        entry_view_rc.borrow().present();
    }
}
