use crate::env::default_values::FRAME_WINDOW_NAME;
use crate::env::default_values::FULL_OPACITY;
use crate::env::default_values::GRID_WINDOW_NAME;
use crate::env::default_values::HALF_OPACITY;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::control::Control;
use crate::gui::control::default_controls;
use crate::gui::direction::Direction;
use crate::gui::display::title_display;
use crate::gui::key_input::menu::order_menu;
use crate::gui::mode::Mode;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use crate::gui::objects::gsr_picture_frame::GsrPictureFrame;
use crate::gui::objects::gsr_picture_grid::GsrPictureGrid;
use crate::gui::view::treelist_view::TreeListView;
use crate::gui::view::treelist_window::TreeListWindow;
use crate::gui::view_state::ViewState;
use crate::gui::view_state::navigator::Navigator;
use crate::model::catalog::Catalog;
use crate::model::order::Order;
use crate::model::shared::Shared;
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
        let single_view = {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            view_state.settings.toggle_palette();
            view_state.settings.single_view()
        };
        if single_view {
            self.frame().set_current_picture();
        } else {
            self.gsr_picture_grid().initialize_pictures();
            self.gsr_picture_grid().leave_current_picture_focus();
            self.gsr_picture_grid().enter_current_picture_focus();
        }
    }

    pub fn toggle_pictures_per_row(&self, pictures_per_row: i32) {
        let shared_view_state = self.shared_view_state();
        {
            let mut view_state = shared_view_state.borrow_mut();
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
        };
        self.refresh_view()
    }
    fn refresh_view(&self) {
        let pictures_per_row = {
            let shared_view_state = self.shared_view_state();
            shared_view_state.borrow().settings.pictures_per_row()
        };
        self.set_stack_visible_child(pictures_per_row);
        if pictures_per_row > 1 {
            self.gsr_picture_grid().initialize_pictures();
            self.gsr_picture_grid().leave_current_picture_focus();
            let shared_view_state = self.shared_view_state();
            {
                let mut view_state = shared_view_state.borrow_mut();
                if let Some((row, col)) = view_state
                    .navigator
                    .coords_from_position(view_state.navigator.position())
                {
                    view_state.focus_at_coords = (col as i32, row as i32);
                }
            }
            self.gsr_picture_grid().enter_current_picture_focus();
        } else {
            self.frame().set_current_picture();
        }
        self.refresh_title()
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
        let position_opt = {
            let shared_view_state = self.shared_view_state();
            let view_state = shared_view_state.borrow();
            view_state
                .navigator
                .position_from_coords(row as usize, col as usize)
        };
        if let Some(position) = position_opt {
            match n_pressed {
                1 => self.grid_view_move(&Direction::Index { value: position }),
                2 => {
                    self.grid_view_move(&Direction::Index { value: position });
                    self.set_selection_range_end();
                }
                _ => {}
            }
        }
    }

    pub fn cell_box_right_click(&self, col: i32, row: i32, n_pressed: i32) {
        let position_opt = {
            let shared_view_state = self.shared_view_state();
            let view_state = shared_view_state.borrow();
            view_state
                .navigator
                .position_from_coords(row as usize, col as usize)
        };
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
                        Control::CancelRange => this.cancel_range(),
                        Control::Quit => {
                            println!("closing gsr_application_window");
                            this.close()
                        } // TEMPORARY, should call a quit action that saves things
                        Control::RepeatRange => this.repeat_range(),
                        Control::SetOrder => this.set_order(),
                        Control::SetSelectionRangeEnd => this.set_selection_range_end(),
                        Control::SetSelectionRangeAll => this.set_selection_range_all(),
                        Control::SetSelectionRangePage => this.set_selection_range_page(),
                        Control::ToggleBlinking => this.toggle_blinking(),
                        Control::ToggleExpand => this.toggle_expand(),
                        Control::TogglePalette => this.toggle_palette(),
                        Control::ToggleFullSize => this.toggle_full_size(),
                        Control::ToggleSelected => this.toggle_selected(),
                        Control::ToggleSingleView => this.toggle_pictures_per_row(1),
                        Control::ToggleThumbView => this.toggle_pictures_per_row(10),
                        Control::ToggleTwoByTwoView => this.toggle_pictures_per_row(2),
                        _ => {}
                    }
                }
                Propagation::Proceed
            }
        ));
        self.add_controller(event_controller_key);
    }

    pub fn process_action(
        &self,
        action: &gtk::gio::SimpleAction,
        variant: Option<&gtk::glib::Variant>,
    ) {
        let gio_action = GioAction::from((action, variant));
        let action = Action::from(gio_action);
        match action {
            Action::Cancel => self.action_cancel(),
            Action::ApplyOrderSetting(order) => self.action_apply_order_setting(order),
            _ => {
                println!("* * * todo: {:?}", action);
            }
        }
    }
    fn action_cancel(&self) {
        self.imp().gsr_entry_window.borrow().close();
    }

    fn action_apply_order_setting(&self, order: Order) {
        self.action_cancel();
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            let mut gallery = &mut view_state.gallery;
            println!(
                "#{}:{}",
                &gallery.current_picture_index(),
                &gallery.current_picture().file_path()
            );
            gallery.sort_by(order);
            println!(
                "#{}:{}",
                &gallery.current_picture_index(),
                &gallery.current_picture().file_path()
            );
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
        }
        self.refresh_view();
    }
    fn cancel_range(&self) {
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            view_state.selection.cancel();
        }
        self.refresh_view();
    }

    fn set_order(&self) {
        {
            let gsr_entry_window = GsrEntryWindow::new_with(
                self,
                &self.gsr_application().shared_main_controller(),
                order_menu(),
                None,
            );
            gsr_entry_window.present();
            *self.imp().gsr_entry_window.borrow_mut() = gsr_entry_window;
        }
    }

    fn repeat_range(&self) {
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            view_state.selection.repeat();
            view_state.navigator.set_page_changed();
        }
        self.refresh_view();
    }

    fn set_selection_range_end(&self) {
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            let position = view_state.navigator.position();
            view_state.selection.set_range_end(position);
            view_state.navigator.set_page_changed();
        }
        self.refresh_view();
    }

    fn set_selection_range_all(&self) {
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            let limit = &view_state.navigator.limit();
            view_state.selection.set_range(0, *limit);
            view_state.navigator.set_page_changed();
        }
        self.refresh_view();
    }

    fn set_selection_range_page(&self) {
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            let page_start = &view_state.navigator.page_start();
            let page_end = &view_state.navigator.page_end();
            view_state.selection.set_range(*page_start, *page_end);
            view_state.navigator.set_page_changed();
        }
        self.refresh_view();
    }

    fn toggle_blinking(&self) {
        let on = {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            view_state.settings.toggle_blinking();
            view_state.settings.blinking_on()
        };
        if on == true {
            self.gsr_picture_grid().initialize_pictures();
            self.gsr_picture_grid().leave_current_picture_focus();
            self.gsr_picture_grid().enter_current_picture_focus();
        }
    }

    fn toggle_expand(&self) {
        let pictures_per_row = {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            if view_state.settings.pictures_per_row() == 1 {
                view_state.settings.toggle_expand();
            }
            view_state.settings.pictures_per_row()
        };
        if pictures_per_row == 1 {
            self.frame().set_current_picture();
        }
    }
    fn toggle_full_size(&self) {
        let pictures_per_row = {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            if view_state.settings.pictures_per_row() == 1 {
                view_state.settings.toggle_full_size();
            }
            view_state.settings.pictures_per_row()
        };
        if pictures_per_row == 1 {
            self.frame().set_current_picture();
        }
    }

    fn toggle_selected(&self) {
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            let position = view_state.navigator.position();
            if view_state.selection.contains(position) {
                view_state.selection.unselect(position)
            } else {
                view_state.selection.select(position)
            }
            view_state.navigator.set_page_changed()
        }
        self.refresh_view()
    }

    fn move_navigator(&self, direction: &Direction) -> Navigator {
        let navigator = {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            if view_state.navigator.can_move(&direction) {
                view_state.navigator.move_towards(&direction);
            }
            view_state.navigator.clone()
        };
        {
            let shared_view_state = self.shared_view_state();
            let mut view_state = shared_view_state.borrow_mut();
            let gallery = &mut view_state.gallery;
            gallery.set_current_picture_index(navigator.position());
        };
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
                    {
                        let shared_view_state = self.shared_view_state();
                        let mut view_state = shared_view_state.borrow_mut();
                        view_state.focus_at_coords = (col as i32, row as i32);
                    }
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
