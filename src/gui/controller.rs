use crate::cli::command::Command;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::Configuration;
use crate::file::paths::check_path_exists;
use crate::file::paths::grand_parent_directory;
use crate::file::paths::parent_directory;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::control::{Control, Controls, default_controls, help_on_controls};
use crate::gui::direction::Direction;
use crate::gui::editor::legacy_editor::LegacyEditor;
use crate::gui::enter_label::enter_label;
use crate::gui::entry_kind::EntryKind;
use crate::gui::event::Event;
use crate::gui::key_input::KeyInput;
use crate::gui::key_input::entry::label_change_entry;
use crate::gui::key_input::information;
use crate::gui::key_input::information::information_key_input;
use crate::gui::key_input::menu::change_menu;
use crate::gui::key_input::menu::order_menu;
use crate::gui::key_input::menu::view_menu;
use crate::gui::main_controller::RcMainController;
use crate::gui::mode::Mode;
use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use crate::gui::selector::Selector;
use crate::gui::state::State;
use crate::gui::view::main_view::{LEFT_PANE, MainView};
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
use crate::model::view_option::ViewOption;
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

#[derive(Debug)]
pub struct Controller {
    configuration_rc: RefCell<Configuration>,
    repository: Repository,
    command_line_arguments_rc: RefCell<CommandLineArguments>,
    navigator_rc: RefCell<Navigator>,
    controls: Controls,
    state_rc: RefCell<State>,
    main_view_opt_rc: RefCell<Option<MainView>>,
    gsr_entry_window_opt_rc: RefCell<Option<GsrEntryWindow>>,
    legacy_editor_rc: RefCell<LegacyEditor>,
    selector_rc: RefCell<Selector>,
    last_action_rc: RefCell<Action>,
    main_controller_rc_opt: Option<RcMainController>,
}

pub type RcController = Rc<RefCell<Controller>>;

impl Controller {
    pub fn new(
        config: Configuration,
        command_line_arguments: CommandLineArguments,
    ) -> IOResult<Self> {
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
        let repository = Repository::new(config.clone(), cli.clone(), false);
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
        let controller = Self {
            configuration_rc: RefCell::new(config.clone()),
            repository: repository.clone(),
            command_line_arguments_rc: RefCell::new(cli.clone()),
            legacy_editor_rc: RefCell::new(LegacyEditor::new()),
            selector_rc: RefCell::new(Selector::new(&catalog)),
            navigator_rc: RefCell::new(Navigator::new(repository.len(), pictures_per_row as usize)),
            controls: default_controls(),
            state_rc: RefCell::new(State::new(
                pictures_per_row as usize,
                cli.slideshow().is_some(),
            )),
            main_view_opt_rc: RefCell::new(None),
            gsr_entry_window_opt_rc: RefCell::new(None),
            last_action_rc: RefCell::new(Action::Nothing),
            main_controller_rc_opt: None,
        };
        Ok(controller)
    }

    pub fn set_main_controller_rc(&mut self, main_controller_rc: RcMainController) {
        self.main_controller_rc_opt = Some(main_controller_rc);
    }
    // BAR
    pub fn process_action(
        &self,
        simple_action: &gtk::gio::SimpleAction,
        variant_opt: Option<&gtk::glib::Variant>,
    ) {
        let old_slideshow_on = self.state().slideshow_on();
        let gio_action = GioAction::from((simple_action, variant_opt));
        let action = Action::from(gio_action.clone());
        dbg!(&gio_action, &action);
        match action {
            Action::ApplyOrderSetting(order_setting) => self.apply_order_setting(order_setting),
            Action::ApplyViewSetting(view_option) => self.apply_view_setting(view_option),
            Action::Cancel => self.cancel_edition(),
            Action::Dismiss => self.dismiss(),
            Action::EnterLabel => self.enter_label(),
            Action::FocusAt(col, row) => {
                self.process_event(Event::PictureClicked {
                    button: 1,
                    col,
                    row,
                });
            }
            Action::GotoDirectory => self.go_to_directory(),
            Action::Label(label) => self.confirm_label(&label),
            Action::MoveTowards(Direction::Down) => self.arrow_move(Direction::Down),
            Action::MoveTowards(Direction::First) => self.move_towards(Direction::First),
            Action::MoveTowards(Direction::Last) => self.move_towards(Direction::Last),
            Action::MoveTowards(Direction::Left) => {
                self.arrow_move(Direction::Left);
            }
            Action::MoveTowards(Direction::NextPage) => {
                self.move_next();
            }
            Action::MoveTowards(Direction::PageEnd) => self.move_towards(Direction::PageEnd),
            Action::MoveTowards(Direction::PageStart) => self.move_towards(Direction::PageStart),
            Action::MoveTowards(Direction::PrevPage) => {
                self.move_towards(Direction::PrevPage);
            }
            Action::MoveTowards(Direction::Right) => {
                self.arrow_move(Direction::Right);
            }
            Action::MoveTowards(Direction::Up) => self.arrow_move(Direction::Up),
            Action::Nothing => {}
            Action::PickChange => self.pick_change(),
            Action::PickOrderSetting => self.pick_order_setting(),
            Action::PickViewOption => self.pick_view_option(),
            Action::Quit => self.quit(),
            Action::QuitDirectory => self.back_from_directory(),
            Action::Rank(rank) => self.rank_selected_pictures(rank),
            Action::TogglePalette => self.toggle_palette(),
            Action::ToggleSelectedAt(col, row) => {
                self.process_event(Event::PictureClicked {
                    button: 3,
                    col,
                    row,
                });
                let mut navigator = self.navigator_rc.borrow_mut();
                navigator.set_page_changed();
            }
            Action::ToggleSingleView => self.toggle_single_view(),
            Action::ToggleThumbnailsView => self.toggle_thumbview(),
            Action::ToggleTwoByTwoView => self.toggle_2x2_view(),
            _ => {
                dbg!("action not yet recognized by controller. todo");
            }
        }
        if self.state().slideshow_on() == old_slideshow_on {
            self.main_view().set_focus_for_current_picture(self);
            let main_view = self.main_view();
            {
                self.set_slideshow_off();
                if self.state().single_view() != self.main_view().single_view() {
                    main_view.toggle_view_stack(self);
                };
            }
            {
                let page_changed = self.navigator_rc.borrow().page_changed();

                if page_changed {
                    self.main_view().set_pictures(self);
                    {
                        let binding = &self.navigator_rc;
                        let mut navigator = binding.borrow_mut();
                        navigator.set_page_unchanged();
                    }
                }
                self.main_view().set_focus_for_current_picture(self);
                self.main_view().set_title(self);
            }
        }
    }

    pub fn last_action(&self) -> Action {
        self.last_action_rc.borrow().clone()
    }
    pub fn set_last_action(&self, action: Action) {
        *self.last_action_rc.borrow_mut() = action
    }

    pub fn command_line_arguments(&self) -> CommandLineArguments {
        self.command_line_arguments_rc.borrow().clone()
    }

