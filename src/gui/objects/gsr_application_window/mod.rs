use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::CONFIGURATION;
use crate::env::configuration::Configuration;
use crate::env::default_values::FRAME_WINDOW_NAME;
use crate::env::default_values::FULL_OPACITY;
use crate::env::default_values::GRID_WINDOW_NAME;
use crate::env::default_values::HALF_OPACITY;
use crate::file::paths::name_and_extension;
use crate::file::paths::parent_directory;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::control::Control;
use crate::gui::control::default_controls;
use crate::gui::direction::Direction;
use crate::gui::display::title_display;
use crate::gui::key_input::entry::add_new_category;
use crate::gui::key_input::entry::add_tags_entry;
use crate::gui::key_input::entry::label_change_entry;
use crate::gui::key_input::entry::remove_tags_entry;
use crate::gui::key_input::entry::rename_entry;
use crate::gui::key_input::information::information;
use crate::gui::key_input::menu::catalog_menu;
use crate::gui::key_input::menu::change_menu;
use crate::gui::key_input::menu::find_menu;
use crate::gui::key_input::menu::order_menu;
use crate::gui::key_input::menu::view_menu;
use crate::gui::mode::Mode;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use crate::gui::objects::gsr_picture_frame::GsrPictureFrame;
use crate::gui::objects::gsr_picture_grid::GsrPictureGrid;
use crate::gui::objects::gsr_treelist_window::GsrTreelistWindow;
use crate::gui::view::treelist_window::TreeListWindow;
use crate::gui::view_state::ViewState;
use crate::gui::view_state::navigator::Navigator;
use crate::gui::view_state::selection_range::SelectionRange;
use crate::model::catalog::Catalog;
use crate::model::category::Category;
use crate::model::category::category_from_string;
use crate::model::finder::Predicate;
use crate::model::order::Order;
use crate::model::picture::Picture;
use crate::model::repository::Repository;
use crate::model::shared::Shared;
use crate::model::tags::Tags;
use crate::model::view_option::ViewOption;
use gtk::glib;
use gtk::glib::Propagation;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::cell::RefCell;
use std::rc::Rc;

pub const LEFT_PANE: usize = 0;
pub const RIGHT_PANE: usize = 1;

mod imp;

