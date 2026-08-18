use crate::gui::navigator::Navigator;
use std::cell::Cell;
use crate::cli::command_line_arguments::CommandLineArguments;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::env::default_values::APPLICATION_NAME;
use gtk::glib;

use gtk::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct GsrApplicationWindow {
        pub navigator: Option<RefCell<Navigator>>,
        pub palette_on: Cell<bool>,
        pub pictures_per_row: Cell<i32>, 
        pub last_pictures_per_row: Cell<i32>, 
        pub full_size_on: Cell<bool>,
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
            .property("title", (Some(APPLICATION_NAME)))
            .property("default_width", clargs.width.unwrap())
            .property("default_height", clargs.height.unwrap())
            .build();
        obj.initialize();
        obj
    }

    fn initialize(&self) {
        // build the components
        // self.set_child(Some(view_stack));
        // connect the events
        // navigate to current position
    }

    pub fn toggle_palette_on(&self) {
        self.imp().palette_on.set(! self.imp().palette_on.get());
    }

    pub fn palette_on(&self) -> bool {
        self.imp().palette_on.get()
    }

    pub fn toggle_pictures_per_row(&self, n: i32) {
        if self.imp().pictures_per_row.get() != n {
            self.imp().last_pictures_per_row.set(n);
            self.imp().pictures_per_row.swap(
                &self.imp().last_pictures_per_row);
        }
    }

    pub fn pictures_per_row(&self) -> i32 {
        self.imp().pictures_per_row.get()
    }

    pub fn set_full_size_on(&self, yes_no: bool) {
        self.imp().full_size_on.set(yes_no)
    }

    pub fn full_size_on(&self) -> bool {
        self.imp().full_size_on.get()
    }

}
