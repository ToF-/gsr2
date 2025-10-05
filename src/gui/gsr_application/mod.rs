mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct GsrApplication(ObjectSubclass<imp::GsrApplication>)
        @extends gtk::Application,
        @implements gtk::gio::ActionGroup, gtk::gio::ActionMap;
}

impl GsrApplication {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
