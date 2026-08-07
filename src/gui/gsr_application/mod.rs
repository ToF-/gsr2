use gtk::prelude::*;
use crate::env::default_values::APPLICATION_ID;
mod imp;

use glib::Object;
use gtk::glib;
use gtk::gio;

glib::wrapper! {
    pub struct GsrApplication(ObjectSubclass<imp::GsrApplication>)
        @extends gio::Application, gtk::Application,
        @implements gtk::gio::ActionGroup, gtk::gio::ActionMap;
}

impl GsrApplication {
    pub fn new(application_id: &str) -> Self {
        Object::builder()
            .property("application-id", application_id)
            .build()
    }
}

