use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::sync::OnceLock;

pub struct MainController {}

impl Default for MainController {
    fn default() -> Self {
        Self {}
    }
}
#[gtk::glib::object_subclass]
impl ObjectSubclass for MainController {
    const NAME: &'static str = "MainController";
    type Type = super::MainController;
    type ParentType = gtk::glib::Object;
}

impl ObjectImpl for MainController {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("closed").build(),
                Signal::builder("action").build(),
            ]
        })
    }
}
