use crate::env::default_values::FRAME_WINDOW_NAME;
use crate::env::default_values::GRID_WINDOW_NAME;
use crate::gui::control::Control;
use crate::gui::control::default_controls;
use crate::gui::controller::Controller;
use crate::gui::direction::Direction;
use crate::gui::mode::Mode;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_picture_frame::GsrPictureFrame;
use crate::gui::objects::gsr_picture_grid::GsrPictureGrid;
use crate::gui::view::View;
use crate::gui::view::treelist_view::TreeListView;
use crate::model::catalog::Catalog;
use gtk::glib;
use gtk::glib::Propagation;
use gtk::glib::clone;
use gtk::prelude::*;

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

    pub fn stack(&self) -> gtk::Stack {
        self.first_child()
            .expect("no child on stack")
            .downcast::<gtk::Stack>()
            .expect("can't donwcast stack")
    }

    pub fn frame(&self) -> GsrPictureFrame {
        self.stack()
            .child_by_name(FRAME_WINDOW_NAME)
            .expect("frame scrolled window not set")
            .downcast::<gtk::ScrolledWindow>()
            .expect("can't downcast frame scrolled window")
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
            let binding = self.gsr_application().shared_view();
            let mut view = binding.borrow_mut();
            // TEST SETUP
            view.set_full_size(false);
            view.set_palette_on(true);
            view.set_pictures_per_row(10);
            let pictures_per_row = &view.pictures_per_row();
            if *pictures_per_row == 1 {
                stack.set_visible_child(&frame_scrolled_window);
            } else {
                stack.set_visible_child(&grid_scrolled_window);
            }
        }
        self.gsr_picture_grid().initialize_pictures();
        let gallery = self.gsr_application().shared_gallery().borrow().clone();
        frame.set_current_picture();
        if gallery.len() == 0 {
            self.set_title(Some("gallery is empty"));
        }
        // connect the events
        self.attach_key_pressed_event_handlers();
        // navigate to current position
    }

    // change what is visible  according the given view
    pub fn change_view(&self, new_view: View) {
        {
            let shared_view = self.gsr_application().shared_view();
            let mut view = shared_view.borrow_mut();
            *view = new_view;
            let shared_navigator = self.gsr_application().shared_navigator();
            let mut navigator = shared_navigator.borrow_mut();
            navigator.set_pictures_per_row(view.pictures_per_row() as usize);
        }
        self.gsr_picture_grid().initialize_pictures();
        self.gsr_picture_grid().set_focus_symbol();
    }

    pub fn toggle_palette(&self) {
        {
            let shared_view = self.gsr_application().shared_view();
            let mut view = shared_view.borrow_mut();
            view.toggle_palette_on();
        }
        self.gsr_picture_grid().initialize_pictures();
    }

    pub fn set_pictures_per_row(&self, n: i32) {
        {
            let shared_view = self.gsr_application().shared_view();
            let mut view = shared_view.borrow_mut();
            view.set_pictures_per_row(n);
            let shared_navigator = self.gsr_application().shared_navigator();
            let mut navigator = shared_navigator.borrow_mut();
            navigator.set_pictures_per_row(view.pictures_per_row() as usize);
        }
        self.gsr_picture_grid().initialize_pictures();
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
    pub fn full_size_arrow_move(&self, direction: Direction) {
        let full_size_on = self.gsr_application().shared_view().borrow().full_size_on();
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
    fn attach_key_pressed_event_handlers(&self) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong (rename_to = this)]
            self,
            move |_, key, _key_code, _modifier_type| {
                let key_name = key.name().unwrap_or_default();
                let key_name = key_name.as_str();
                let key_name = key_name.to_string();
                if let Some(control) = default_controls().get(&(key_name, Mode::View)) {
                    match control {
                        Control::Right | Control::Left | Control::Up | Control::Down => {
                            let direction = Direction::from(control.clone());
                            this.full_size_arrow_move(direction)
                        }
                        Control::Quit => {
                            // TEMPORARY, should call a quit action that saves things
                            this.close()
                        }
                        Control::TogglePalette => {
                            this.toggle_palette();
                        }
                        Control::ToggleThumbView => {
                            this.set_pictures_per_row(10);
                        }
                        Control::ToggleTwoByTwoView => {
                            this.set_pictures_per_row(2);
                        }
                        _ => {}
                    }
                }
                Propagation::Proceed
            }
        ));
        self.add_controller(event_controller_key);
    }
    pub fn set_focus_for_current_picture(&self, _controller: &Controller) {
        todo!()
    }

    pub fn set_pictures(&self, _controller: &Controller) {
        todo!()
    }

    pub fn toggle_view_stack(&self, _controller: &Controller) {
        todo!()
    }

    pub fn set_title_for_current_picture(&self, _controller: &Controller) {
        todo!()
    }

    pub fn set_label_text_for_current_picture(
        &self,
        _controller: &Controller,
        _label: Option<char>,
    ) {
        todo!()
    }

    pub fn set_opacity_for_current_picture(&self, _controller: &Controller, _opacity: f64) {
        todo!()
    }

    pub fn single_view(&self) -> bool {
        todo!()
    }

    pub fn reattach_slideshow_event(&self, _seconds: i32) {
        todo!();
    }

    pub fn change_grid_size(&self, _pictures_per_row: i32, _palette_on: bool) {
        todo!();
    }

    pub fn popup_treelist_view(&self, _prompt: &str, _catalog: &Catalog) -> TreeListView {
        todo!()
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
