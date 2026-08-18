use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct GsrApplicationWindow {
        pub document_id: RefCell<Option<String>>,
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
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder()
            .property("application", app)
            .build()
    }

    pub fn set_document_id(&self, id: impl Into<String>) {
        self.imp().document_id.replace(Some(id.into()));
    }

    pub fn document_id(&self) -> Option<String> {
        self.imp().document_id.borrow().clone()
    }
}
