use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::Configuration;
use crate::env::default_values::APPLICATION_NAME;
use crate::env::default_values::FRAME_WINDOW_NAME;
use crate::env::default_values::GRID_WINDOW_NAME;
use crate::gui::controller::Controller;
use crate::gui::direction::Direction;
use crate::gui::main_controller::MainController;
use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_picture_frame::GsrPictureFrame;
use crate::gui::objects::gsr_picture_grid::GsrPictureGrid;
use crate::gui::view::View;
use crate::gui::view::picture_frame::PictureFrame;
use crate::gui::view::treelist_view::TreeListView;
use crate::model::catalog::Catalog;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

use gtk::subclass::prelude::*;
use std::cell::RefCell;

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

impl GsrApplicationWindow {
    pub fn new(application: &GsrApplication) -> Self {
        let obj = glib::Object::builder()
            .property("application", application)
            .build();
        obj
    }

    pub fn shared_view(&self) -> Shared<View> {
        (*self.imp().view.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_navigator(&self) -> Shared<Navigator> {
        (*self.imp().navigator.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_gallery(&self) -> Shared<Gallery> {
        (*self.imp().gallery.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_command_line_arguments(&self) -> Shared<CommandLineArguments> {
        (*self.imp().command_line_arguments.borrow())
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn shared_configuration(&self) -> Shared<Configuration> {
        (*self.imp().configuration.borrow())
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn shared_main_controller(&self) -> Shared<MainController> {
        (*self.imp().main_controller.borrow())
            .as_ref()
            .unwrap()
            .clone()
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

    pub fn initialize(&self) {
        dbg!("GsrApplication::initialize");
        let command_line_arguments = self.shared_command_line_arguments().borrow().clone();
        self.set_default_width(command_line_arguments.width.unwrap());
        self.set_default_height(command_line_arguments.height.unwrap());
        // build the components
        let frame = GsrPictureFrame::new(
            self.shared_view(),
            self.shared_navigator(),
            self.shared_gallery(),
        );
        let frame_scrolled_window = make_scrolled_window_with_child(&frame);
        let gsr_picture_grid = GsrPictureGrid::new(
            self.shared_view(),
            self.shared_navigator(),
            self.shared_gallery(),
        );
        let panel = make_panel_with_child(&gsr_picture_grid);
        let grid_scrolled_window = make_scrolled_window_with_child(&panel);
        let stack = gtk::Stack::builder().hexpand(true).vexpand(true).build();
        let _ = stack.add_named(&frame_scrolled_window, Some(FRAME_WINDOW_NAME));
        let _ = stack.add_named(&grid_scrolled_window, Some(GRID_WINDOW_NAME));
        self.set_child(Some(&stack));
        let pictures_per_row = self.shared_view().borrow().pictures_per_row();
        if pictures_per_row == 1 {
            stack.set_visible_child(&frame_scrolled_window);
        } else {
            stack.set_visible_child(&grid_scrolled_window);
        }
        let gallery = self.shared_gallery().borrow().clone();
        {
            let view_rc = self.imp().view.borrow().as_ref().unwrap().clone();
            let mut view = view_rc.borrow_mut();
            view.set_full_size(false);
            view.set_palette_on(true);
        }
        frame.set_current_picture();
        if gallery.len() == 0 {
            self.set_title(Some("gallery is empty"));
        }
        // connect the events
        // navigate to current position
    }

    // change what is visible  according the the state of view
    pub fn change_view(&self) {
        todo!()
    }

    pub fn set_focus_for_current_picture(&self, controller: &Controller) {
        todo!()
    }

    pub fn set_pictures(&self, controller: &Controller) {
        todo!()
    }

    pub fn toggle_view_stack(&self, controller: &Controller) {
        todo!()
    }

    pub fn set_title_for_current_picture(&self, controller: &Controller) {
        todo!()
    }

    pub fn set_label_text_for_current_picture(&self, controller: &Controller, label: Option<char>) {
        todo!()
    }

    pub fn set_opacity_for_current_picture(&self, controller: &Controller, opacity: f64) {
        todo!()
    }

    pub fn single_view(&self) -> bool {
        todo!()
    }

    pub fn reattach_slideshow_event(&self, seconds: i32) {
        todo!();
    }

    pub fn change_grid_size(&self, pictures_per_row: i32, palette_on: bool) {
        todo!();
    }

    pub fn full_size_arrow_move(&self, direction: Direction) {
        todo!();
    }

    pub fn popup_treelist_view(&self, prompt: &str, catalog: &Catalog) -> TreeListView {
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

fn make_stack() -> gtk::Stack {
    gtk::Stack::builder().hexpand(true).vexpand(true).build()
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
