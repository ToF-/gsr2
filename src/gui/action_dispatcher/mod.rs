use crate::gui::controller::RcController;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::cell::RefCell;
use std::rc::Rc;

mod imp;
pub type RcActionDispatcher = Rc<RefCell<ActionDispatcher>>;

gtk::glib::wrapper! {
    pub struct ActionDispatcher(ObjectSubclass<imp::ActionDispatcher>);
}

impl ActionDispatcher {
    pub fn new() -> Self {
        let obj: Self = gtk::glib::Object::new();
        obj.imp().initialize();
        obj
    }

    pub fn set_rc_controller(&self, controller_rc: RcController) {
        self.imp().set_rc_controller(controller_rc)
    }
    pub fn actions(&self) -> gtk::gio::SimpleActionGroup {
        self.imp().actions.clone()
    }
    pub fn help_command(&self, key_name: &str) {
        self.emit_by_name::<()>("help-command", &[&key_name]);
    }

    pub fn close(&self) {
        self.emit_by_name::<()>("closed", &[]);
    }

    pub fn connect_key_pressed<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&Self, &str) + 'static,
    {
        self.connect_local("key-pressed", false, move |values| {
            let obj = values[0].get::<ActionDispatcher>().unwrap();
            let key_name = values[0].get::<&str>().unwrap();
            f(&obj, key_name);
            None
        })
    }
    pub fn connect_closed<F>(&self, f: F) -> gtk::glib::SignalHandlerId
    where
        F: Fn(&ActionDispatcher) + 'static,
    {
        self.connect_local("closed", false, move |values| {
            let obj = values[0].get::<ActionDispatcher>().unwrap();
            f(&obj);
            None
        })
    }
}