    pub fn repository(&self) -> Repository {
        self.repository.clone()
    }
    pub fn selector(&self) -> Selector {
        self.selector_rc.borrow().clone()
    }

    pub fn set_selected(&self, selected: &str) {
        self.selector_rc.borrow_mut().set_selected(selected);
    }
    pub fn main_view(&self) -> MainView {
        let main_view_opt = self.main_view_opt_rc.borrow().clone();
        if let Some(main_view) = main_view_opt {
            main_view.clone()
        } else {
            panic!("main_view is not set");
        }
    }
    pub fn set_main_view(&self, main_view: MainView) {
        *self.main_view_opt_rc.borrow_mut() = Some(main_view)
    }

    pub fn state(&self) -> State {
        self.state_rc.borrow().clone()
    }

    pub fn set_state_mode(&self, mode: Mode) {
        let mut state = self.state_rc.borrow_mut();
        state.set_mode(mode)
    }

    pub fn navigator(&self) -> Navigator {
        self.navigator_rc.borrow().clone()
    }

    pub fn set_navigator(&self, navigator: Navigator) {
        *self.navigator_rc.borrow_mut() = navigator
    }

    pub fn current_picture(&self) -> Picture {
        self.repository.picture_at(self.navigator().position())
    }

    fn load_repository(&self) -> IOResult<usize> {
        println!("loading directory");
        let args = self.command_line_arguments().clone();
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
                self.set_navigator(Navigator::new(
                    self.repository.len(),
                    self.state().pictures_per_row(),
                ));
                Ok(count)
            }
        }
    }

    pub fn process_event(&self, event: Event) {
        match event {
            Event::KeyPressed {
                key,
                key_code,
                modifier_type,
            } => {
                self.process_key_event(key, key_code, modifier_type);
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

    fn set_slideshow_off(&self) {
        let mut state = self.state_rc.borrow_mut();
        if state.slideshow_on() {
            println!("setting slideshow off…");
            state.set_slideshow_off();
        }
    }
    fn process_picture_clicked(&self, button: u32, col: i32, row: i32) {
        self.main_view()
            .set_label_text_for_current_picture(self, None);
        {
            let binding = &self.navigator_rc;
            let mut navigator = binding.borrow_mut();
            if let Some(index) = navigator.position_from_coords(row as usize, col as usize)
                && navigator.can_move(Direction::Index { value: index })
            {
                navigator.move_towards(Direction::Index { value: index });
            } else {
                println!("cannot move");
            }
        }
        if button == 3 {
            self.toggle_selected();
            self.main_view().set_pictures(self);
            self.main_view().set_title(self);
        }
    }

    fn process_picture_double_clicked(&self, button: u32, col: i32, row: i32) {
        self.main_view()
            .set_label_text_for_current_picture(self, None);
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        if let Some(index) = navigator.position_from_coords(row as usize, col as usize)
            && navigator.can_move(Direction::Index { value: index })
        {
            navigator.move_towards(Direction::Index { value: index });
            if button == 1 {
                let main_view = self.main_view();
                main_view.set_label_text_for_current_picture(self, None);
                let old_slideshow_on = self.state().slideshow_on();
                self.process_control(&Control::SetSelectionRange);
                if self.state().slideshow_on() == old_slideshow_on {
                    self.set_slideshow_off();
                    if self.state().single_view() != self.main_view().single_view() {
                        main_view.toggle_view_stack(self);
                    };
                    if navigator.page_changed() {
                        self.main_view().set_pictures(self);
                        navigator.set_page_unchanged();
                    };
                    self.set_label_text_for_current_picture();
                    self.main_view().set_title(self);
                }
            } else if button == 3 {
                self.toggle_selected();
                self.main_view().set_pictures(self);
                self.main_view().set_title(self);
            }
        }
        self.set_label_text_for_current_picture();
    }

    fn process_pane_clicked(&self, _button: usize, pane_number: usize) {
        self.process_control(if pane_number == LEFT_PANE {
            &Control::MovePrev
        } else {
            &Control::MoveNext
        });
        if self.navigator().has_moved() {
            self.main_view().set_pictures(self)
        }
    }

    fn process_key_event(&self, key: Key, _key_code: u32, _modifier_type: ModifierType) {
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        let main_view = self.main_view();
        main_view.set_label_text_for_current_picture(self, None);
        let old_slideshow_on = self.state().slideshow_on();
        self.process_key(key);

        if self.state().slideshow_on() == old_slideshow_on {
            self.set_slideshow_off();
            if self.state().single_view() != self.main_view().single_view() {
                main_view.toggle_view_stack(self);
            };
            if navigator.page_changed() {
                self.main_view().set_pictures(self);
                navigator.set_page_unchanged();
            };
            self.set_label_text_for_current_picture();
            self.main_view().set_title(self);
        }
    }

    pub fn set_label_text_for_current_picture(&self) {
        {
            let mut state = self.state_rc.borrow_mut();
            if state.change_focus_symbol_on() {
                state.toggle_focus_symbol()
            }
        };
        self.main_view()
            .set_label_text_for_current_picture(self, Some(self.state().focus_symbol()))
    }

    fn set_opacity_for_current_picture(&self, opacity: f64) {
        self.main_view()
            .set_opacity_for_current_picture(self, opacity)
    }

    fn process_key(&self, key: Key) {
        const SHIFT_L: &str = "Shift_L";
        const SHIFT_R: &str = "Shift_R";
        if let Some(name) = key.name()
            && (name == SHIFT_L || name == SHIFT_R)
        {
            return;
        }
        let controls = self.controls.clone();
        let binding = self.selector_rc.clone();
        let mut selector = binding.borrow_mut();
        match self.state().mode() {
            Mode::MovingToCategory => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        self.move_sub_category_to_category(
                            &selector.prev_selected(),
                            &selector.selected(),
                        )
                    }
                }
            }
            Mode::MovingCategory => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        self.move_to_category(&selector.selected())
                    }
                }
            }
            Mode::AddingCategory => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        self.add_category(
                            &(self.legacy_editor_rc.borrow()).input(),
                            &selector.selected(),
                        )
                    }
                }
            }
            Mode::RemovingCategory => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        self.remove_category(&selector.selected())
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
                self.set_state_mode(Mode::View)
            }
            Mode::Categorizing => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        let category: Category = category_from_string(&selector.selected());
                        self.categorize_selected_pictures(category)
                    }
                    self.set_opacity_for_current_picture(1.00);
                }
            }
            Mode::SelectingCategory => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        self.find_first(
                            &(self.legacy_editor_rc.borrow()).input(),
                            Find::SubCategory,
                        );
                    }
                }
            }
            Mode::FindingSubCategory => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        let category_name = selector.selected();
                        self.find_first(&category_name, Find::SubCategory)
                    }
                }
            }
            Mode::SelectingSubCategory => {
                selector.process(key);
                if !selector.selecting() {
                    self.set_state_mode(Mode::View);
                    if !selector.selected().is_empty() {
                        let category_name = selector.selected();
                        self.select(&category_name, Find::SubCategory)
                    }
                }
            }
            Mode::Editing => {
                (self.legacy_editor_rc.borrow_mut()).process(key);
                if !(self.legacy_editor_rc.borrow()).editing() {
                    self.set_state_mode(Mode::View);
                    match (self.legacy_editor_rc.borrow()).entry_kind() {
                        EntryKind::AddCategory => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.adding_category(&(self.legacy_editor_rc.borrow()).input())
                            }
                        }
                        EntryKind::MoveCategory => {}
                        EntryKind::RemoveCategory => {}
                        EntryKind::Catalog => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                match Change::from_str(&(self.legacy_editor_rc.borrow()).input()) {
                                    Ok(Change::AddCategory) => self.enter_add_category(),
                                    Ok(Change::MoveCategory) => self.enter_move_category(),
                                    Ok(Change::RemoveCategory) => self.enter_remove_category(),
                                    _ => {}
                                }
                            };
                        }
                        EntryKind::Change => {
                            println!(
                                "(self.legacy_editor_rc.borrow()).input(): {}",
                                (self.legacy_editor_rc.borrow()).input()
                            );
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                match Change::from_str(&(self.legacy_editor_rc.borrow()).input()) {
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
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.rename_selected_picture(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                )
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::Categorize => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.categorize_selected_pictures(Some(
                                    (self.legacy_editor_rc.borrow()).input(),
                                ))
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::Select => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                match Find::from_str(&(self.legacy_editor_rc.borrow()).input()) {
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
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                match Find::from_str(&(self.legacy_editor_rc.borrow()).input()) {
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
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.label_selected_pictures(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                )
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::AddTag => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.tag_selected_pictures(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                )
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::RemoveTag => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.untag_selected_pictures(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                )
                            };
                            self.set_opacity_for_current_picture(1.00);
                        }
                        EntryKind::Number => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.move_towards_index(
                                    (self.legacy_editor_rc.borrow()).input().parse().unwrap(),
                                )
                            };
                        }
                        EntryKind::Order => {
                            let input = self.legacy_editor_rc.borrow().input();
                            self.set_order(&input);
                        }
                        EntryKind::Rank => {
                            self.confirm_rank(&(self.legacy_editor_rc.borrow()).input())
                        }
                        EntryKind::View => {
                            self.confirm_view(&(self.legacy_editor_rc.borrow()).input())
                        }
                        EntryKind::DeleteConfirmation => {
                            if &(self.legacy_editor_rc.borrow()).input() == "yes" {
                                self.confirm_delete_picture()
                            } else {
                                self.cancel_delete_picture()
                            }
                        }
                        EntryKind::MoveConfirmation => {
                            if &(self.legacy_editor_rc.borrow()).input() == "yes" {
                                self.confirm_move_picture()
                            } else {
                                self.cancel_move_picture()
                            }
                        }
                        EntryKind::MoveToLabelConfirmation(ref target) => {
                            if &(self.legacy_editor_rc.borrow()).input() == "yes" {
                                self.confirm_move_picture_to_label(target)
                            } else {
                                self.cancel_move_picture()
                            }
                        }
                        EntryKind::FindName => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.find_first(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::Name,
                                );
                            };
                        }
                        EntryKind::FindLabel => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.find_first(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::Label,
                                );
                            };
                        }
                        EntryKind::FindCategory => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.find_first(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::Category,
                                );
                            };
                        }
                        EntryKind::FindSubCategory => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.find_first(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::SubCategory,
                                );
                            };
                        }
                        EntryKind::FindSomeTags => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.find_first(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::AllTags,
                                )
                            }
                        }
                        EntryKind::FindAllTags => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.find_first(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::AllTags,
                                )
                            };
                        }
                        EntryKind::SelectName => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.select(&(self.legacy_editor_rc.borrow()).input(), Find::Name);
                            };
                        }
                        EntryKind::SelectLabel => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.select(&(self.legacy_editor_rc.borrow()).input(), Find::Label);
                            };
                        }
                        EntryKind::SelectCategory => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.select(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::Category,
                                );
                            };
                        }
                        EntryKind::SelectSubCategory => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.select(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::SubCategory,
                                );
                            };
                        }
                        EntryKind::SelectSomeTags => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.select(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::AllTags,
                                )
                            }
                        }
                        EntryKind::SelectAllTags => {
                            if !(self.legacy_editor_rc.borrow()).input().is_empty() {
                                self.find_first(
                                    &(self.legacy_editor_rc.borrow()).input(),
                                    Find::AllTags,
                                )
                            };
                        }
                        EntryKind::Information => {}
                        EntryKind::Help => {}
                    }
                }
            }
        }
    }

    fn set_order(&self, input: &str) {
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

    fn rename_selected_picture(&self, name: &str) {
        for index in self.navigator().selection() {
            match self.repository.rename_picture_at_index(index, name) {
                Ok(count) => {
                    println!("{} picture renamed", count);
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        match self
            .repository
            .initialize_for_args(&self.command_line_arguments(), None)
        {
            Ok(()) => {
                let _ = self.reload();
                self.navigator().set_page_changed();
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    fn label_picture_at_index(&self, index: usize, label: &str) {
        let mut picture = self.repository.picture_at(index);
        picture.set_label(label);
        self.repository.set_picture_at(index, &picture);
        self.set_last_action(Action::Label(label.to_string()));
    }

    fn label_selected_pictures(&self, label: &str) {
        self.repository.add_label(label);
        let mut navigator = self.navigator_rc.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
                    self.label_picture_at_index(index, label);
                }
            }
            navigator.unselect_all();
        } else {
            self.label_picture_at_index(navigator.position(), label)
        };
        navigator.set_page_changed()
    }

    fn unlabel_selected_pictures(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
                    self.label_picture_at_index(index, "");
                }
            }
            navigator.unselect_all();
        } else {
            self.label_picture_at_index(navigator.position(), "")
        };
        navigator.set_page_changed();
        self.set_last_action(Action::Unlabel);
    }

    fn tag_picture_at_index(&self, index: usize, input: &str) {
        let labels: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
        let mut picture = self.repository.picture_at(index);
        labels.iter().for_each(|label| {
            self.repository.add_label(label);
            picture.add_tag(label);
        });
        self.repository.set_picture_at(index, &picture);
    }

    fn untag_picture_at_index(&self, index: usize, label: &str) {
        let mut picture = self.repository.picture_at(index);
        picture.remove_tag(label);
        self.repository.set_picture_at(index, &picture);
    }

    fn tag_selected_pictures(&self, labels: &str) {
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
                    self.tag_picture_at_index(index, labels);
                }
            }
            navigator.unselect_all();
        } else {
            self.tag_picture_at_index(navigator.position(), labels)
        };
        navigator.set_page_changed();
        self.set_last_action(Action::AddTag(labels.to_string()));
    }

    fn untag_selected_pictures(&self, label: &str) {
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
                    self.untag_picture_at_index(index, label);
                }
            }
            navigator.unselect_all();
        } else {
            self.untag_picture_at_index(navigator.position(), label)
        };
        navigator.set_page_changed();
        self.set_last_action(Action::RemoveTag(label.to_string()));
    }

    fn move_next(&self) {
        if self.state().search_in_progress() {
            self.find_next()
        } else {
            self.move_towards(Direction::NextPage)
        };
    }
    fn move_towards_index(&self, index: usize) {
        let mut navigator = self.navigator_rc.borrow_mut();
        let direction = Direction::Index { value: index };
        if navigator.can_move(direction.clone()) {
            navigator.move_towards(direction)
        }
    }

    fn set_setting(&self, setting: &Control, choice: &Control) {
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

    fn setting_display(&self) {
        println!("Setting display…");
        self.set_state_mode(Mode::Setting(Control::SetDisplay));
    }

    fn enter_change(&self) {
        self.enter_editing(EntryKind::Change, None);
    }

    fn enter_change_catalog(&self) {
        self.enter_editing(EntryKind::Catalog, None)
    }

    fn enter_add_category(&self) {
        self.enter_editing(EntryKind::AddCategory, None)
    }

    fn adding_category(&self, category_name: &str) {
        if self.main_controller_rc_opt.is_none() {
            panic!("main_controller not set");
        }
        let mut selector = self.selector_rc.borrow_mut();
        if !self.repository.catalog().contains(category_name) {
            selector.begin(
                &self.main_view(),
                &format!("select a category to add {} to ", category_name),
                &self.repository.catalog(),
            );
            self.set_state_mode(Mode::AddingCategory);
        } else {
            self.display_information(&format!("the category {} already exists", category_name));
        }
    }
    fn enter_move_category(&self) {
        let mut selector = self.selector_rc.borrow_mut();
        selector.begin(
            &self.main_view(),
            "select the category to move",
            &self.repository.catalog(),
        );
        self.set_state_mode(Mode::MovingCategory);
    }

    fn move_to_category(&self, moving_category_name: &str) {
        let mut selector = self.selector_rc.borrow_mut();
        let mut pruned_catalog = self.repository.catalog();
        let _ = pruned_catalog.remove_category(moving_category_name, true);
        selector.set_prev_selected(moving_category_name);
        selector.begin(
            &self.main_view(),
            &format!("select the category where to move {}", moving_category_name),
            &pruned_catalog,
        );
        self.set_state_mode(Mode::MovingToCategory);
    }

    fn enter_remove_category(&self) {
        let mut selector = self.selector_rc.borrow_mut();
        selector.begin(
            &self.main_view(),
            "select a category to remove",
            &self.repository.catalog(),
        );
        self.set_state_mode(Mode::RemovingCategory);
    }

    fn enter_find(&self) {
        self.enter_editing(EntryKind::Find, None)
    }

    fn enter_select(&self) {
        if !self.state().has_saved_command_line_arguments() {
            self.enter_editing(EntryKind::Select, None)
        } else {
            self.display_information("selection not allowed while in a directory or a selection");
        }
    }

    fn enter_editing(&self, entry_kind: EntryKind, choice_opt: Option<Tags>) {
        let mut legacy_editor = self.legacy_editor_rc.borrow_mut();
        legacy_editor.begin(&self.main_view(), entry_kind, choice_opt);
        let mut state = self.state_rc.borrow_mut();
        state.set_mode(Mode::Editing);
    }

    fn enter_find_label(&self) {
        self.enter_editing(EntryKind::FindLabel, None)
    }

    fn enter_find_name(&self) {
        self.enter_editing(EntryKind::FindName, None)
    }

    fn enter_find_category(&self) {
        self.enter_editing(EntryKind::FindCategory, None);
    }

    fn enter_find_tags(&self, all_match: bool) {
        let kind = if all_match {
            EntryKind::FindAllTags
        } else {
            EntryKind::FindSomeTags
        };
        self.enter_editing(kind, None);
    }

    fn enter_find_sub_category(&self) {
        self.set_category_selection();
        self.set_state_mode(Mode::FindingSubCategory);
    }

    fn enter_select_label(&self) {
        self.enter_editing(EntryKind::SelectLabel, None);
    }

    fn enter_select_name(&self) {
        self.enter_editing(EntryKind::SelectName, None);
    }

    fn enter_select_category(&self) {
        self.enter_editing(EntryKind::SelectCategory, None);
    }

    fn enter_select_tags(&self, all_match: bool) {
        let kind = if all_match {
            EntryKind::SelectAllTags
        } else {
            EntryKind::SelectSomeTags
        };
        self.enter_editing(kind, None);
    }
    fn enter_select_sub_category(&self) {
        self.set_category_selection();
        self.set_state_mode(Mode::SelectingSubCategory);
    }
    fn setting_mark(&self) {
        println!("Setting mark…");
        self.set_state_mode(Mode::Setting(Control::SetMark));
    }

    fn jumping_mark(&self) {
        println!("Jumping to mark…");
        self.set_state_mode(Mode::Setting(Control::GotoMark));
    }

    fn set_mark(&self, mark: char) {
        let mut configuration = self.configuration_rc.borrow_mut();
        let file_path = self.current_picture().file_path();
        let _ = configuration.marked.insert(mark, file_path.clone());
        println!("{}={}", mark, file_path);
        let _ = configuration.save();
    }
    fn setting_order(&self) {
        self.enter_editing(EntryKind::Order, None);
    }

    fn next_slide_delay(&self) {
        self.move_towards(Direction::NextPage);
        self.main_view().set_pictures(self)
    }

    fn process_control(&self, control: &Control) {
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
                value: rng().random_range(0..self.navigator().limit()),
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
                if self.navigator().has_selected() && self.navigator().selected_picture_count() == 1
                {
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

    fn go_to_directory(&self) {
        // don't go if we are not in cover views
        if !self.command_line_arguments().cover {
            self.display_information("cannot go to a directory when not in cover view");
            return;
        }
        // don't go if we are in single view mode
        if self.state().single_view() {
            self.display_information("cannot go to a directory when in single view");
            return;
        };
        // don't go if we are already in the directory
        if let Some(directory) = parent_directory(&self.current_picture().file_path())
            && Some(directory.clone()) == self.command_line_arguments().directory
        {
            eprintln!("cannot go to this directory: {}", directory);
            return;
        };
        match parent_directory(&self.current_picture().file_path()) {
            // don't go if we cannot find a parent directory for the filepath of current picture
            None => {
                eprintln!(
                    "cannot find parent directory for {}",
                    self.current_picture().file_path()
                );
                return;
            }
            Some(directory) => {
                {
                    let mut command_line_arguments = self.command_line_arguments_rc.borrow_mut();
                    command_line_arguments.index = Some(self.navigator().position());
                    let clargs = command_line_arguments.clone();
                    {
                        let mut state = self.state_rc.borrow_mut();
                        state.push_saved_command_line_arguments(clargs.clone(), &directory);
                    }
                    let new_clargs = CommandLineArguments {
                        directory: Some(directory),
                        cover: false,
                        ..clargs.clone()
                    };
                    *command_line_arguments = new_clargs.clone();
                }

                let binding = self.navigator_rc.clone();
                let mut navigator = binding.borrow_mut();
                match self
                    .repository
                    .initialize_for_args(&self.command_line_arguments(), None)
                {
                    Ok(()) => {
                        let _ = self.reload();
                        navigator.set_page_changed();
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }

    fn go_to_selection(&self, selection: &str, predicate: Predicate) -> Option<String> {
        let mut command_line_arguments = self.command_line_arguments_rc.borrow_mut();
        command_line_arguments.index = Some(self.navigator().position());
        let clargs = command_line_arguments.clone();
        self.state()
            .push_saved_command_line_arguments(clargs.clone(), selection);
        let new_clargs = CommandLineArguments {
            directory: None,
            cover: false,
            ..clargs.clone()
        };
        *command_line_arguments = new_clargs.clone();
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        match self
            .repository
            .initialize_for_args(&new_clargs, Some(predicate))
        {
            Ok(()) => match self.reload() {
                Ok(0) => {
                    self.back_from_directory();
                    navigator.set_page_changed();
                    Some(format!("no picture found with: {}", selection))
                }
                Ok(_) => {
                    navigator.set_page_changed();
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

    fn back_from_directory(&self) {
        // don't go if not in directory currently
        if self.state().pop_saved_command_line_arguments().is_none() {
            let application_window = &self.main_view().application_window();
            self.display_information("not in a directory currently");
            return;
        };
        let (old_pictures_per_row, old_clargs) = {
            let mut state = self.state_rc.borrow_mut();
            state.pop_saved_command_line_arguments().unwrap()
        };
        {
            let mut command_line_arguments = self.command_line_arguments_rc.borrow_mut();
            *command_line_arguments = old_clargs.clone();
        }
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        match self.repository.initialize_for_args(&old_clargs, None) {
            Ok(()) => {
                self.change_grid_size(old_pictures_per_row);
                let _ = self.reload();
                if let Some(index) = self.command_line_arguments().index
                    && navigator.can_move(Direction::Index { value: index })
                {
                    navigator.move_towards(Direction::Index { value: index })
                };
                navigator.set_page_changed()
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    fn toggle_single_view(&self) {
        {
            let mut state = self.state_rc.borrow_mut();
            state.toggle_single_view();
            if state.full_size_on() {
                state.toggle_full_size()
            }
        }
        self.change_grid_size(self.state().pictures_per_row());
    }

    fn toggle_thumbview(&self) {
        {
            let mut state = self.state_rc.borrow_mut();
            if state.pictures_per_row() != 10 {
                state.change_grid_size(10)
            } else {
                state.toggle_back_grid_size()
            };
        }
        self.change_grid_size(self.state().pictures_per_row());
    }

    fn toggle_grid_size(&self, size: usize) {
        {
            let mut state = self.state_rc.borrow_mut();
            if state.pictures_per_row() != size {
                state.change_grid_size(size)
            } else {
                state.toggle_back_grid_size()
            };
        }
        self.change_grid_size(self.state().pictures_per_row());
    }
    fn toggle_2x2_view(&self) {
        {
            let mut state = self.state_rc.borrow_mut();
            if state.pictures_per_row() != 2 {
                state.change_grid_size(2)
            } else {
                state.toggle_back_grid_size()
            };
        }
        self.change_grid_size(self.state().pictures_per_row());
    }

    fn toggle_3x3_view(&self) {
        {
            let mut state = self.state_rc.borrow_mut();
            if state.pictures_per_row() != 3 {
                state.change_grid_size(3)
            } else {
                state.toggle_back_grid_size()
            };
        }
        self.change_grid_size(self.state().pictures_per_row());
    }

    fn toggle_cover(&self) {
        let index = self.navigator().position();
        let counts = self.repository.directory_count_at_index(index);
        let mut picture = self.repository.picture_at(index);
        picture.toggle_cover(counts.0);
        self.repository.set_picture_at(index, &picture);
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.set_page_changed()
    }

    fn rank_picture_at_index(&self, index: usize, rank: Rank) {
        let mut picture = self.repository.picture_at(index);
        picture.set_rank(rank);
        self.repository.set_picture_at(index, &picture);
    }

    fn categorize_picture_at_index(&self, index: usize, category_opt: Category) {
        let mut picture = self.repository.picture_at(index);
        picture.set_category(category_opt.clone());
        self.repository.set_picture_at(index, &picture);
    }

    fn toggle_cover_selection(&self) {
        println!("toggle cover selection");
        let initial_command_line_arguments = { self.command_line_arguments_rc.borrow().clone() };
        if !self.state().has_saved_command_line_arguments() {
            if !initial_command_line_arguments.cover && self.repository.covers() > 0 {
                {
                    let mut command_line_arguments = self.command_line_arguments_rc.borrow_mut();
                    let new_clargs = CommandLineArguments {
                        cover: true,
                        ..command_line_arguments.clone()
                    };
                    *command_line_arguments = new_clargs;
                }
                {
                    let new_command_line_arguments = self.command_line_arguments_rc.borrow();
                    match self
                        .repository
                        .initialize_for_args(&new_command_line_arguments, None)
                    {
                        Ok(_) => match self.reload() {
                            Ok(0) => {
                                self.navigator().set_page_changed();
                            }
                            Ok(_) => {}
                            Err(e) => panic!("{}", e),
                        },
                        Err(e) => panic!("{}", e),
                    }
                }
            } else if initial_command_line_arguments.cover {
                {
                    let mut command_line_arguments = self.command_line_arguments_rc.borrow_mut();
                    let new_clargs = CommandLineArguments {
                        cover: false,
                        ..command_line_arguments.clone()
                    };
                    *command_line_arguments = new_clargs;
                }
                let new_command_line_arguments = self.command_line_arguments_rc.borrow();
                match self
                    .repository
                    .initialize_for_args(&new_command_line_arguments, None)
                {
                    Ok(_) => match self.reload() {
                        Ok(0) => {
                            self.navigator().set_page_changed();
                        }
                        Ok(_) => {}
                        Err(e) => panic!("{}", e),
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        } else {
            self.display_information("cannot toggle cover selection while in a directory");
        }
    }

    fn cancel_selection_criteria(&self) {
        let current_file_path = self.current_picture().file_path();
        self.repository
            .set_selection_criteria(SelectionCriteria::empty());
        let mut navigator = self.navigator_rc.borrow_mut();
        if let Some(index) = self.repository.find_index_for_file_path(&current_file_path) {
            navigator.move_towards(Direction::Index { value: index })
        } else {
            navigator.move_towards(Direction::First)
        };
        navigator.set_page_changed();
    }

    fn add_category(&self, new_category_name: &str, target_category_name: &str) {
        match self
            .repository
            .add_category(new_category_name, target_category_name)
        {
            Ok(_) => {}
            Err(e) => {
                self.display_information(&format!("{}", e));
            }
        }
    }

    fn move_sub_category_to_category(
        &self,
        moving_category_name: &str,
        target_category_name: &str,
    ) {
        match self
            .repository
            .move_category(moving_category_name, target_category_name)
        {
            Ok(_) => {}
            Err(e) => {
                self.display_information(&format!("{}", e));
            }
        }
    }

    fn remove_category(&self, input: &str) {
        let _ = self.repository.retrieve_all_categories();
        if !self.repository.all_categories().contains(input) {
            match self.repository.remove_category(input) {
                Ok(_) => {}
                Err(e) => {
                    self.display_information(&format!("{}", e));
                }
            }
        } else {
            self.display_information(&format!(
                "category {} is being used and cannot be removed",
                input
            ));
        }
    }

    fn add_tag(&self) {
        self.set_opacity_for_current_picture(0.25);
        self.enter_editing(EntryKind::AddTag, Some(self.repository.all_labels()));
        self.set_state_mode(Mode::Editing);
    }

    fn remove_tag(&self) {
        self.set_opacity_for_current_picture(0.25);
        self.enter_editing(EntryKind::RemoveTag, Some(self.current_picture().tags()));
    }

    fn label(&self) {
        self.set_opacity_for_current_picture(0.25);
        self.enter_editing(EntryKind::Label, Some(self.repository.all_labels()));
    }

    fn label_(&self) {
        let application_window = self.main_view().application_window();
        enter_label(&application_window, &self.repository);
    }

    fn rename(&self) {
        if self.navigator().has_selected() && self.navigator().selected_picture_count() == 1 {
            self.set_opacity_for_current_picture(0.25);
            (self.legacy_editor_rc.borrow_mut()).begin(&self.main_view(), EntryKind::Rename, None);
            self.set_state_mode(Mode::Editing);
        } else {
            (self.legacy_editor_rc.borrow_mut()).begin(
                &self.main_view(),
                EntryKind::Information,
                None,
            );
            (self.legacy_editor_rc.borrow_mut())
                .set_input("Select the picture you want to rename first");
            self.set_state_mode(Mode::Editing);
        }
    }

    fn categorize_selected_pictures(&self, category: Category) {
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
                    self.categorize_picture_at_index(index, category.clone());
                }
            }
            navigator.unselect_all();
        } else {
            self.categorize_picture_at_index(self.navigator().position(), category.clone());
        };
        navigator.set_page_changed();
        self.set_last_action(Action::Categorize(category));
    }

    fn categorize(&self) {
        self.set_opacity_for_current_picture(0.25);
        let mut selector = self.selector_rc.borrow_mut();
        selector.begin(
            &self.main_view(),
            "select a category to apply",
            &self.repository.catalog(),
        );
        self.set_state_mode(Mode::Categorizing);
    }

    fn set_category_selection(&self) {
        let mut selector = self.selector_rc.borrow_mut();
        selector.begin(
            &self.main_view(),
            "select a category to find",
            &self.repository.catalog(),
        );
        self.set_state_mode(Mode::SelectingCategory);
    }

    fn uncategorize_selected_pictures(&self) {
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
                    self.categorize_picture_at_index(index, None);
                }
            }
            navigator.unselect_all();
        } else {
            self.categorize_picture_at_index(navigator.position(), None)
        };
        navigator.set_page_changed();
        self.set_last_action(Action::Unlabel);
    }
    fn rank_selected_pictures(&self, rank: Rank) {
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
                    self.rank_picture_at_index(index, rank);
                }
            }
            navigator.unselect_all();
        } else {
            self.rank_picture_at_index(navigator.position(), rank)
        };
        navigator.set_page_changed();
        self.set_last_action(Action::Rank(rank));
    }

    fn jump(&self) {
        (self.legacy_editor_rc.borrow_mut()).begin(&self.main_view(), EntryKind::Number, None);
        self.set_state_mode(Mode::Editing);
    }

    fn help(&self) {
        self.display_information(&help_on_controls());
    }

    fn information(&self) {
        (self.legacy_editor_rc.borrow_mut()).begin(&self.main_view(), EntryKind::Information, None);
        (self.legacy_editor_rc.borrow_mut())
            .set_input(&self.current_picture().file_path().to_string());
        self.set_state_mode(Mode::Editing);
    }

    fn toggle_display_path(&self) {
        self.state().toggle_display_path();
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.set_page_changed()
    }

    fn find_mark(&self, mark: char) {
        let configuration = self.configuration_rc.borrow();
        if let Some(file_path) = configuration.marked.get(&mark) {
            if let Ok(gallery) = self.repository.gallery_rc().try_borrow() {
                if let Some(index) = gallery
                    .pictures()
                    .iter()
                    .position(|picture| picture.file_path() == *file_path)
                {
                    let mut navigator = self.navigator_rc.borrow_mut();
                    navigator.move_towards(Direction::Index { value: index });
                    navigator.set_page_changed()
                } else {
                    // display_information(&self.main_view().application_window(),&format!("mark: {} not found", mark));
                }
            } else {
                panic!("can't borrow")
            }
        } else {
            self.display_information(&format!("no picture with mark {}", mark));
        }
    }

    pub fn quit(&self) {
        let mut configuration = self.configuration_rc.borrow_mut();
        if self.state().has_saved_command_line_arguments() {
            self.back_from_directory()
        } else {
            configuration.current_picture = Some(self.current_picture().file_path());
            configuration.cover = self.command_line_arguments().cover;
            configuration.current_pictures_per_row = if self.state().single_view() {
                Some(1)
            } else {
                Some(self.state().pictures_per_row())
            };
            configuration.current_order = Some(self.repository.order());
            let _ = configuration.save();
            let application_window = self.main_view().application_window();
            self.repository.update_picture_scores(self.state().scores());
            application_window.close()
        }
    }

    fn reload(&self) -> Result<usize, Error> {
        match self.load_repository() {
            Ok(0) => Ok(0),
            Ok(n) => {
                self.move_towards(Direction::First);
                let mut navigator = self.navigator_rc.borrow_mut();
                navigator.set_page_changed();
                Ok(n)
            }
            Err(e) => {
                eprintln!("{}", e);
                self.quit();
                Err(e)
            }
        }
    }

    fn toggle_expand(&self) {
        if self.state().single_view() {
            self.state().toggle_expand();
            let mut navigator = self.navigator_rc.borrow_mut();
            navigator.set_page_changed();
        }
    }

    fn toggle_display_date(&self) {
        if self.state().display_path_on() {
            self.state().toggle_display_path();
        };
        self.state().toggle_display_date();
        self.main_view().set_title(self);
        println!(
            "display date {}",
            if self.state().display_date_on() {
                String::from("on")
            } else {
                String::from("off")
            }
        );
    }

    fn toggle_display_focus_symbol_change(&self) {
        self.state().toggle_change_focus_symbol()
    }

    fn toggle_display_size(&self) {
        if self.state().display_path_on() {
            self.state().toggle_display_path();
        };
        self.state().toggle_display_size();
        self.main_view().set_title(self);
        println!(
            "display size {}",
            if self.state().display_size_on() {
                String::from("on")
            } else {
                String::from("off")
            }
        );
    }

    fn toggle_full_size(&self) {
        if self.state().single_view() {
            let mut state = self.state_rc.borrow_mut();
            state.toggle_full_size();
            let mut navigator = self.navigator_rc.borrow_mut();
            navigator.set_page_changed();
        }
    }

    fn display_information(&self, message: &str) {
        let application_window = self
            .main_view_opt_rc
            .borrow()
            .as_ref()
            .unwrap()
            .application_window();
        let gsr_entry_window = GsrEntryWindow::new_with(
            &application_window,
            &self.main_controller_rc_opt.as_ref().unwrap(),
            information_key_input(),
            Some(message),
        );
        gsr_entry_window.present();
        *self.gsr_entry_window_opt_rc.borrow_mut() = Some(gsr_entry_window);
    }

    fn toggle_palette(&self) {
        let mut state = self.state_rc.borrow_mut();
        state.toggle_palette();
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.set_page_changed()
    }

    fn toggle_slideshow(&self) {
        if let Some(seconds) = self.command_line_arguments().slideshow() {
            self.state().toggle_slideshow();
            if self.state().slideshow_on() {
                self.main_view().reattach_slideshow_event(seconds);
                let mut navigator = self.navigator_rc.borrow_mut();
                navigator.set_page_changed();
            }
        }
    }

    fn order_by(&self, order: Order) {
        let new_position: Option<usize>;
        let current_file_path = self.current_picture().file_path();
        if let Ok(mut gallery) = self.repository.gallery_rc().try_borrow_mut() {
            gallery.sort_by(order);
            new_position = gallery.find_file_path(&current_file_path);
            self.command_line_arguments().order = Some(order);
        } else {
            panic!("can't borrow mut")
        };
        let mut navigator = self.navigator_rc.borrow_mut();
        if let Some(index) = new_position {
            navigator.move_towards(Direction::Index { value: index })
        } else {
            navigator.move_towards(Direction::First)
        };
        navigator.set_page_changed()
    }

    fn change_grid_size(&self, pictures_per_row: usize) {
        self.state().change_grid_size(pictures_per_row);
        self.apply_grid_size_change();
    }

    fn apply_grid_size_change(&self) {
        let pictures_per_row = self.state().pictures_per_row();
        let palette_on = self.state().palette_on();
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.set_pictures_per_row(pictures_per_row);
        navigator.update_page_limits();
        navigator.set_page_changed();
        let main_controller = self.main_controller_rc_opt.as_ref().unwrap().borrow();
        self.main_view()
            .change_grid_size(pictures_per_row, palette_on, &main_controller);
    }

    fn set_selection_range(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        let position = navigator.position();
        navigator.set_selection_range(position);
        navigator.set_page_changed()
    }

    fn set_selection_range_all(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.set_selection_range_all();
        navigator.set_page_changed()
    }

    fn set_selection_range_page(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.set_selection_range_page();
        navigator.set_page_changed()
    }
    fn repeat_range(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.repeat_range();
        navigator.set_page_changed()
    }

    fn toggle_selected(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        let position = navigator.position();
        if navigator.is_selected(position) {
            navigator.unselect(position)
        } else {
            navigator.select(position)
        }
        navigator.set_page_changed()
    }

    fn cancel_range(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.cancel_range();
        navigator.set_page_changed()
    }

    fn delete_selected_pictures(&self) {
        for index in self.navigator().selection() {
            match self.repository.delete_picture_at_index(index) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }

    fn move_selected_pictures_to_target(&self, target_dir: &str) {
        let mut picture_count = 0;
        let mut operation_count = 0;
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        for index in navigator.selection() {
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
        navigator.set_page_changed();
    }
    fn move_selected_pictures(&self) {
        if let Some(target_dir) = &self.command_line_arguments().clone().r#move {
            self.move_selected_pictures_to_target(target_dir);
        }
    }
    fn cancel_delete_picture(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.cancel_range();
        navigator.set_page_changed()
    }

    fn confirm_delete_picture(&self) {
        let binding = self.navigator_rc.clone();
        let mut navigator = binding.borrow_mut();
        let _ = self.reload();
        navigator.set_page_changed()
    }

    fn confirm_move_picture(&self) {
        self.move_selected_pictures()
    }

    fn confirm_move_picture_to_label(&self, directory: &str) {
        self.move_selected_pictures_to_target(directory);
    }

    fn cancel_move_picture(&self) {
        let mut navigator = self.navigator_rc.borrow_mut();
        navigator.cancel_range();
        navigator.set_page_changed()
    }

    fn copy_to_temp(&self) {
        match self
            .repository
            .copy_picture_at_index_to_temp_dir(self.navigator().position())
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    fn extract_filenames(&self) {
        if self.navigator().has_selected() {
            let _ = self
                .repository
                .extract_file_names(&self.navigator().selection());
        }
    }

    fn delete_picture(&self) {
        if self.navigator().has_selected() {
            (self.legacy_editor_rc.borrow_mut()).begin(
                &self.main_view(),
                EntryKind::DeleteConfirmation,
                None,
            );
            self.set_state_mode(Mode::Editing);
        }
    }

    fn move_picture(&self) {
        if let Some(target_dir) = &self.command_line_arguments().r#move {
            (self.legacy_editor_rc.borrow_mut()).begin(
                &self.main_view(),
                EntryKind::MoveConfirmation,
                None,
            );
            (self.legacy_editor_rc.borrow_mut())
                .set_prompt(&format!("move these pictures to {} ?", target_dir));
            self.set_state_mode(Mode::Editing);
        }
    }
    fn check_move_destination_label(&self) -> Option<String> {
        let mut label: Option<String> = None;
        let mut grand_parent: Option<String> = None;

        let navigator = self.navigator_rc.borrow_mut();
        if navigator.has_selected() {
            for index in 0..navigator.limit() {
                if navigator.is_selected(index) {
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

    fn move_picture_to_label(&self) {
        if let Some(target_dir) = self.check_move_destination_label() {
            (self.legacy_editor_rc.borrow_mut()).begin(
                &self.main_view(),
                EntryKind::MoveToLabelConfirmation(target_dir),
                None,
            );
            self.set_state_mode(Mode::Editing);
        }
    }

    fn arrow_move(&self, direction: Direction) {
        if self.state().single_view() && self.state().full_size_on() {
            self.full_size_arrow_move(direction)
        } else {
            let mut navigator = self.navigator_rc.borrow_mut();
            if navigator.can_move(direction.clone()) {
                navigator.move_towards(direction)
            }
        }
    }

    fn full_size_arrow_move(&self, direction: Direction) {
        self.main_view().full_size_arrow_move(direction.clone())
    }

    fn can_move(&self, direction: Direction) -> bool {
        !self.state().full_size_on() && self.navigator().can_move(direction)
    }

    fn move_towards(&self, direction: Direction) {
        match direction {
            Direction::NextPage if self.state().single_view() => {
                self.move_towards(Direction::Right)
            }
            Direction::PrevPage if self.state().single_view() => self.move_towards(Direction::Left),
            ref other => {
                if self.can_move(other.clone()) {
                    let binding = &self.navigator_rc;
                    let mut navigator = binding.borrow_mut();
                    navigator.move_towards(other.clone());
                }
            }
        }
    }

    fn repeat_last_action(&self) {
        let action = self.last_action().clone();
        if action.is_repeatable() {
            match action {
                Action::Nothing => {}
                Action::Label(label) => self.label_selected_pictures(&label),
                Action::Categorize(category) => self.categorize_selected_pictures(category),
                Action::Unlabel => self.unlabel_selected_pictures(),
                Action::AddTag(label) => self.tag_selected_pictures(&label),
                Action::RemoveTag(label) => self.untag_selected_pictures(&label),
                Action::Rank(rank) => self.rank_selected_pictures(rank),
                _ => {}
            }
        }
    }

    pub fn increment_picture_score(&self, file_path: &str) {
        if let Some(score) = self.state().scores().get_mut(file_path) {
            *score += 1;
        } else {
            _ = self.state().scores_mut().insert(file_path.to_string(), 1);
        };
    }

    fn confirm_view(&self, input: &str) {
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

    fn confirm_rank(&self, input: &str) {
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

    pub fn find_first(&self, pattern: &str, find: Find) {
        self.apply_search(pattern, find)
    }

    fn select(&self, pattern: &str, find: Find) {
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
            self.display_information(&information);
        }
    }

    fn apply_search(&self, pattern: &str, find: Find) {
        let information_opt = match predicate(pattern, find, self.repository.catalog().clone()) {
            Ok(predicate) => {
                if let Ok(mut gallery) = self.repository.gallery_rc().try_borrow_mut() {
                    let finder = &mut gallery.finder;
                    if let Some(index) = finder.find_first(predicate) {
                        let mut navigator = self.navigator_rc.borrow_mut();
                        navigator.move_towards(Direction::Index { value: index });
                        navigator.set_page_changed();
                        self.state().set_search_in_progress(true);
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
            self.display_information(&information);
        }
    }

    fn find_next(&self) {
        let information_opt = if let Ok(mut gallery) = self.repository.gallery_rc().try_borrow_mut()
        {
            if let Some(index) = gallery.finder.find_next() {
                let mut navigator = self.navigator_rc.borrow_mut();
                navigator.move_towards(Direction::Index { value: index });
                navigator.set_page_changed();
                None
            } else {
                self.state().set_search_in_progress(false);
                Some("end of search")
            }
        } else {
            panic!("can't borrow");
        };
        if let Some(information) = information_opt {
            self.display_information(information);
        }
    }

    fn test(&self) {
        println!("…test…");
        let application_window = self.main_view().application_window();
        gtk::prelude::ActionGroupExt::activate_action(
            &application_window,
            "main-controller.test",
            Some(&"foo bar test action".to_variant()),
        );
    }

    fn cancel_edition(&self) {
        println!("cancel_editon…");
        self.dismiss()
    }
    fn dismiss(&self) {
        {
            let gsr_entry_window_opt = self.gsr_entry_window_opt_rc.borrow();
            if let Some(gsr_entry_window) = gsr_entry_window_opt.as_ref() {
                gsr_entry_window.close()
            } else {
                dbg!("not gsr_entry_window set");
            };
        }
        *self.gsr_entry_window_opt_rc.borrow_mut() = None;
    }

    fn pick_view_option(&self) {
        let application_window = self
            .main_view_opt_rc
            .borrow()
            .as_ref()
            .unwrap()
            .application_window();
        let gsr_entry_window = GsrEntryWindow::new_with(
            &application_window,
            &self.main_controller_rc_opt.as_ref().unwrap(),
            view_menu(),
            None,
        );
        gsr_entry_window.present();
        *self.gsr_entry_window_opt_rc.borrow_mut() = Some(gsr_entry_window);
    }

    fn apply_view_setting(&self, view_option: ViewOption) {
        match view_option {
            ViewOption::Single => self.toggle_single_view(),
            ViewOption::Grid2x2 => self.toggle_grid_size(2),
            ViewOption::Grid3x3 => self.toggle_grid_size(3),
            ViewOption::Grid4x4 => self.toggle_grid_size(4),
            ViewOption::Grid5x5 => self.toggle_grid_size(5),
            ViewOption::Thumbnails => self.toggle_thumbview(),
            ViewOption::Covers => self.toggle_cover_selection(),
            ViewOption::FilePath => self.toggle_display_path(),
            ViewOption::FileDate => self.toggle_display_date(),
            ViewOption::FileSize => self.toggle_display_size(),
            ViewOption::FullSize => self.toggle_full_size(),
        }
        self.dismiss();
    }
    fn pick_order_setting(&self) {
        let application_window = self
            .main_view_opt_rc
            .borrow()
            .as_ref()
            .unwrap()
            .application_window();
        let gsr_entry_window = GsrEntryWindow::new_with(
            &application_window,
            &self.main_controller_rc_opt.as_ref().unwrap(),
            order_menu(),
            None,
        );
        gsr_entry_window.present();
        *self.gsr_entry_window_opt_rc.borrow_mut() = Some(gsr_entry_window);
    }

    fn apply_order_setting(&self, order_setting: Order) {
        self.order_by(order_setting);
        self.dismiss()
    }

    fn pick_change(&self) {
        let application_window = self
            .main_view_opt_rc
            .borrow()
            .as_ref()
            .unwrap()
            .application_window();
        let gsr_entry_window = GsrEntryWindow::new_with(
            &application_window,
            &self.main_controller_rc_opt.as_ref().unwrap(),
            change_menu(),
            None,
        );
        gsr_entry_window.present();
        *self.gsr_entry_window_opt_rc.borrow_mut() = Some(gsr_entry_window);
    }

    fn enter_label(&self) {
        self.dismiss();
        let application_window = self
            .main_view_opt_rc
            .borrow()
            .as_ref()
            .unwrap()
            .application_window();
        let gsr_entry_window = GsrEntryWindow::new_with(
            &application_window,
            &self.main_controller_rc_opt.as_ref().unwrap(),
            label_change_entry(self.repository.all_labels()),
            None,
        );
        gsr_entry_window.present();
        *self.gsr_entry_window_opt_rc.borrow_mut() = Some(gsr_entry_window);
    }

    fn confirm_label(&self, label: &str) {
        self.dismiss();
        self.label_selected_pictures(label);
    }
}
