use crate::gui::controller::RcController;
use crate::model::find::Find;
use gtk::gio;
use gtk::gio::ActionEntry;
use gtk::gio::prelude::*;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use std::cell::RefCell;
use std::sync::OnceLock;

pub struct ActionDispatcher {
    pub actions: gtk::gio::SimpleActionGroup,
    controller_opt_rc: RefCell<Option<RcController>>,
}

impl Default for ActionDispatcher {
    fn default() -> Self {
        Self {
            actions: gtk::gio::SimpleActionGroup::new(),
            controller_opt_rc: RefCell::new(None),
        }
    }
}
#[gtk::glib::object_subclass]
impl ObjectSubclass for ActionDispatcher {
    const NAME: &'static str = "ActionDispatcher";
    type Type = super::ActionDispatcher;
    type ParentType = gtk::glib::Object;
}

impl ObjectImpl for ActionDispatcher {
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
impl ActionDispatcher {
    pub fn new() -> Self {
        let obj = Self::default();
        obj.initialize();
        obj
    }
    pub fn set_rc_controller(&self, controller_rc: RcController) {
        *self.controller_opt_rc.borrow_mut() = Some(controller_rc)
    }
    pub fn initialize(&self) {
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

        let controller_opt_rc = self.controller_opt_rc.clone();
        let action_find_label = ActionEntry::builder("find-label")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(clone!(
                #[strong]
                controller_opt_rc,
                move |_obj, _simple_action, variant_opt| {
                    let value = variant_opt
                        .expect("could not get parameter")
                        .get::<String>()
                        .expect("the variant need to be of type string");
                    if let Ok(controller_rc_opt) = controller_opt_rc.try_borrow() {
                        if let Some(controller_rc) = controller_rc_opt.clone() {
                            if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                                controller.find_first(&value, Find::Label);
                            } else {
                                println!("can't borrow mutably controller");
                            }
                        } else {
                            println!("controller_rc is not set")
                        }
                    } else {
                        println!("can't borrow controller_rc_opt");
                    }
                }
            ))
            .build();

        let action_entries = vec![action_test, action_find_label];
        self.actions.add_action_entries(action_entries);
    }
}
