use gtk::glib::subclass::prelude::*;
use gtk::glib::subclass::Signal;
use std::sync::OnceLock;
use gtk::glib;

#[derive(Default)]
pub struct MyController;

#[gtk::glib::object_subclass]
impl ObjectSubclass for MyController {
    const NAME: &'static str = "MyController";
    type Type = super::MyController;
    type ParentType = gtk::glib::Object;
}

impl ObjectImpl for MyController {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("finished")
                    .param_types([
                        gtk::glib::types::Type::STRING,
                    ])
                    .build(),
            ]
        })
    }
}
