use crate::gui::main_controller_action::ActionParameterType;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::model::change::Change;
use crate::model::action::Action;
use crate::gui::controller::Controller;
use crate::gui::main_controller::RcController;
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
    controller_opt_rc: RefCell<Option<RcController>>,
}

impl Default for MainController {
    fn default() -> Self {
        Self {
            actions: gtk::gio::SimpleActionGroup::new(),
            controller_opt_rc: RefCell::new(None),
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
        *self.controller_opt_rc.borrow_mut() = controller_opt.clone();

        let controller_rc = controller_opt.unwrap();
        let action_test = ActionEntry::builder("test")
            .activate(clone!( #[strong] controller_rc, move |_obj, _simple_action, variant_opt| {
                println!("todo!");
            }))
            .build();

        let action_change_undefined = Self::make_action_entry(Action::EnterChange(Change::Undefined),
            clone!( #[strong] controller_rc, move |_obj, _simple_action, variant_opt| {
                if let Ok(controller) = controller_rc.try_borrow() {
                    println!("here's the controller's state:\n {:?}", controller.state());
                } else {
                    println!("can't borrow controller_rc");
                }
            }));
        let action_entries = vec![action_test, action_change_undefined];
        println!("initializing {:?}",action_entries);
        self.actions.add_action_entries(action_entries);
        println!("main_controller.actions() = {:?}", self.actions);
    }

    pub fn make_action_entry<F>(action: Action, activate: F,) -> ActionEntry<gtk::gio::SimpleActionGroup>
        where F:Fn(&gtk::gio::SimpleActionGroup, &gtk::gio::SimpleAction, Option<&gtk::glib::Variant>) + 'static,
        {
            let mca = action.main_controller_action();
            let action_entry = ActionEntry::builder(&mca.name())
                .parameter_type(mca.parameter_type().variant_ty())
                .activate(activate)
            .build();
            println!("making ActionEntry: {:?}", action_entry);
            action_entry
        }
}
