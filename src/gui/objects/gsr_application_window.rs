use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::default_values::APPLICATION_NAME;
use crate::env::default_values::FRAME_WINDOW_NAME;
use crate::env::default_values::GRID_WINDOW_NAME;
use crate::gui::controller::Controller;
use crate::gui::direction::Direction;
use crate::gui::navigator::Navigator;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_picture_grid::GsrPictureGrid;
use crate::gui::view::View;
use crate::gui::view::picture_frame::PictureFrame;
use crate::gui::view::treelist_view::TreeListView;
use crate::model::catalog::Catalog;
use crate::model::gallery::Gallery;
use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;

use gtk::subclass::prelude::*;
use std::cell::RefCell;

pub const LEFT_PANE: usize = 0;
pub const RIGHT_PANE: usize = 1;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct GsrApplicationWindow {
        pub view: RefCell<View>,
        pub navigator: RefCell<Navigator>,
        pub gallery: RefCell<Gallery>,
    }

    #[gtk::glib::object_subclass]
    impl ObjectSubclass for GsrApplicationWindow {
        const NAME: &'static str = "GsrApplicationWindow";
        type Type = super::GsrApplicationWindow;
        type ParentType = gtk::ApplicationWindow;
    }

    impl ObjectImpl for GsrApplicationWindow {}

    impl WidgetImpl for GsrApplicationWindow {}

    impl WindowImpl for GsrApplicationWindow {}

    impl ApplicationWindowImpl for GsrApplicationWindow {}
}

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
    pub fn new(application: &GsrApplication, clargs: &CommandLineArguments) -> Self {
        let obj: Self = glib::Object::builder()
            .property("application", application)
            .property("title", Some(APPLICATION_NAME))
            .property("default_width", clargs.width.unwrap())
            .property("default_height", clargs.height.unwrap())
            .build();
        let view = RefCell::new(View::default());
        let navigator = RefCell::new(Navigator::default());
        let gallery = RefCell::new(Gallery::default());
        obj.initialize(view, navigator, gallery);
        obj
    }

    fn initialize(
        &self,
        view: RefCell<View>,
        navigator: RefCell<Navigator>,
        gallery: RefCell<Gallery>,
    ) {
        dbg!("GsrApplication::initialize");
        // register the shared tools
        *self.imp().view.borrow_mut() = view.borrow().clone();
        *self.imp().navigator.borrow_mut() = navigator.borrow().clone();
        *self.imp().gallery.borrow_mut() = gallery.borrow().clone();
        // build the components
        let frame = PictureFrame::new().frame(); // TODO make PictureFrame a subclass of gkt::Box 
        let frame_scrolled_window = make_scrolled_window_with_child(&frame);
        let gsr_picture_grid = GsrPictureGrid::new(view, navigator, gallery);
        let panel = make_panel_with_child(&gsr_picture_grid);
        let grid_scrolled_window = make_scrolled_window_with_child(&panel);
        let stack = gtk::Stack::builder().hexpand(true).vexpand(true).build();
        let _ = stack.add_named(&frame_scrolled_window, Some(FRAME_WINDOW_NAME));
        let _ = stack.add_named(&grid_scrolled_window, Some(GRID_WINDOW_NAME));
        self.set_child(Some(&stack));
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
