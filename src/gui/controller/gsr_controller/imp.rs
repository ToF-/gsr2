use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::sync::OnceLock;

#[derive(Default)]
pub struct GsrController;

#[gtk::glib::object_subclass]
impl ObjectSubclass for GsrController {
    const NAME: &'static str = "GsrController";
    type Type = super::GsrController;
    type ParentType = gtk::glib::Object;
}

impl ObjectImpl for GsrController {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("entered")
                    .param_types([gtk::glib::types::Type::STRING])
                    .build(),
            ]
        })
    }
}
