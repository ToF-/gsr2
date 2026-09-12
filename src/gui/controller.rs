use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::action::gio_action_type::GioActionType;
use crate::gui::direction::Direction;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use crate::gui::objects::gsr_treelist_window::GsrTreelistWindow;
use crate::model::find::Find;
use crate::model::order::Order;
use crate::model::rank::Rank;
use crate::model::shared::Shared;
use crate::model::view_option::ViewOption;
use gtk::gio::ActionEntry;
use gtk::gio::prelude::*;
use gtk::glib::clone;
use std::cell::RefCell;

pub const MAIN_CONTROLLER_GROUP_NAME: &str = "main-controller";
pub type RcController = RefCell<Controller>;

#[derive(Debug, Clone)]
pub struct Controller {
    pub gio_action_group: gtk::gio::SimpleActionGroup,
    pub gsr_application_window: Option<Shared<GsrApplicationWindow>>,
    pub gsr_entry_window: Option<Shared<GsrEntryWindow>>,
    pub gsr_treelist_window: Option<Shared<GsrTreelistWindow>>,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            gio_action_group: gtk::gio::SimpleActionGroup::new(),
            gsr_application_window: None,
            gsr_entry_window: None,
            gsr_treelist_window: None,
        }
    }
}

impl Controller {
    pub fn new() -> Self {
        let obj = Self::default();
        obj.initialize();
        obj
    }

    pub fn gio_action_group(&self) -> gtk::gio::SimpleActionGroup {
        self.gio_action_group.clone()
    }

    pub fn set_application_window(&mut self, gsr_application_window: Shared<GsrApplicationWindow>) {
        self.gsr_application_window = Some(gsr_application_window);
    }
    // LAW
    pub fn initialize(&self) {
        let mut entries = vec![];
        let shared_gsr_application_window = self
            .gsr_application_window
            .as_ref()
            .expect("application window not set in main controller");

        let activate = clone!(
            #[strong]
            shared_gsr_application_window,
            move |_group: &gtk::gio::SimpleActionGroup,
                  object: &gtk::gio::SimpleAction,
                  variant: Option<&gtk::glib::Variant>| {
                let gsr_application_window = shared_gsr_application_window.borrow();
                gsr_application_window.process_gio_action(object, variant);
            }
        );

        entries.push(Self::action_entry(
            GioActionType::from(Action::AddCategory("foo".to_string(), "bar".to_string())),
            activate.clone(),
        ));
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
            GioActionType::from(Action::DeleteSelectedPicture("yes".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Dismiss),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterFind(Find::Name)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterSelect(Find::Name)),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterLabel),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Find(Find::Name, "foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Select(Find::Name, "foo".to_string())),
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
            GioActionType::from(Action::AddTag("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::RemoveTag("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Rename("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::MoveSelectedPicture("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Categorize(None)),
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
            clone!(
                #[strong]
                shared_gsr_application_window,
                move |_group: &gtk::gio::SimpleActionGroup,
                      object: &gtk::gio::SimpleAction,
                      variant: Option<&gtk::glib::Variant>| {
                    let gio_action = GioAction::from((object, variant));
                    if let Action::Rank(rank) = Action::from(gio_action) {
                        let gsr_application_window = shared_gsr_application_window.borrow();
                        gsr_application_window.dismiss();
                        let indices = gsr_application_window.selected_indices();
                        for position in indices {
                            gsr_application_window.with_view_state_mut(|view_state| {
                                let mut picture = view_state.gallery.picture(position);
                                picture.set_rank(rank);
                                gsr_application_window.with_repository(
                                    |repository| match repository.update_picture(&picture) {
                                        Ok(_) => {}
                                        Err(e) => eprintln!("{}", e),
                                    },
                                );
                                view_state.gallery.set_picture(position, picture);
                            });
                        }
                        gsr_application_window.deselect_pictures();
                    }
                }
            ),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::TogglePalette),
            clone!(
                #[strong]
                shared_gsr_application_window,
                move |_, _, _| {
                    let gsr_application_window = shared_gsr_application_window.borrow();
                    {
                        let binding = gsr_application_window.gsr_application().shared_view_state();
                        let mut view_state = binding.borrow_mut();
                        view_state.settings.toggle_palette();
                    }
                    gsr_application_window.refresh_view();
                }
            ),
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
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterRename),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::PickCatalogChange),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::SelectCategoryForPicture),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::SelectCategoryToMove),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::SelectCategoryToRemove),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::SelectCategoryAddTarget("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::SelectCategoryMoveTarget("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterNewCategory),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterRemoveTag),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::EnterAddTag),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::MoveCategory("foo".to_string(), "bar".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::RemoveCategory("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleCover),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Unlabel),
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