glib::wrapper! {
    pub struct GsrApplicationWindow(ObjectSubclass<imp::GsrApplicationWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::ConstraintTarget,
            gtk::Native,
            gtk::Root,
            gtk::ShortcutManager,
            gtk::gio::ActionGroup,
            gtk::gio::ActionMap;
}

// GSR_WINDOW
impl GsrApplicationWindow {
    pub fn new(application: &GsrApplication) -> Self {
        let obj = glib::Object::builder()
            .property("application", application)
            .build();
        obj
    }
    pub fn shared_view_state(&self) -> Shared<ViewState> {
        self.gsr_application().shared_view_state()
    }

    pub fn with_view_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&ViewState) -> R,
    {
        let shared_view_state = self.shared_view_state();
        let view_state = shared_view_state.borrow();

        f(&view_state)
    }

    pub fn with_view_state_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut ViewState) -> R,
    {
        let shared_view_state = self.shared_view_state();
        let mut view_state = shared_view_state.borrow_mut();

        f(&mut view_state)
    }

    pub fn with_repository<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Repository) -> R,
    {
        let shared_repository_opt = self.gsr_application().shared_repository_opt();
        let binding = shared_repository_opt.borrow();
        let repository = binding.as_ref().unwrap();
        f(&repository)
    }

    pub fn stack(&self) -> gtk::Stack {
        self.first_child()
            .expect("no child on stack")
            .downcast::<gtk::Stack>()
            .expect("can't donwcast stack")
    }

    fn set_stack_visible_child(&self, pictures_per_row: i32) {
        let visible_child: gtk::ScrolledWindow = if pictures_per_row > 1 {
            self.grid_scrolled_window()
        } else {
            self.frame_scrolled_window()
        };
        self.stack().set_visible_child(&visible_child);
    }

    pub fn frame(&self) -> GsrPictureFrame {
        self.stack()
            .child_by_name(FRAME_WINDOW_NAME)
            .expect("frame scrolled window not set")
            .downcast::<gtk::ScrolledWindow>()
            .expect("can't downcast frame scrolled window")
            .first_child()
            .expect("gsr frame scrolled windew viewport not set")
            .downcast::<gtk::Viewport>()
            .expect("can't downcast frame scrolled window viewport")
            .first_child()
            .expect("gsr picture frame not set")
            .downcast::<GsrPictureFrame>()
            .expect("can't downcast to GsrPictureFrame")
    }

    pub fn gsr_application(&self) -> GsrApplication {
        self.application()
            .expect("no application set")
            .downcast::<GsrApplication>()
            .expect("not a GsrApplication")
    }
    pub fn initialize(&self) {
        let command_line_arguments = self
            .gsr_application()
            .shared_command_line_arguments()
            .borrow()
            .clone();
        let shared_main_controller = self.gsr_application().shared_main_controller();
        {
            let mut main_controller = shared_main_controller.borrow_mut();
            let shared_gsr_application_window = Rc::new(RefCell::new(self.clone()));
            main_controller.set_application_window(shared_gsr_application_window);
            main_controller.initialize();
        }

        self.set_default_width(command_line_arguments.width.unwrap());
        self.set_default_height(command_line_arguments.height.unwrap());
        // build the components
        let frame = GsrPictureFrame::new();
        let frame_scrolled_window = make_scrolled_window_with_child(&frame);
        let gsr_picture_grid = GsrPictureGrid::new();
        let panel = make_panel_with_child(&gsr_picture_grid);
        let grid_scrolled_window = make_scrolled_window_with_child(&panel);
        let stack = gtk::Stack::builder().hexpand(true).vexpand(true).build();
        let _ = stack.add_named(&frame_scrolled_window, Some(FRAME_WINDOW_NAME));
        let _ = stack.add_named(&grid_scrolled_window, Some(GRID_WINDOW_NAME));
        self.set_child(Some(&stack));
        {
            let pictures_per_row = {
                let shared_view_state = self.shared_view_state();
                let view_state = shared_view_state.borrow();
                view_state.settings.pictures_per_row()
            };
            if pictures_per_row == 1 {
                stack.set_visible_child(&frame_scrolled_window);
                frame.set_current_picture();
            } else {
                stack.set_visible_child(&grid_scrolled_window);
                self.gsr_picture_grid().initialize_pictures();
                self.gsr_picture_grid().leave_current_picture_focus();
                self.gsr_picture_grid().enter_current_picture_focus();
            }
        }
        self.refresh_title();
        // connect the events
        self.attach_key_pressed_event_handlers();
        let left_panel = panel
            .child_at(0, 0)
            .expect("left panel not set")
            .downcast::<gtk::Label>()
            .expect("left panel not a label");

        let right_panel = panel
            .child_at(2, 0)
            .expect("right panel not set")
            .downcast::<gtk::Label>()
            .expect("right panel not a label");

        left_panel.add_controller(Self::left_panel_click_gesture(self));
        right_panel.add_controller(Self::right_panel_click_gesture(self));
        self.gsr_picture_grid().leave_current_picture_focus();
        self.gsr_picture_grid().enter_current_picture_focus();
    }

    fn left_panel_click_gesture(gsr_application_window: &Self) -> gtk::GestureClick {
        let gesture_click = gtk::GestureClick::new();
        gesture_click.set_button(1);
        gesture_click.connect_pressed(clone!(
            #[strong]
            gsr_application_window,
            move |_, n_pressed, _, _| {
                match n_pressed {
                    1 => gsr_application_window.grid_view_move(&Direction::PrevPage),
                    2 => gsr_application_window.grid_view_move(&Direction::First),
                    _ => {}
                }
            }
        ));
        gesture_click
    }

    fn right_panel_click_gesture(gsr_application_window: &Self) -> gtk::GestureClick {
        let gesture_click = gtk::GestureClick::new();
        gesture_click.set_button(1);
        gesture_click.connect_pressed(clone!(
            #[strong]
            gsr_application_window,
            move |_, n_pressed, _, _| {
                match n_pressed {
                    1 => gsr_application_window.grid_view_move(&Direction::NextPage),
                    2 => gsr_application_window.grid_view_move(&Direction::Last),
                    _ => {}
                }
            }
        ));
        gesture_click
    }
    pub fn toggle_palette(&self) {
        let single_view = self.with_view_state_mut(|view_state| {
            view_state.settings.toggle_palette();
            view_state.settings.single_view()
        });
        if single_view {
            self.frame().set_current_picture();
        } else {
            self.gsr_picture_grid().initialize_pictures();
            self.gsr_picture_grid().leave_current_picture_focus();
            self.gsr_picture_grid().enter_current_picture_focus();
        }
    }

    pub fn toggle_pictures_per_row(&self, pictures_per_row: i32) {
        self.with_view_state_mut(|view_state| {
            let new_pictures_per_row = view_state
                .settings
                .toggle_pictures_per_row(pictures_per_row);
            view_state
                .navigator
                .set_pictures_per_row(new_pictures_per_row as usize);
            view_state.navigator.update_page_limits();
            if let Some((row, col)) = view_state
                .navigator
                .coords_from_position(view_state.navigator.position())
            {
                view_state.focus_at_coords = (col as i32, row as i32);
            }
        });
        self.refresh_view()
    }

    fn retrieve_from_repository(
        &self,
        covers_only_opt: Option<bool>,
        sub_directory: Option<String>,
        predicate_opt: Option<Predicate>,
    ) {
        {
            let shared_command_line_arguments =
                self.gsr_application().shared_command_line_arguments();
            let initial_command_line_arguments = shared_command_line_arguments.borrow().clone();
            let command_line_arguments = CommandLineArguments {
                covers: covers_only_opt.unwrap_or_default(),
                directory: sub_directory,
                ..initial_command_line_arguments
            };
            let configuration = CONFIGURATION.get().expect("configuration not set");

            let repository =
                Repository::new(configuration.clone(), command_line_arguments.clone(), false);

            match repository.retrieve_pictures(predicate_opt) {
                Err(_) => panic!("can't retrieve from repository"),
                Ok(_) => {
                    let repository_gallery = repository.gallery_rc().borrow_mut();
                    self.with_view_state_mut(|view_state| {
                        view_state.gallery = repository_gallery.clone();
                        view_state.navigator = Navigator::new(
                            repository_gallery.len(),
                            view_state.settings.pictures_per_row() as usize,
                        );
                    })
                }
            }
        }
    }

    fn refresh_view(&self) {
        let pictures_per_row =
            self.with_view_state(|view_state| view_state.settings.pictures_per_row());
        self.set_stack_visible_child(pictures_per_row);
        if pictures_per_row > 1 {
            self.gsr_picture_grid().initialize_pictures();
            self.gsr_picture_grid().leave_current_picture_focus();
            self.with_view_state_mut(|view_state| {
                if let Some((row, col)) = view_state
                    .navigator
                    .coords_from_position(view_state.navigator.position())
                {
                    view_state.focus_at_coords = (col as i32, row as i32);
                }
            });
            self.gsr_picture_grid().enter_current_picture_focus();
        } else {
            self.frame().set_current_picture();
        }
        self.refresh_title();
    }

    fn refresh_title(&self) {
        let shared_view_state = self.shared_view_state();
        self.set_title(Some(&title_display(&shared_view_state.borrow())));
    }

    pub fn frame_scrolled_window(&self) -> gtk::ScrolledWindow {
        self.first_child()
            .expect("application window stack not set")
            .downcast::<gtk::Stack>()
            .expect("not a stack")
            .first_child()
            .expect("application window frame scrolled window not set")
            .downcast::<gtk::ScrolledWindow>()
            .expect("not a scrolled window")
    }
    pub fn grid_scrolled_window(&self) -> gtk::ScrolledWindow {
        self.first_child()
            .expect("application window stack not set")
            .downcast::<gtk::Stack>()
            .expect("not a stack")
            .first_child()
            .expect("application window frame scrolled window not set")
            .next_sibling()
            .expect("application window grid scrolled window not set")
            .downcast::<gtk::ScrolledWindow>()
            .expect("not a scrolled window")
    }
    fn gsr_picture_grid(&self) -> GsrPictureGrid {
        let gsw = self.grid_scrolled_window();
        let vp = gsw
            .first_child()
            .expect("grid scrolled window has no panel child")
            .downcast::<gtk::Viewport>()
            .expect("panel is not a viewport");
        let grid = vp
            .first_child()
            .expect("panel has no children")
            .downcast::<gtk::Grid>()
            .expect("panel has no grid")
            .child_at(1, 0)
            .expect("panel grid has no middle child")
            .downcast::<GsrPictureGrid>()
            .expect("middle child is not a gsr_picture_grid");
        grid
    }
    pub fn full_size_arrow_move(&self, direction: &Direction) {
        let full_size_on = self
            .gsr_application()
            .shared_view_state()
            .borrow()
            .settings
            .full_size_on();
        if self.stack().visible_child_name().unwrap() == FRAME_WINDOW_NAME && full_size_on {
            let step: f64 = 100.0;
            let window = self.frame_scrolled_window();
            let h = window.hadjustment();
            let v = window.vadjustment();
            match direction {
                Direction::Right => h.set_value(h.value() + step),
                Direction::Left => h.set_value(h.value() - step),
                Direction::Down => v.set_value(v.value() + step),
                Direction::Up => v.set_value(v.value() - step),
                _ => {}
            }
        }
    }
    pub fn cell_box_left_click(&self, col: i32, row: i32, n_pressed: i32) {
        let position_opt = self.with_view_state(|view_state| {
            view_state
                .navigator
                .position_from_coords(row as usize, col as usize)
        });
        if let Some(position) = position_opt {
            match n_pressed {
                1 => self.grid_view_move(&Direction::Index { value: position }),
                2 => {
                    self.grid_view_move(&Direction::Index { value: position });
                    self.set_selection_range(SelectionRange::End);
                }
                _ => {}
            }
        }
    }

    pub fn cell_box_right_click(&self, col: i32, row: i32, _n_pressed: i32) {
        let position_opt = self.with_view_state(|view_state| {
            view_state
                .navigator
                .position_from_coords(row as usize, col as usize)
        });
        if let Some(position) = position_opt {
            self.grid_view_move(&Direction::Index { value: position });
            self.toggle_selected();
        }
    }

    fn attach_key_pressed_event_handlers(&self) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong (rename_to = this)]
            self,
            move |_, key, _key_code, _modifier_type| {
                let settings = {
                    let shared_view_state = this.gsr_application().shared_view_state();
                    let view_state = shared_view_state.borrow();
                    view_state.settings.clone()
                };
                let key_name = key.name().unwrap_or_default();
                let key_name = key_name.as_str();
                let key_name = key_name.to_string();
                if let Some(control) = default_controls().get(&(key_name, Mode::View)) {
                    match control {
                        Control::Right | Control::Left | Control::Up | Control::Down => {
                            let direction = Direction::from(control.clone());
                            if settings.single_view() {
                                if settings.full_size_on() {
                                    this.full_size_arrow_move(&direction)
                                } else {
                                    this.single_view_move(&direction)
                                }
                            } else {
                                this.grid_view_move(&direction)
                            }
                        }
                        Control::MovePrev => {
                            if settings.single_view() {
                                this.single_view_move(&Direction::Left)
                            } else {
                                this.grid_view_move(&Direction::PrevPage)
                            }
                        }
                        Control::MoveNext => {
                            if settings.single_view() {
                                this.single_view_move(&Direction::Right)
                            } else {
                                this.grid_view_move(&Direction::NextPage)
                            }
                        }
                        Control::MoveStartPage => {
                            if settings.single_view() {
                                this.single_view_move(&Direction::First)
                            } else {
                                this.grid_view_move(&Direction::PageStart)
                            }
                        }
                        Control::MoveEndPage => {
                            if settings.single_view() {
                                this.single_view_move(&Direction::Last)
                            } else {
                                this.grid_view_move(&Direction::PageEnd)
                            }
                        }
                        Control::MoveFirst => {
                            if settings.single_view() {
                                this.single_view_move(&Direction::First)
                            } else {
                                this.grid_view_move(&Direction::First)
                            }
                        }
                        Control::MoveLast => {
                            if settings.single_view() {
                                this.single_view_move(&Direction::Last)
                            } else {
                                this.grid_view_move(&Direction::Last)
                            }
                        }
                        Control::ToggleCoverSelection => this.toggle_view_covers(),
                        Control::BackFromDirectory => this.back_from_directory(),
                        Control::CancelRange => this.cancel_range(),
                        Control::EnterFind => this.pick_find(),
                        Control::PickChange => this.pick_change(),
                        Control::Quit => this.action_quit(),
                        Control::GotoDirectory => this.goto_directory(),
                        Control::RepeatRange => this.repeat_range(),
                        Control::RepeatLastAction => this.repeat_last_action(),
                        Control::SetOrder => this.set_order(),
                        Control::SetView => this.set_view(),
                        Control::SetSelectionRangeEnd => {
                            this.set_selection_range(SelectionRange::End)
                        }
                        Control::SetSelectionRangeAll => {
                            this.set_selection_range(SelectionRange::All)
                        }
                        Control::SetSelectionRangePage => {
                            this.set_selection_range(SelectionRange::Page)
                        }
                        Control::ToggleBlinking => this.toggle_blinking(),
                        Control::ToggleExpand => this.toggle_expand(),
                        Control::TogglePalette => this.toggle_palette(),
                        Control::ToggleSelected => this.toggle_selected(),
                        Control::ToggleSingleView => this.toggle_pictures_per_row(1),
                        Control::ToggleThumbView => this.toggle_pictures_per_row(10),
                        Control::ToggleTwoByTwoView => this.toggle_pictures_per_row(2),
                        _ => {}
                    }
                }
                Propagation::Stop
            }
        ));
        self.add_controller(event_controller_key);
    }

    pub fn process_gio_action(
        &self,
        action: &gtk::gio::SimpleAction,
        variant: Option<&gtk::glib::Variant>,
    ) {
        let gio_action = GioAction::from((action, variant));
        let action = Action::from(gio_action);
        self.process_action(action);
    }

    pub fn process_action(&self, action: Action) {
        // println!("processing action: {:?}", &action);
        match action {
            Action::Nothing => {}
            Action::Dismiss | Action::Cancel => self.dismiss(),
            Action::Quit => self.action_quit(),
            Action::AddCategory(ref new_category_name, ref target_category_name) => {
                self.action_add_category(&new_category_name, &target_category_name)
            }
            Action::ApplyOrderSetting(order) => self.action_apply_order_setting(order),
            Action::ApplyViewSetting(view_option) => self.action_apply_view_setting(view_option),
            Action::Categorize(ref category) => self.action_categorize(category),
            Action::EnterAddTag => self.action_enter_add_tag(),
            Action::EnterNewCategory => self.action_enter_new_category(),
            Action::SelectCategoryForPicture => self.action_select_category(),
            Action::EnterRemoveTag => self.action_enter_remove_tag(),
            Action::EnterRename => self.action_enter_rename(),
            Action::EnterLabel => self.action_enter_label(),
            Action::Unlabel => self.action_unlabel(),
            Action::Label(ref label) => self.action_label(&label),
            Action::AddTag(ref tags) => self.action_tag(&tags),
            Action::MoveCategory(ref category_name, ref target_category_name) => {
                self.action_move_category(&category_name, &target_category_name)
            }
            Action::PickCatalogChange => self.action_pick_catalog_change(),
            Action::RemoveCategory(ref category_name) => {
                self.action_remove_category(&category_name)
            }
            Action::RemoveTag(ref tags) => self.action_untag(&tags),
            Action::Rename(ref name) => self.action_rename(&name),
            Action::SelectCategoryAddTarget(ref name) => {
                self.action_select_category_add_target(&name)
            }
            Action::SelectCategoryMoveTarget(ref name) => {
                self.action_select_category_move_target(&name)
            }
            Action::SelectCategoryToMove => self.action_select_category_to_move(),
            Action::SelectCategoryToRemove => self.action_select_category_to_remove(),
            Action::ToggleCover => self.action_toggle_cover(),
            _ => {
                println!("* * * todo: {:?}", action);
            }
        };
        if action.is_repeatable() {
            *self.imp().last_action.borrow_mut() = action.clone();
        }
    }
    fn begin_entry(&self, gsr_entry_window: GsrEntryWindow) {
        gsr_entry_window.present();
        *self.imp().gsr_entry_window.borrow_mut() = gsr_entry_window;
        self.imp().entry_on.set(true);
    }

    fn dismiss(&self) {
        if self.imp().entry_on.get() {
            self.imp().gsr_entry_window.borrow().close();
            self.imp().entry_on.set(false);
        }
        if self.imp().treelist_on.get() {
            self.imp().gsr_treelist_window.borrow().close();
            self.imp().treelist_on.set(false);
        }
    }
    fn begin_treelist_selection(&self, gsr_treelist_window: GsrTreelistWindow) {
        gsr_treelist_window.present();
        let initial_position = gsr_treelist_window.position();
        gsr_treelist_window.list_view().scroll_to(
            initial_position,
            gtk::ListScrollFlags::FOCUS,
            None,
        );

        *self.imp().gsr_treelist_window.borrow_mut() = gsr_treelist_window;
        self.imp().treelist_on.set(true);
    }

    fn action_quit(&self) {
        self.with_view_state(|view_state| {
            if let Ok(mut configuration) = Configuration::from_env() {
                configuration.current_picture =
                    Some(view_state.gallery.current_picture().file_path());
                configuration.current_pictures_per_row =
                    Some(view_state.settings.pictures_per_row() as usize);
                configuration.current_order = Some(view_state.gallery.order());
                let _ = configuration.save();
            }
        });
        self.gsr_picture_grid().leave_current_picture_focus();
        self.close();
    }

    fn action_add_category(&self, new_category_name: &str, target_category_name: &str) {
        self.dismiss();
        let result = self.with_repository(|repository| {
            repository.add_category(new_category_name, target_category_name)
        });
        match result {
            Ok(_) => {}
            Err(e) => self.present_information(&format!("{}", e)),
        }
    }

    fn action_move_category(&self, category_name: &str, target_category_name: &str) {
        self.dismiss();
        let result = self.with_repository(|repository| {
            repository.move_category(category_name, target_category_name)
        });
        match result {
            Ok(_) => {}
            Err(e) => self.present_information(&format!("{}", e)),
        }
    }

    fn action_remove_category(&self, category_name: &str) {
        self.dismiss();
        let result = self.with_repository(|repository| repository.remove_category(category_name));
        match result {
            Ok(_) => {}
            Err(e) => self.present_information(&format!("{}", e)),
        }
    }

    fn action_apply_order_setting(&self, order: Order) {
        self.dismiss();
        self.with_view_state_mut(|view_state| {
            let gallery = &mut view_state.gallery;
            gallery.sort_by(order);
            let new_position = gallery.current_picture_index();
            let direction = Direction::Index {
                value: new_position,
            };
            if view_state.navigator.can_move(&direction) {
                view_state.navigator.move_towards(&direction);
            } else {
                println!("navigator can't move to: {:?}", &direction);
            };
            view_state.navigator.set_page_changed();
        });
        self.refresh_view();
    }

    fn action_apply_view_setting(&self, view_option: ViewOption) {
        self.dismiss();
        match view_option {
            ViewOption::Single => self.toggle_pictures_per_row(1),
            ViewOption::Grid2x2 => self.toggle_pictures_per_row(2),
            ViewOption::Grid3x3 => self.toggle_pictures_per_row(3),
            ViewOption::Grid4x4 => self.toggle_pictures_per_row(4),
            ViewOption::Grid5x5 => self.toggle_pictures_per_row(5),
            ViewOption::Thumbnails => self.toggle_pictures_per_row(10),
            ViewOption::Covers => self.toggle_view_covers(),
            ViewOption::FilePath | ViewOption::FileDate | ViewOption::FileSize => {
                self.toggle_view_display_option(view_option)
            }
            ViewOption::FullSize => self.toggle_expand(),
            ViewOption::Catalog => self.action_view_catalog(),
        }
    }

    fn retrieve_all_labels(&self) -> Tags {
        let tags = self.with_repository(|repository| {
            let _ = repository.retrieve_all_labels();
            repository.all_labels()
        });
        tags
    }
    fn action_enter_add_tag(&self) {
        self.dismiss();
        let tags = self.retrieve_all_labels();
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            add_tags_entry(tags),
            None,
        );
        self.begin_entry(gsr_entry_window);
    }

    fn action_enter_new_category(&self) {
        self.dismiss();
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            add_new_category(),
            None,
        );
        self.begin_entry(gsr_entry_window);
    }

    fn action_select_category(&self) {
        self.dismiss();
        let mut current_category: Category = None;
        let mut category_found: bool = false;
        for position in self.selected_indices() {
            let category = category_from_string(&self.with_view_state(|view_state| {
                view_state.gallery.picture(position).category_name()
            }));
            if !category_found {
                current_category = category;
                category_found = true;
            } else {
                if category != current_category {
                    current_category = None;
                    break;
                }
            }
        }
        let catalog = self.with_repository(|repository| repository.catalog());
        let gsr_treelist_window = GsrTreelistWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            &catalog,
            "Select a category",
            current_category.as_deref(),
            Action::Categorize(None),
        );
        self.begin_treelist_selection(gsr_treelist_window);
    }

    fn action_view_catalog(&self) {
        let catalog = self.with_repository(|repository| repository.catalog());
        let gsr_treelist_window = GsrTreelistWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            &catalog,
            "List of all categories",
            None,
            Action::Dismiss,
        );
        self.begin_treelist_selection(gsr_treelist_window);
    }
    fn action_select_category_add_target(&self, name: &str) {
        self.dismiss();
        let catalog = self.with_repository(|repository| repository.catalog());
        let gsr_treelist_window = GsrTreelistWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            &catalog,
            &format!("Select the category where to add {name}"),
            None,
            Action::AddCategory(name.to_string(), String::from("")),
        );
        self.begin_treelist_selection(gsr_treelist_window);
    }

    fn action_select_category_move_target(&self, name: &str) {
        self.dismiss();
        let mut catalog = self.with_repository(|repository| repository.catalog());
        match catalog.remove_category(name, true) {
            Err(e) => {
                self.present_information(&format!("{e}"));
                return;
            }
            Ok(_) => {
                let gsr_treelist_window = GsrTreelistWindow::new_with(
                    self,
                    &self.gsr_application().shared_main_controller(),
                    &catalog,
                    &format!("Select the category where to rattach {name}"),
                    None,
                    Action::MoveCategory(name.to_string(), String::from("")),
                );
                self.begin_treelist_selection(gsr_treelist_window);
            }
        }
    }

    fn action_select_category_to_move(&self) {
        self.dismiss();
        let catalog = self.with_repository(|repository| repository.catalog());
        let gsr_treelist_window = GsrTreelistWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            &catalog,
            &format!("Select the category to move"),
            None,
            Action::SelectCategoryMoveTarget(String::from("")),
        );
        self.begin_treelist_selection(gsr_treelist_window);
    }

    fn action_select_category_to_remove(&self) {
        self.dismiss();
        let catalog = self.with_repository(|repository| repository.catalog());
        let gsr_treelist_window = GsrTreelistWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            &catalog,
            &format!("Select the category to remove"),
            None,
            Action::RemoveCategory(String::from("")),
        );
        self.begin_treelist_selection(gsr_treelist_window);
    }

    fn action_enter_remove_tag(&self) {
        self.dismiss();
        let tags = self.retrieve_all_labels();
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            remove_tags_entry(tags),
            None,
        );
        self.begin_entry(gsr_entry_window);
    }

    fn action_enter_rename(&self) {
        self.dismiss();
        let (selected_count, current_picture_name) = self.with_view_state(|view_state| {
            (
                view_state.selection.count(),
                view_state.gallery.current_picture().file_name(),
            )
        });
        if selected_count != 1 {
            self.present_information("select one picture to rename first");
            return;
        };
        let (name, _extension) = name_and_extension(&current_picture_name);
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            rename_entry(),
            Some(&name),
        );
        self.begin_entry(gsr_entry_window);
    }

    fn action_enter_label(&self) {
        self.dismiss();
        let tags = self.retrieve_all_labels();
        let label = self.with_view_state(|view_state| view_state.gallery.current_picture().label());
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            label_change_entry(tags),
            Some(&label),
        );
        self.begin_entry(gsr_entry_window);
    }

    fn action_pick_catalog_change(&self) {
        self.dismiss();
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            catalog_menu(),
            None,
        );
        self.begin_entry(gsr_entry_window);
    }

    fn selected_indices(&self) -> Vec<usize> {
        self.with_view_state(|view_state| {
            if view_state.selection.has_selected() {
                assert!(view_state.selection.count() == view_state.selection.indices().len());
                view_state.selection.indices()
            } else {
                vec![view_state.gallery.current_picture_index()]
            }
        })
    }

    fn action_tag(&self, input: &str) {
        let tags: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
        let indices = self.selected_indices();
        self.dismiss();
        for position in indices {
            self.with_view_state_mut(|view_state| {
                let mut picture = view_state.gallery.picture(position);
                tags.iter().for_each(|tag| {
                    picture.add_tag(tag);
                    self.with_repository(|repository| match repository.update_picture(&picture) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    })
                });
                view_state.gallery.set_picture(position, picture);
            });
        }
        self.refresh_view();
    }

    fn action_untag(&self, input: &str) {
        let tags: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
        let indices = self.selected_indices();
        self.dismiss();
        for position in indices {
            self.with_view_state_mut(|view_state| {
                let mut picture = view_state.gallery.picture(position);
                tags.iter().for_each(|tag| {
                    picture.remove_tag(tag);
                    self.with_repository(|repository| match repository.update_picture(&picture) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{}", e),
                    })
                });
                view_state.gallery.set_picture(position, picture);
            });
        }
        self.refresh_view();
    }

    fn action_categorize(&self, category: &Category) {
        let indices = self.selected_indices();
        self.dismiss();
        for position in indices {
            self.with_view_state_mut(|view_state| {
                let mut picture = view_state.gallery.picture(position);
                picture.set_category(category.clone());
                self.with_repository(|repository| match repository.update_picture(&picture) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                });
                view_state.gallery.set_picture(position, picture);
            });
        }
        self.refresh_view();
    }

    fn action_rename(&self, target_name: &str) {
        self.dismiss();
        if target_name.is_empty() {
            self.present_information("picture name can't be empty");
            return;
        }
        let (current_name, extension) = self.with_view_state(|view_state| {
            name_and_extension(&view_state.gallery.current_picture().file_name())
        });
        if target_name == current_name {
            self.present_information("picture name is unchanged");
            return;
        }
        self.with_view_state_mut(|view_state| {
            let position = view_state.gallery.current_picture_index();
            let picture = view_state.gallery.current_picture();
            let new_picture = Picture::copy_with_name(&picture, target_name);
            self.with_repository(|repository| {
                repository.rename_picture(&picture, target_name);
            });
            view_state.gallery.set_picture(position, new_picture);
        });
        self.refresh_view();
    }
    fn action_label(&self, label: &str) {
        self.dismiss();
        let indices = self.selected_indices();
        for position in indices {
            self.with_view_state_mut(|view_state| {
                let mut picture = view_state.gallery.picture(position);
                picture.set_label(label);
                self.with_repository(|repository| match repository.update_picture(&picture) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                });
                view_state.gallery.set_picture(position, picture);
            });
        }
        self.refresh_view();
    }

    fn action_unlabel(&self) {
        self.dismiss();
        let indices = self.selected_indices();
        for position in indices {
            self.with_view_state_mut(|view_state| {
                let mut picture = view_state.gallery.picture(position);
                picture.set_label("");
                self.with_repository(|repository| match repository.update_picture(&picture) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                });
                view_state.gallery.set_picture(position, picture);
            });
        }
        self.refresh_view();
    }

    fn action_toggle_cover(&self) {
        self.dismiss();
        self.with_view_state_mut(|view_state| {
            let position = view_state.gallery.current_picture_index();
            self.with_repository(|repository| {
                let counts = repository.directory_count_at_index(position);
                let mut picture = view_state.gallery.current_picture().clone();
                picture.toggle_cover(counts.0);
                match repository.update_picture(&picture) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                }
                view_state.gallery.set_picture(position, picture);
            });
        });
        self.refresh_view();
    }
    fn cancel_range(&self) {
        self.with_view_state_mut(|view_state| {
            view_state.selection.cancel();
        });
        self.refresh_view();
    }

    fn pick_change(&self) {
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            change_menu(),
            None,
        );
        self.begin_entry(gsr_entry_window);
    }

    fn pick_find(&self) {
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_main_controller(),
            find_menu(),
            None,
        );
        self.begin_entry(gsr_entry_window);
    }

    fn set_order(&self) {
        {
            let gsr_entry_window = GsrEntryWindow::new_with(
                self,
                &self.gsr_application().shared_main_controller(),
                order_menu(),
                None,
            );
            self.begin_entry(gsr_entry_window);
        }
    }

    fn set_view(&self) {
        {
            let gsr_entry_window = GsrEntryWindow::new_with(
                self,
                &self.gsr_application().shared_main_controller(),
                view_menu(),
                None,
            );
            self.begin_entry(gsr_entry_window);
        }
    }

    fn present_information(&self, message: &str) {
        {
            let gsr_entry_window = GsrEntryWindow::new_with(
                self,
                &self.gsr_application().shared_main_controller(),
                information(),
                None,
            );
            gsr_entry_window.set_entry_text(message);
            self.begin_entry(gsr_entry_window);
        }
    }
    fn goto_directory(&self) {
        let (current_picture, covers_only) = self.with_view_state(|view_state| {
            (
                view_state.gallery.current_picture(),
                view_state.settings.covers_only(),
            )
        });
        let parent_directory_opt = parent_directory(&current_picture.file_path());
        if !covers_only {
            self.present_information("can only go to a directory when in covers view");
            return;
        };
        if covers_only && current_picture.is_cover() && parent_directory_opt.clone().is_some() {
            let location = self.with_view_state(|view_state| {
                (
                    view_state.gallery.sub_folder(),
                    view_state.navigator.position(),
                )
            });
            self.retrieve_from_repository(None, parent_directory_opt.clone(), None);
            self.with_view_state_mut(|view_state| {
                view_state.positions.push(location);
                view_state
                    .gallery
                    .set_sub_folder(parent_directory_opt.clone());
                view_state.settings.toggle_covers_only();
            });
            self.refresh_view();
        }
    }

    fn back_from_directory(&self) {
        let nb_positions = self.with_view_state(|view_state| view_state.positions.len());
        if nb_positions > 0 {
            self.retrieve_from_repository(Some(true), None, None);
            self.with_view_state_mut(|view_state| {
                if let Some((sub_folder_opt, position)) = view_state.positions.pop() {
                    view_state.gallery.set_sub_folder(sub_folder_opt);
                    view_state.settings.toggle_covers_only();
                    if view_state
                        .navigator
                        .can_move(&Direction::Index { value: position })
                    {
                        view_state
                            .navigator
                            .move_towards(&Direction::Index { value: position })
                    } else {
                        view_state.navigator.move_towards(&Direction::First)
                    }
                    view_state.gallery.set_current_picture_index(position);
                }
            });
            self.refresh_view();
        }
    }

    fn repeat_last_action(&self) {
        let action = self.imp().last_action.borrow_mut().clone();
        self.process_action(action);
    }

    fn repeat_range(&self) {
        self.with_view_state_mut(|view_state| {
            view_state.selection.repeat();
            view_state.navigator.set_page_changed();
        });
        self.refresh_view();
    }

    fn set_selection_range(&self, range: SelectionRange) {
        self.with_view_state_mut(|view_state| {
            match range {
                SelectionRange::End => {
                    let position = view_state.navigator.position();
                    view_state.selection.set_range_end(position);
                }
                SelectionRange::All => {
                    let limit = &view_state.navigator.limit();
                    view_state.selection.set_range(0, *limit - 1);
                }
                SelectionRange::Page => {
                    let page_start = &view_state.navigator.page_start();
                    let page_end = &view_state.navigator.page_end();
                    view_state.selection.set_range(*page_start, *page_end);
                }
            }
            view_state.navigator.set_page_changed();
        });
        self.refresh_view();
    }

    fn toggle_blinking(&self) {
        let on = self.with_view_state_mut(|view_state| {
            view_state.settings.toggle_blinking();
            view_state.settings.blinking_on()
        });
        if on == true {
            self.gsr_picture_grid().initialize_pictures();
            self.gsr_picture_grid().leave_current_picture_focus();
            self.gsr_picture_grid().enter_current_picture_focus();
        }
    }

    fn toggle_expand(&self) {
        let pictures_per_row = self.with_view_state_mut(|view_state| {
            if view_state.settings.pictures_per_row() == 1 {
                view_state.settings.toggle_view_mode();
            }
            view_state.settings.pictures_per_row()
        });
        if pictures_per_row == 1 {
            self.frame().set_current_picture();
            self.refresh_title();
        }
    }

    fn toggle_view_display_option(&self, view_option: ViewOption) {
        self.with_view_state_mut(|view_state| {
            let settings = &mut view_state.settings;
            let _ = match view_option {
                ViewOption::FilePath => settings.toggle_file_path(),
                ViewOption::FileDate => settings.toggle_file_date(),
                ViewOption::FileSize => settings.toggle_file_size(),
                _ => true,
            };
        });
        self.refresh_title();
    }

    fn toggle_selected(&self) {
        self.with_view_state_mut(|view_state| {
            let position = view_state.navigator.position();
            if view_state.selection.contains(position) {
                view_state.selection.unselect(position)
            } else {
                view_state.selection.select(position)
            }
            view_state.navigator.set_page_changed()
        });
        self.refresh_view()
    }

    fn toggle_view_covers(&self) {
        let (gallery_has_covers, sub_folder) = self.with_view_state(|view_state| {
            (
                view_state.gallery.has_covers(),
                view_state.gallery.sub_folder(),
            )
        });
        if gallery_has_covers && sub_folder.is_none() {
            let covers_only =
                self.with_view_state_mut(|view_state| view_state.settings.toggle_covers_only());
            self.retrieve_from_repository(Some(covers_only), None, None);
            self.refresh_view()
        }
    }

    fn move_navigator(&self, direction: &Direction) -> Navigator {
        let navigator = self.with_view_state_mut(|view_state| {
            if view_state.navigator.can_move(&direction) {
                view_state.navigator.move_towards(&direction);
            }
            view_state.navigator.clone()
        });
        self.with_view_state_mut(|view_state| {
            let gallery = &mut view_state.gallery;
            gallery.set_current_picture_index(navigator.position());
        });
        navigator
    }

    fn single_view_move(&self, direction: &Direction) {
        let direction = match direction {
            Direction::Right | Direction::Down => Direction::NextPage,
            Direction::Left | Direction::Up => Direction::PrevPage,
            other => other.clone(),
        };
        let navigator = self.move_navigator(&direction);
        if navigator.has_moved() {
            self.frame().set_current_picture();
            self.refresh_title();
        }
    }
    fn grid_view_move(&self, direction: &Direction) {
        let navigator = self.move_navigator(direction);
        if navigator.has_moved() {
            {
                self.gsr_picture_grid().leave_current_picture_focus();
                if let Some((row, col)) = navigator.coords_from_position(navigator.position()) {
                    self.with_view_state_mut(|view_state| {
                        view_state.focus_at_coords = (col as i32, row as i32);
                    })
                }
                if navigator.page_changed() {
                    self.gsr_picture_grid().initialize_pictures();
                }
                self.gsr_picture_grid().enter_current_picture_focus();
                self.refresh_title();
            }
        }
    }
    pub fn popup_treelist_window(&self, prompt: &str, catalog: &Catalog) -> TreeListWindow {
        let treelist_window = TreeListWindow::new(&self, prompt, "", catalog);
        treelist_window.popup();
        treelist_window
    }
}

fn make_scrolled_window_with_child<W>(child: &W) -> gtk::ScrolledWindow
where
    W: IsA<gtk::Widget>,
{
    let window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    window.set_child(Some(child));
    window
}

#[allow(deprecated)]
fn make_panel_with_child(gsr_picture_grid: &GsrPictureGrid) -> gtk::Grid {
    let panel = gtk::Grid::new();
    panel.set_hexpand(true);
    panel.set_vexpand(true);
    panel.set_row_homogeneous(true);
    panel.set_column_homogeneous(false);
    let left_pane = gtk::Label::new(Some("←"));
    let right_pane = gtk::Label::new(Some("→"));
    left_pane.set_width_chars(5);
    left_pane.add_css_class("pane");
    right_pane.set_width_chars(5);
    right_pane.add_css_class("pane");
    panel.attach(&left_pane, 0, 0, 1, 1);
    panel.attach(gsr_picture_grid, 1, 0, 1, 1);
    panel.attach(&right_pane, 2, 0, 1, 1);
    panel
}

pub fn picture_opacity(selected: bool) -> f64 {
    match selected {
        false => FULL_OPACITY,
        true => HALF_OPACITY,
    }
}
