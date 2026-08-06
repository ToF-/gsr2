use crate::model::rank::Rank;
use crate::gui::controller::Controller;
use crate::gui::main_controller::RcController;
use crate::gui::gio_action::GioActionParameterType;
use crate::model::action::Action;
use crate::model::change::Change;
use gtk::gio::ActionEntry;
use gtk::gio::prelude::*;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
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

        let mut action_entries = vec![];


        // action_entries.push(Self::make_parameterless_action_entry(Action::AddCategory("foo".to_string(), "bar".to_string()), &controller_rc));
        action_entries.push(Self::make_string_parameter_action_entry(Action::Label("foo".to_string()), &controller_rc));
        action_entries.push(Self::make_int32_parameter_action_entry(Action::Rank(Rank::ThreeStars), &controller_rc));
        action_entries.push(Self::make_parameterless_action_entry(Action::PickChange, &controller_rc));
        action_entries.push(Self::make_parameterless_action_entry(Action::PickViewOption, &controller_rc));
        action_entries.push(Self::make_parameterless_action_entry(Action::PickOrderSetting, &controller_rc));
        println!("initializing {:?}", action_entries);
        self.actions.add_action_entries(action_entries);
    }

    pub fn make_action_entry<F>(
        action: Action,
        activate: F,
    ) -> ActionEntry<gtk::gio::SimpleActionGroup>
    where
        F: Fn(&gtk::gio::SimpleActionGroup, &gtk::gio::SimpleAction, Option<&gtk::glib::Variant>)
            + 'static,
    {
        println!("make_action_entry({action:?})");
        let mca = action.gio_action();
        let action_entry = ActionEntry::builder(&mca.name())
            .parameter_type(mca.parameter_type().variant_ty())
            .activate(activate)
            .build();
        println!("{action_entry:?}");
        action_entry
    }

    pub fn make_parameterless_action_entry(action: Action, controller_rc: &RcController) -> ActionEntry<gtk::gio::SimpleActionGroup> {
        let sample = action.clone();
        Self::make_action_entry(
            action,
            clone!(
                #[strong]
                sample,
                #[strong]
                controller_rc,
                move |_, object, variant| {
                    if let Ok(_) = controller_rc.try_borrow() {
                        println!("object:{object:?},object.name:{0:?},\nvariant:{variant:?}\nsample:{sample:?}", object.name())
                    } else {
                        println!("can't borrow controller_rc");
                    }
                }
            ),
        )
    }

    pub fn make_string_parameter_action_entry(action: Action, controller_rc: &RcController) -> ActionEntry<gtk::gio::SimpleActionGroup> {
        let sample = action.clone();
        Self::make_action_entry(
            action,
            clone!(
                #[strong]
                sample,
                #[strong]
                controller_rc,
                move |_, object, variant| {
                    if let Ok(_) = controller_rc.try_borrow() {
                        println!("object:{object:?},object.name:{0:?},\nvariant:{variant:?}\nsample:{sample:?}", object.name());
                        let parameter: String = variant
                            .expect("can't unwrap variant parameter")
                            .get::<String>().expect("can't get parameter value");
                        let action = match sample {
                            Action::Label(_) => Action::Label(parameter),
                            _ => Action::Nothing,
                        };
                        println!("ready to lauch {action:?}");
                    } else {
                        println!("can't borrow controller_rc");
                    }
                }
            ),
        )
    }
    pub fn make_int32_parameter_action_entry(action: Action, controller_rc: &RcController) -> ActionEntry<gtk::gio::SimpleActionGroup> {
        let sample = action.clone();
        Self::make_action_entry(
            action,
            clone!(
                #[strong]
                sample,
                #[strong]
                controller_rc,
                move |_, object, variant| {
                    if let Ok(_) = controller_rc.try_borrow() {
                        println!("object:{object:?},object.name:{0:?},\nvariant:{variant:?}\nsample:{sample:?}", object.name());
                        let parameter: i32 = variant
                            .expect("can't unwrap variant parameter")
                            .get::<i32>().expect("can't get parameter value");
                        let action = match sample {
                            Action::Rank(_) => Action::Rank(Rank::from(parameter as i64)),
                            _ => Action::Nothing,
                        };
                        println!("ready to lauch {action:?}");
                    } else {
                        println!("can't borrow controller_rc");
                    }
                }
            ),
        )
    }
}
