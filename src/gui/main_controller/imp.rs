use crate::gui::main_controller::RcController;
use crate::gui::controller::Controller;
use gtk::gio::ActionEntry;
use gtk::gio::prelude::*;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::cell::RefCell;
use std::sync::OnceLock;

pub struct MainController {
    pub actions: gtk::gio::SimpleActionGroup,
    controller_opt_rc : RefCell<Option<RcController>>, 
}

impl Default for MainController {
    fn default() -> Self {
        Self {
            actions: gtk::gio::SimpleActionGroup::new(),
            controller_opt_rc : RefCell::new(None),
        }
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

impl MainController {
    pub fn new(controller_opt: Option<RcController>) -> Self {
        let obj = Self::default();
        obj.initialize(controller_opt);
        obj
    }

    pub fn initialize(&self, controller_opt: Option<RcController>) {
        *self.controller_opt_rc.borrow_mut() = controller_opt;

        let action_test = ActionEntry::builder("test")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |_obj, _simple_action, variant_opt| {
                let value = variant_opt
                    .expect("could not get parameter")
                    .get::<String>()
                    .expect("the variant need to be of type string");
                println!("controller.test with value {:?}", value);
            })
            .build();

        let action_entries = vec![action_test];
        self.actions.add_action_entries(action_entries);

    }
}
