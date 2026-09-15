use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::CONFIGURATION;
use crate::env::configuration::Configuration;
use crate::file::paths::parent_directory;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::action::gio_action_type::GioActionType;
use crate::gui::control::Control;
use crate::gui::direction::Direction;
use crate::gui::key_input::menu::change_menu;
use crate::gui::key_input::menu::order_menu;
use crate::gui::key_input::menu::view_menu;
use crate::gui::objects::gsr_application::GsrApplication;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use crate::gui::objects::gsr_treelist_window::GsrTreelistWindow;
use crate::gui::view_state::ViewState;
use crate::gui::view_state::navigator::Navigator;
use crate::model::find::Find;
use crate::model::gallery::Gallery;
use crate::model::order::Order;
use crate::model::predicate::Predicate;
use crate::model::rank::Rank;
use crate::model::repository::Repository;
use crate::model::shared::Shared;
use crate::model::view_option::ViewOption;
use gtk::gio::ActionEntry;
use gtk::gio::SimpleAction;
use gtk::gio::SimpleActionGroup;
use gtk::gio::prelude::*;
use gtk::glib::Variant;
use gtk::glib::clone;
use std::cell::RefCell;
use std::io::Result as IOResult;
use std::rc::Rc;

pub const MAIN_CONTROLLER_GROUP_NAME: &str = "main-controller";
pub type RcController = RefCell<Controller>;
pub type FnGioAction =
    Box<dyn Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static>;
#[derive(Debug, Clone)]
pub struct Controller {
    pub gio_action_group: SimpleActionGroup,
    pub gsr_application_window: Option<Shared<GsrApplicationWindow>>,
    pub gsr_entry_window: Option<Shared<GsrEntryWindow>>,
    pub gsr_treelist_window: Option<Shared<GsrTreelistWindow>>,
    pub last_action: Shared<Action>,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            gio_action_group: SimpleActionGroup::new(),
            gsr_application_window: None,
            gsr_entry_window: None,
            gsr_treelist_window: None,
            last_action: Rc::new(RefCell::new(Action::Nothing)),
        }
    }
}

impl Controller {
    pub fn new() -> Self {
        let obj = Self::default();
        obj.initialize();
        obj
    }

    pub fn gio_action_group(&self) -> SimpleActionGroup {
        self.gio_action_group.clone()
    }

    pub fn set_application_window(&mut self, gsr_application_window: Shared<GsrApplicationWindow>) {
        self.gsr_application_window = Some(gsr_application_window);
    }

    pub fn gsr_application(&self) -> GsrApplication {
        self.gsr_application_window
            .as_ref()
            .unwrap()
            .borrow()
            .gsr_application()
    }

