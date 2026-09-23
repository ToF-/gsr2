use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::CONFIGURATION;
use crate::env::default_values::FRAME_WINDOW_NAME;
use crate::env::default_values::FULL_OPACITY;
use crate::env::default_values::GRID_WINDOW_NAME;
use crate::env::default_values::HALF_OPACITY;
use crate::file::paths::parent_directory;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::control::Control;
use crate::gui::control::default_controls;
use crate::gui::direction::Direction;
use crate::gui::display::title_display;
use crate::gui::key_input::entry::target_directory_entry;
use crate::gui::key_input::information::information;
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
use crate::model::gallery::Gallery;
use crate::model::predicate::Predicate;
use crate::model::repository::Repository;
use crate::model::shared::Shared;
use crate::model::view_option::ViewOption;
use gtk::glib;
use gtk::glib::Propagation;
use gtk::glib::clone;
use gtk::glib::timeout_add_local;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::cell::RefCell;
use std::io::Result as IOResult;
use std::ops::ControlFlow;
use std::rc::Rc;
use std::time::Duration;

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
    pub fn new(gsr_application: &GsrApplication) -> Self {
        let obj: GsrApplicationWindow = glib::Object::builder()
            .property("application", gsr_application)
            .build();
        let binding = gsr_application.shared_controller();
        let controller = binding.borrow();
        obj.insert_action_group("main-controller", Some(&controller.gio_action_group()));
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
        f(repository)
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
        let shared_controller = self.gsr_application().shared_controller();
        {
            let mut controller = shared_controller.borrow_mut();
            let shared_gsr_application_window = Rc::new(RefCell::new(self.clone()));
            controller.set_application_window(shared_gsr_application_window);

            controller.initialize();
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
        self.start_slide_show();
    }

    pub fn start_slide_show(&self) {
        let slideshow_delay =
            self.with_view_state(|view_state| view_state.settings.slideshow_delay());
        if let Some(delay) = slideshow_delay {
            self.with_view_state_mut(|view_state| {
                view_state.settings.set_slideshow_on(true);
            });
            self.attach_timeout_event_handler(delay);
        }
    }

    pub fn stop_slide_show(&self) {
        self.with_view_state_mut(|view_state| {
            view_state.settings.set_slideshow_on(false);
        });
        self.detach_timeout_event_handler();
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

    pub fn retrieve_from_repository(
        &self,
        covers_only_opt: Option<bool>,
        sub_directory: Option<String>,
        predicate_opt: Option<Predicate>,
    ) -> IOResult<usize> {
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
                Err(e) => Err(e),
                Ok(0) => {
                    self.present_information("no picture matching these criteria");
                    Ok(0)
                }
                Ok(n) => {
                    let repository_gallery = repository.gallery_rc().borrow_mut();
                    self.with_view_state_mut(|view_state| {
                        view_state.navigator = Navigator::new(
                            repository_gallery.len(),
                            view_state.settings.pictures_per_row() as usize,
                        );
                        view_state.gallery = Gallery::from_gallery_and_navigator(
                            repository_gallery.clone(),
                            &view_state.navigator,
                        );
                    });
                    Ok(n)
                }
            }
        }
    }

    pub fn refresh_view(&self) {
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

    pub fn deselect_pictures(&self) {
        self.activate_action_for_control(&Control::CancelRange)
    }

    pub fn refresh_title(&self) {
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
    pub fn gsr_picture_grid(&self) -> GsrPictureGrid {
        let gsw = self.grid_scrolled_window();
        let vp = gsw
            .first_child()
            .expect("grid scrolled window has no panel child")
            .downcast::<gtk::Viewport>()
            .expect("panel is not a viewport");
        vp.first_child()
            .expect("panel has no children")
            .downcast::<gtk::Grid>()
            .expect("panel has no grid")
            .child_at(1, 0)
            .expect("panel grid has no middle child")
            .downcast::<GsrPictureGrid>()
            .expect("middle child is not a gsr_picture_grid")
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
            self.activate_action_toggle_selected();
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
                    this.stop_slide_show();
                    match control {
                        Control::Right | Control::Left | Control::Up | Control::Down => {
                            let direction = Direction::from(*control);
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
                        Control::MovePicture => this.enter_move_picture(),
                        Control::SetSelectionRangeEnd => {
                            this.set_selection_range(SelectionRange::End)
                        }
                        Control::SetSelectionRangeAll => {
                            this.set_selection_range(SelectionRange::All)
                        }
                        Control::SetSelectionRangePage => {
                            this.set_selection_range(SelectionRange::Page)
                        }
                        Control::RankNoStar | Control::RankOneStar | Control::RankTwoStars | Control::RankThreeStars => {
                            let action = Action::from(*control);
                            let (name, variant) = GioAction::from(action.clone()).to_simple_action_call();
                            let variant_ref = variant.as_ref();
                            match WidgetExt::activate_action(&this, &name, variant_ref) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!(
                                        "connect_key_pressed_controller for gsr_entry_window {} {:?} : {}",
                                        name, variant_ref, e
                                    )
                                }
                            }
                        },
                        Control::ToggleBlinking => {
                            let action = Action::from(*control);
                            let (name, variant) = GioAction::from(action.clone()).to_simple_action_call();
                            let variant_ref = variant.as_ref();
                            match WidgetExt::activate_action(&this, &name, variant_ref) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!(
                                        "connect_key_pressed_controller for gsr_entry_window {} {:?} : {}",
                                        name, variant_ref, e
                                    )
                                }
                            }

                        },
                        _ => this.activate_action_for_control(control),
                    }
                }
                Propagation::Stop
            }
        ));
        self.add_controller(event_controller_key);
    }

    pub fn attach_timeout_event_handler(&self, seconds: i32) {
        let delay: u64 = seconds.try_into().unwrap();
        *self.imp().timeout_rc.borrow_mut() = Some(timeout_add_local(
            Duration::new(delay, 0),
            clone!(
                #[strong (rename_to = this)]
                self,
                move || {
                    let settings = {
                        let shared_view_state = this.gsr_application().shared_view_state();
                        let view_state = shared_view_state.borrow();
                        view_state.settings.clone()
                    };
                    if settings.slideshow_on() {
                        this.activate_action(Action::NextSlide);
                        gtk::glib::ControlFlow::Continue
                    } else {
                        this.detach_timeout_event_handler();
                        gtk::glib::ControlFlow::Break
                    }
                }
            ),
        ))
    }

    fn detach_timeout_event_handler(&self) {
        if let Some(id) = self.imp().timeout_rc.borrow_mut().take() {
            id.remove();
        }
    }

    pub fn activate_action(&self, action: Action) {
        let (name, variant) = GioAction::from(action.clone()).to_simple_action_call();
        let variant_ref = variant.as_ref();
        match WidgetExt::activate_action(self, &name, variant_ref) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "connect_key_pressed_controller for gsr_entry_window {} {:?} : {}",
                    name, variant_ref, e
                )
            }
        }
    }
    pub fn activate_action_for_control(&self, control: &Control) {
        let action = Action::from(*control);
        self.activate_action(action);
    }
    fn activate_action_toggle_selected(&self) {
        let action = Action::ToggleSelected;
        let (name, variant) = GioAction::from(action.clone()).to_simple_action_call();
        let variant_ref = variant.as_ref();
        match WidgetExt::activate_action(self, &name, variant_ref) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "connect_key_pressed_controller for gsr_entry_window {} {:?} : {}",
                    name, variant_ref, e
                )
            }
        }
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
            Action::Nothing => println!("processing Action::Nothing"),
            _ => {
                println!("* * * todo: {:?}", action);
            }
        };
        if action.is_repeatable() {
            *self.imp().last_action.borrow_mut() = action.clone();
        }
    }
    pub fn begin_entry(&self, gsr_entry_window: GsrEntryWindow) {
        gsr_entry_window.present();
        *self.imp().gsr_entry_window.borrow_mut() = gsr_entry_window;
        self.imp().entry_on.set(true);
    }

    pub fn dismiss(&self) {
        if self.imp().entry_on.get() {
            self.imp().gsr_entry_window.borrow().close();
            self.imp().entry_on.set(false);
        }
        if self.imp().treelist_on.get() {
            self.imp().gsr_treelist_window.borrow().close();
            self.imp().treelist_on.set(false);
        }
    }
    pub fn begin_treelist_selection(&self, gsr_treelist_window: GsrTreelistWindow) {
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

    pub fn action_view_catalog(&self) {
        let catalog = self.with_repository(|repository| repository.catalog());
        let gsr_treelist_window = GsrTreelistWindow::new_with(
            self,
            &self.gsr_application().shared_controller(),
            &catalog,
            "List of all categories",
            None,
            Action::Dismiss,
        );
        self.begin_treelist_selection(gsr_treelist_window);
    }

    pub fn selected_indices(&self) -> Vec<usize> {
        self.with_view_state(|view_state| view_state.selected_indices())
    }

    pub fn present_information(&self, message: &str) {
        {
            let gsr_entry_window = GsrEntryWindow::new_with(
                self,
                &self.gsr_application().shared_controller(),
                information(),
                None,
            );
            gsr_entry_window.set_entry_text(message);
            self.begin_entry(gsr_entry_window);
        }
    }

    fn enter_move_picture(&self) {
        if !self.with_view_state(|view_state| view_state.selection.has_selected()) {
            self.present_information("cannot move: no picture selected");
            return;
        };
        let gsr_entry_window = GsrEntryWindow::new_with(
            self,
            &self.gsr_application().shared_controller(),
            target_directory_entry(),
            None,
        );
        let directory = self.with_view_state(|view_state| {
            parent_directory(&view_state.gallery.current_picture().file_path())
        });
        gsr_entry_window.set_entry_text(&directory.unwrap_or_default());
        self.begin_entry(gsr_entry_window);
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

    pub fn toggle_expand(&self) {
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

    pub fn toggle_view_display_option(&self, view_option: ViewOption) {
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

    pub fn quit(&self) {
        self.close()
    }

    fn move_navigator(&self, direction: &Direction) -> Navigator {
        let navigator = self.with_view_state_mut(|view_state| {
            if view_state.navigator.can_move(direction) {
                view_state.navigator.move_towards(direction);
            }
            view_state.navigator.clone()
        });
        self.with_view_state_mut(|view_state| {
            view_state.set_current_location_position(navigator.position());
        });
        navigator
    }

    pub fn move_next_slide(&self) {
        let direction = self.with_view_state_mut(|view_state| {
            if view_state.navigator.can_move(&Direction::NextPage) {
                Direction::NextPage
            } else {
                Direction::First
            }
        });
        self.move_navigator(&direction);
        self.refresh_view();
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
        let treelist_window = TreeListWindow::new(self, prompt, "", catalog);
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
