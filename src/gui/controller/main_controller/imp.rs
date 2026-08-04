use gtk::gio;
use gtk::gio::ActionEntry;
use gtk::gio::prelude::*;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::sync::OnceLock;

pub struct MainController {
    pub actions: gtk::gio::SimpleActionGroup,
}

impl Default for MainController {
    fn default() -> Self {
        Self {
            actions: gtk::gio::SimpleActionGroup::new(),
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
    pub fn new() -> Self {
        let obj = Self::default();
        obj.initialize();
        obj
    }
    pub fn initialize(&self) {
        let action_test = ActionEntry::builder("test")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |_obj, _simple_action, variant_opt| {
                let value = variant_opt
                    .expect("could not get parameter")
                    .get::<String>()
                    .expect("the variant need to be of type string");
                println!("action parameter: {:?}", value)
            })
            .build();
        println!("{:?}", action_test);
        let action_entries = vec![action_test];
        self.actions.add_action_entries(action_entries);
        // self.add_action_entries([action_test]);
    }
}