    pub fn command_line_arguments(&self) -> CommandLineArguments {
        self.gsr_application()
            .shared_command_line_arguments()
            .borrow()
            .clone()
    }
    pub fn with_view_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&ViewState) -> R,
    {
        let shared_view_state = self.gsr_application().shared_view_state();
        let view_state = shared_view_state.borrow();

        f(&view_state)
    }

    pub fn with_view_state_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut ViewState) -> R,
    {
        let shared_view_state = self.gsr_application().shared_view_state();
        let mut view_state = shared_view_state.borrow_mut();

        f(&mut view_state)
    }

    pub fn with_repository<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Repository) -> R,
    {
        let shared_repository_opt = self.gsr_application().shared_repository_opt();
        let binding = shared_repository_opt.borrow();
        let repository = binding.as_ref().unwrap();
        f(&repository)
    }

    // LAW
    pub fn initialize(&self) {
        let mut entries = vec![];
        let shared_window = self
            .gsr_application_window
            .as_ref()
            .expect("application shared_window not set in main controller");
        let window = shared_window.borrow();

        let activate = clone!(
            #[strong]
            shared_window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gsr_application_shared_window = shared_window.borrow();
                dbg!(&object.name());
                gsr_application_shared_window.process_gio_action(object, variant);
            }
        );

        entries.push(Self::action_entry(
            GioActionType::from(Action::AddCategory("foo".to_string(), "bar".to_string())),
            self.add_category_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ApplyOrderSetting(Order::Name)),
            self.apply_order_setting_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ApplyViewSetting(ViewOption::Grid2x2)),
            self.apply_view_setting_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Cancel),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::CancelSelectionRange),
            self.cancel_selection_range_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Categorize(Some("foo".to_string()))),
            self.categorize_action(window.clone()),
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
            self.goto_directory_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Label("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::AddTag("foo".to_string())),
            self.add_tag_action(window.clone()),
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
            self.pick_change_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::PickOrderSetting),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::PickViewOption),
            self.pick_view_option_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Quit),
            self.quit_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::QuitDirectory),
            self.quit_directory_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::Rank(Rank::ThreeStars)),
            self.rank_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::RepeatAction),
            self.repeat_last_action_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::RepeatRangeSelection),
            self.repeat_range_selection_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleCoversView),
            self.toggle_covers_view_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::TogglePalette),
            self.toggle_palette_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleSelected(0)),
            self.toggle_selected_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::TogglePicturesPerRow(1)),
            self.toggle_pictures_per_row_action(window.clone()),
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
            GioActionType::from(Action::PickChange),
            self.pick_change_action(window.clone()),
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
            GioActionType::from(Action::PickOrderSetting),
            self.pick_order_setting_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::RemoveCategory("foo".to_string())),
            activate.clone(),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleBlinking),
            self.toggle_blinking_action(window.clone()),
        ));
        entries.push(Self::action_entry(
            GioActionType::from(Action::ToggleExpand),
            self.toggle_expand_action(window.clone()),
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
    ) -> ActionEntry<SimpleActionGroup>
    where
        F: Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static,
    {
        ActionEntry::builder(&gio_action_ty.name())
            .parameter_type(gio_action_ty.parameter_type().variant_ty())
            .activate(activate)
            .build()
    }

    fn retrieve_from_repository(
        &self,
        covers_only_opt: Option<bool>,
        sub_directory: Option<String>,
        predicate_opt: Option<Predicate>,
    ) -> IOResult<usize> {
        {
            let initial_command_line_arguments = self.command_line_arguments();
            let command_line_arguments = CommandLineArguments {
                covers: covers_only_opt.unwrap_or_default(),
                directory: sub_directory,
                ..initial_command_line_arguments
            };
            let configuration = CONFIGURATION.get().expect("configuration not set");

            let repository =
                Repository::new(configuration.clone(), command_line_arguments.clone(), false);

            match repository.retrieve_pictures(predicate_opt) {
                Err(e) => Err(e),
                Ok(0) => Ok(0),
                Ok(n) => {
                    let repository_gallery = repository.gallery_rc().borrow_mut();
                    self.with_view_state_mut(|view_state| {
                        view_state.navigator = Navigator::new(
                            repository_gallery.len(),
                            view_state.settings.pictures_per_row() as usize,
                        );
                        view_state.gallery = Gallery::from_gallery_and_navigator(
                            repository_gallery.clone(),
                            &view_state.navigator,
                        );
                    });
                    Ok(n)
                }
            }
        }
    }

    // ACTIONS

    fn add_category_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gio_action = GioAction::from((object, variant));
                let action = Action::from(gio_action);
                if let Action::AddCategory(new_category_name, target_category_name) = action {
                    let result = this.with_repository(|repository| {
                        repository.add_category(&new_category_name, &target_category_name)
                    });
                    match result {
                        Ok(_) => {}
                        Err(e) => window.present_information(&format!("{}", e)),
                    }
                    window.dismiss();
                }
            }
        )
    }
    fn add_tag_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gio_action = GioAction::from((object, variant));

                let action = Action::from(gio_action);
                if let Action::AddTag(tags) = action {
                    let tags: Vec<String> = tags.split(',').map(|s| s.to_string()).collect();
                    let indices = window.selected_indices();
                    for position in indices {
                        this.with_view_state_mut(|view_state| {
                            let mut picture = view_state.gallery.picture(position);
                            tags.iter().for_each(|tag| {
                                picture.add_tag(tag);
                                this.with_repository(|repository| {
                                    match repository.update_picture(&picture) {
                                        Ok(_) => {}
                                        Err(e) => eprintln!("{}", e),
                                    }
                                })
                            });
                            view_state.gallery.set_picture(position, picture);
                        });
                    }
                    window.dismiss();
                    window.deselect_pictures();
                }
            }
        )
    }

    fn apply_view_setting_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gio_action = GioAction::from((object, variant));

                window.dismiss();
                let action = Action::from(gio_action);
                if let Action::ApplyViewSetting(view_option) = action {
                    match view_option {
                        ViewOption::Single => {
                            window.activate_action_for_control(&Control::ToggleSingleView)
                        }
                        ViewOption::Grid2x2 => {
                            window.activate_action_for_control(&Control::ToggleTwoByTwoView)
                        }
                        ViewOption::Grid3x3 => {
                            window.activate_action_for_control(&Control::TogglePicturesPerRow(3))
                        }
                        ViewOption::Grid4x4 => {
                            window.activate_action_for_control(&Control::TogglePicturesPerRow(4))
                        }
                        ViewOption::Grid5x5 => {
                            window.activate_action_for_control(&Control::TogglePicturesPerRow(5))
                        }
                        ViewOption::Thumbnails => {
                            window.activate_action_for_control(&Control::ToggleThumbView)
                        }
                        ViewOption::Covers => {
                            window.activate_action_for_control(&Control::ToggleCoverSelection)
                        }
                        ViewOption::Palette => {
                            window.activate_action_for_control(&Control::TogglePalette)
                        }
                        ViewOption::FilePath | ViewOption::FileDate | ViewOption::FileSize => {
                            window.toggle_view_display_option(view_option)
                        }
                        ViewOption::FullSize => window.toggle_expand(),
                        ViewOption::Catalog => window.action_view_catalog(),
                    }
                }
                if action.is_repeatable() {
                    let mut last_action = this.last_action.borrow_mut();
                    *last_action = action
                }
            }
        )
    }

    fn back_to_previous_location(&self) {
        let location = self.with_view_state_mut(|view_state| {
            view_state.set_old_location();
            view_state.current_location.clone()
        });
        let _ = self.retrieve_from_repository(
            Some(location.covers_only()),
            location.sub_directory(),
            location.predicate(),
        );
        self.with_view_state_mut(|view_state| {
            view_state.settings.set_covers_only(location.covers_only());
            if view_state.navigator.can_move(&Direction::Index {
                value: location.position(),
            }) {
                view_state.navigator.move_towards(&Direction::Index {
                    value: location.position(),
                })
            } else {
                view_state.navigator.move_towards(&Direction::First)
            }
        });
    }
    fn apply_order_setting_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gio_action = GioAction::from((object, variant));
                if let Action::ApplyOrderSetting(order) = Action::from(gio_action) {
                    this.with_view_state_mut(|view_state| {
                        let gallery = &mut view_state.gallery;
                        gallery.sort_by(order);
                        let new_position = gallery.current_picture_index();
                        let direction = Direction::Index {
                            value: new_position,
                        };
                        if view_state.navigator.can_move(&direction) {
                            view_state.navigator.move_towards(&direction);
                        } else {
                            println!("navigator can't move to: {:?}", &direction);
                        };
                        dbg!(view_state.navigator.position());
                        dbg!(view_state.gallery.current_picture_index());
                        view_state.navigator.set_page_changed();
                    });

                    window.dismiss();
                    window.refresh_view();
                }
            }
        )
    }

    fn cancel_selection_range_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                this.with_view_state_mut(|view_state| {
                    view_state.selection.cancel();
                });

                window.refresh_view();
            }
        )
    }

    fn categorize_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gio_action = GioAction::from((object, variant));

                let action = Action::from(gio_action);
                if let Action::Categorize(category) = action {
                    let indices = window.selected_indices();
                    for position in indices {
                        this.with_view_state_mut(|view_state| {
                            let mut picture = view_state.gallery.picture(position);
                            picture.set_category(category.clone());
                            this.with_repository(|repository| {
                                match repository.update_picture(&picture) {
                                    Ok(_) => {}
                                    Err(e) => eprintln!("{}", e),
                                }
                            });
                            view_state.gallery.set_picture(position, picture);
                        });
                    }
                    window.dismiss();
                    window.deselect_pictures();
                }
            }
        )
    }
    fn goto_directory_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                let (current_picture, covers_only) = this.with_view_state(|view_state| {
                    (
                        view_state.gallery.current_picture(),
                        view_state.settings.covers_only(),
                    )
                });
                let directory_opt = if current_picture.cover().is_some() {
                    parent_directory(&current_picture.file_path())
                } else if current_picture.is_folder() {
                    Some(current_picture.file_path())
                } else {
                    None
                };
                if !covers_only && !current_picture.is_folder() {
                    window.present_information(
                        "can only go to a directory when in covers view or from a folder",
                    );
                    return;
                };
                if directory_opt.clone().is_some() {
                    this.with_view_state_mut(|view_state| {
                        view_state.set_current_location_position(
                            view_state.gallery.current_picture_index(),
                        );
                        view_state
                            .set_current_location_covers_only(view_state.settings.covers_only());
                        view_state.set_new_location(directory_opt, None, 0, false)
                    });
                    let location =
                        this.with_view_state(|view_state| view_state.current_location.clone());
                    match this.retrieve_from_repository(
                        Some(location.covers_only()),
                        location.sub_directory(),
                        location.predicate(),
                    ) {
                        Err(e) => panic!("{}", e),
                        Ok(0) => this.back_to_previous_location(),
                        Ok(_) => {}
                    };
                    window.refresh_view();
                }
            }
        )
    }

    fn pick_change_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                let gsr_entry_window = GsrEntryWindow::new_with(
                    &window,
                    &window.gsr_application().shared_controller(),
                    change_menu(),
                    None,
                );
                window.begin_entry(gsr_entry_window);
            }
        )
    }

    fn quit_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                this.with_view_state(|view_state| {
                    if let Ok(mut configuration) = Configuration::from_env() {
                        configuration.current_picture =
                            Some(view_state.gallery.current_picture().file_path());
                        configuration.current_pictures_per_row =
                            Some(view_state.settings.pictures_per_row() as usize);
                        configuration.current_order = Some(view_state.gallery.order());
                        let _ = configuration.save();
                    }
                });

                window.gsr_picture_grid().leave_current_picture_focus();
                window.quit();
            }
        )
    }

    fn quit_directory_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                this.back_to_previous_location();

                window.refresh_view();
            }
        )
    }

    fn toggle_covers_view_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                let (gallery_has_covers, sub_folder) = this.with_view_state(|view_state| {
                    (
                        view_state.gallery.has_covers(),
                        view_state.gallery.sub_folder(),
                    )
                });
                if gallery_has_covers && sub_folder.is_none() {
                    let covers_only: bool = this
                        .with_view_state_mut(|view_state| view_state.settings.toggle_covers_only());
                    match this.retrieve_from_repository(Some(covers_only), None, None) {
                        Err(e) => eprintln!("{}", e),
                        Ok(0) => window.present_information("no picture matching these criteria"),
                        Ok(_) => window.refresh_view(),
                    };
                }
            }
        )
    }
    fn toggle_palette_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                this.with_view_state_mut(|view_state| {
                    view_state.settings.toggle_palette();
                });

                window.refresh_view();
            }
        )
    }

    fn pick_order_setting_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                let gsr_entry_window = GsrEntryWindow::new_with(
                    &window,
                    &this.gsr_application().shared_controller(),
                    order_menu(),
                    None,
                );
                window.begin_entry(gsr_entry_window);
            }
        )
    }
    fn pick_view_option_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                let gsr_entry_window = GsrEntryWindow::new_with(
                    &window,
                    &this.gsr_application().shared_controller(),
                    view_menu(),
                    None,
                );
                window.begin_entry(gsr_entry_window);
            }
        )
    }
    fn rank_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gio_action = GioAction::from((object, variant));
                if let Action::Rank(rank) = Action::from(gio_action) {
                    this.with_repository(|repository| {
                        this.with_view_state_mut(|view_state| {
                            let indices = view_state.selected_indices();
                            for position in indices {
                                let mut picture = view_state.gallery.picture(position);
                                picture.set_rank(rank);
                                match repository.update_picture(&picture) {
                                    Ok(_) => {}
                                    Err(e) => eprintln!("{}", e),
                                }
                                view_state.gallery.set_picture(position, picture);
                            }
                        });
                    });

                    window.dismiss();
                    window.deselect_pictures();
                }
            }
        )
    }

    fn repeat_last_action_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                dbg!();

                let action = this.last_action.borrow().clone();
                window.activate_action(action);
            }
        )
    }

    fn repeat_range_selection_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                this.with_view_state_mut(|view_state| {
                    view_state.selection.repeat();
                    view_state.navigator.set_page_changed();
                });

                window.refresh_view();
            }
        )
    }

    fn toggle_blinking_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                let on = this.with_view_state_mut(|view_state| {
                    view_state.settings.toggle_blinking();
                    view_state.settings.blinking_on()
                });

                if on == true {
                    window.gsr_picture_grid().initialize_pictures();
                    window.gsr_picture_grid().leave_current_picture_focus();
                    window.gsr_picture_grid().enter_current_picture_focus();
                }
                window.refresh_view();
            }
        )
    }

    fn toggle_expand_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_, _, _| {
                let pictures_per_row = this.with_view_state_mut(|view_state| {
                    if view_state.settings.pictures_per_row() == 1 {
                        view_state.settings.toggle_view_mode();
                    }
                    view_state.settings.pictures_per_row()
                });
                if pictures_per_row == 1 {
                    window.frame().set_current_picture();
                    window.refresh_title();
                }
            }
        )
    }

    fn toggle_pictures_per_row_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup, object: &SimpleAction, variant: Option<&Variant>| {
                let gio_action = GioAction::from((object, variant));
                if let Action::TogglePicturesPerRow(pictures_per_row) = Action::from(gio_action) {
                    this.with_view_state_mut(|view_state| {
                        let new_pictures_per_row = view_state
                            .settings
                            .toggle_pictures_per_row(pictures_per_row);
                        view_state
                            .navigator
                            .set_pictures_per_row(new_pictures_per_row as usize);
                        view_state.navigator.update_page_limits();
                        if let Some((row, col)) = view_state
                            .navigator
                            .coords_from_position(view_state.navigator.position())
                        {
                            view_state.focus_at_coords = (col as i32, row as i32);
                        }
                    });

                    window.refresh_view()
                }
            }
        )
    }
    fn toggle_selected_action(
        &self,
        window: GsrApplicationWindow,
    ) -> impl Fn(&SimpleActionGroup, &SimpleAction, Option<&Variant>) + 'static {
        clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            window,
            move |_group: &SimpleActionGroup,
                  object: &SimpleAction,
                  variant: Option<&gtk::glib::Variant>| {
                let gio_action = GioAction::from((object, variant));
                if let Action::ToggleSelected(position) = Action::from(gio_action) {
                    this.with_view_state_mut(|view_state| {
                        if view_state.selection.contains(position) {
                            view_state.selection.unselect(position)
                        } else {
                            view_state.selection.select(position)
                        }
                        view_state.navigator.set_page_changed()
                    });

                    window.refresh_view()
                }
            }
        )
    }
}
