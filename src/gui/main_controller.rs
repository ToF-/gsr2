use crate::gui::action::Action;
use crate::gui::action::gio_action_type::GioActionType;
use crate::gui::controller::RcController;
use crate::gui::direction::Direction;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::view_option::ViewOption;
use gtk::gio::ActionEntry;
use gtk::gio::prelude::*;
use gtk::glib::clone;
use std::cell::RefCell;
use std::rc::Rc;

pub const MAIN_CONTROLLER_GROUP_NAME: &str = "main-controller";
pub type RcMainController = RefCell<MainController>;

#[derive(Debug, Clone)]
pub struct MainController {
    pub gio_action_group: gtk::gio::SimpleActionGroup,
    pub controller_opt_rc: RefCell<Option<RcController>>,
}

impl Default for MainController {
    fn default() -> Self {
        Self {
            gio_action_group: gtk::gio::SimpleActionGroup::new(),
            controller_opt_rc: RefCell::new(None),
        }
    }
}

impl MainController {
    pub fn new(controller_opt: Option<RcController>) -> Self {
        let obj = Self::default();
        obj.initialize(controller_opt);
        obj
    }

    pub fn gio_action_group(&self) -> gtk::gio::SimpleActionGroup {
        self.gio_action_group.clone()
    }

    pub fn controller_rc_opt(&self) -> Option<RcController> {
        self.controller_opt_rc.borrow().clone()
    }
    // LAW
    pub fn initialize(&self, controller_opt: Option<RcController>) {
        *self.controller_opt_rc.borrow_mut() = controller_opt.clone();

        let mut entries = vec![];

        let activate = clone!(
            #[strong]
            controller_opt,
            move |_group: &gtk::gio::SimpleActionGroup,
                  object: &gtk::gio::SimpleAction,
                  variant: Option<&gtk::glib::Variant>| {
                if let Some(controller_rc) = &controller_opt {
                    let controller = controller_rc.borrow_mut();
                    dbg!(&object.name(), &variant);
                    controller.process_action(object, variant);
                } else {
                    println!("controller not set");
                }
            }
        );

        entries.push(Self::action_entry(
            GioActionType::from(Action::ApplyOrderSetting(Order::Name)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ApplyViewSetting(ViewOption::Grid2x2)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Cancel),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Dismiss),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterLabel),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::FocusAt(0, 0)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::GotoDirectory),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Label("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::MoveTowards(Direction::Left)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Nothing),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::PickChange),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::PickOrderSetting),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::PickViewOption),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Quit),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::QuitDirectory),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Rank(Rank::ThreeStars)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::TogglePalette),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleSelectedAt(0, 0)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleSingleView),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleThumbnailsView),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleTwoByTwoView),
            activate.clone(),
        ));

        self.gio_action_group.add_action_entries(entries);
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
