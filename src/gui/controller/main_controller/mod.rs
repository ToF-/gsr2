use gtk::glib::subclass::prelude::*;
use gtk::prelude::ObjectExt;
use std::cell::RefCell;
use std::rc::Rc;

mod imp;
pub type RcMainController = Rc<RefCell<MainController>>;

gtk::glib::wrapper! {
    pub struct MainController(ObjectSubclass<imp::MainController>);
}

impl MainController {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn key_pressed(&self, key_name: &str) {
        self.emit_by_name::<()>("key-pressed", &[&key_name]);
    }

    pub fn close(&self) {
        self.emit_by_name::<()>("closed", &[]);
    }

    pub fn connect_key_pressed<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&Self, &str) + 'static,
    {
        self.connect_local("key-pressed", false, move |values| {
            let obj = values[0].get::<MainController>().unwrap();
            let key_name = values[0].get::<&str>().unwrap();
            f(&obj, key_name);
            None
        })
    }
    pub fn connect_closed<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&MainController) + 'static,
    {
        self.connect_local("closed", false, move |values| {
            let obj = values[0].get::<MainController>().unwrap();
            f(&obj);
            None
        })
    }
}
