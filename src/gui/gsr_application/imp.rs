use gtk::glib;
use gtk::gio::ActionMap;
use gtk::gio::ActionGroup;
use gtk::subclass::prelude::*;
use gtk::subclass::prelude::ApplicationImpl;

#[derive(Default)]
pub struct GsrApplication;

#[glib::object_subclass]
impl ObjectSubclass for GsrApplication {
    const NAME: &'static str = "GsrApplication";
    type Type = super::GsrApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for GsrApplication {}

impl ApplicationImpl for GsrApplication {}

impl ActionGroupImpl for  GsrApplication {}
