use crate::gui::controller::Controller;
use crate::gui::controller::RcController;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::cell::RefCell;
use std::rc::Rc;

mod imp;
pub type RcMainController = Rc<RefCell<MainController>>;

gtk::glib::wrapper! {
    pub struct MainController(ObjectSubclass<imp::MainController>);
}

impl MainController {
    pub fn new(controller_opt: Option<RcController>) -> Self {
        let obj: Self = gtk::glib::Object::new();
        obj.imp().initialize(controller_opt);
        obj
    }

    pub fn actions(&self) -> gtk::gio::SimpleActionGroup {
        self.imp().actions.clone()
    }

    pub fn close(&self) {
        self.emit_by_name::<()>("closed", &[]);
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
