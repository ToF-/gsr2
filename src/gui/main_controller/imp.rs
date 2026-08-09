use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter_type::GioActionParameterType;
use crate::gui::action::gio_action_type::GioActionType;
use crate::gui::controller::Controller;
use crate::gui::main_controller::RcController;
use crate::model::change::Change;
use crate::model::rank::Rank;
use gtk::gio::ActionEntry;
use gtk::gio::prelude::*;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::subclass::Signal;
use gtk::glib::subclass::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::cell::RefCell;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct MainController {
    pub gio_action_group: gtk::gio::SimpleActionGroup,
    controller_opt_rc: RefCell<Option<RcController>>,
}

impl Default for MainController {
    fn default() -> Self {
        Self {
            gio_action_group: gtk::gio::SimpleActionGroup::new(),
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

    pub fn controller_rc_opt(&self) -> Option<RcController> {
        self.controller_opt_rc.borrow().clone()
    }
    pub fn initialize(&self, controller_opt: Option<RcController>) {
        *self.controller_opt_rc.borrow_mut() = controller_opt.clone();

        let mut action_entries = vec![];

        let activate = clone!(
            #[strong]
            controller_opt,
            move |
            _group: &gtk::gio::SimpleActionGroup,
            object: &gtk::gio::SimpleAction,
            variant: Option<&gtk::glib::Variant>,
            | {
                if let Some(controller_rc) = &controller_opt {
                    let controller = controller_rc.borrow();
                    controller.process_action(object, variant);
                } else {
                    println!("controller not set");
                }
            }
        );
        action_entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleThumbnailsView),
            activate.clone(),
        ));
        action_entries.push(Self::action_entry(
            GioActionType::from(Action::Rank(Rank::ThreeStars)),
            activate.clone(),
        ));
        action_entries.push(Self::action_entry(
            GioActionType::from(Action::TogglePalette),
            activate.clone(),
        ));
        self.gio_action_group.add_action_entries(action_entries);
    }

    pub fn action_entry<F>(
        gio_action_ty: GioActionType,
        activate: F,
    ) -> ActionEntry<gtk::gio::SimpleActionGroup>
    where
        F: Fn(&gtk::gio::SimpleActionGroup, &gtk::gio::SimpleAction, Option<&gtk::glib::Variant>)
            + 'static,
    {
        ActionEntry::builder(&gio_action_ty.name())
            .parameter_type(gio_action_ty.parameter_type().variant_ty())
            .activate(activate)
            .build()
    }
}
