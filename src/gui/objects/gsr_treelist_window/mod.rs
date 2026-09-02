use crate::env::default_values::TREELIST_WINDOW_HEIGHT;
use crate::env::default_values::TREELIST_WINDOW_WIDTH;
use crate::gui::action::Action;
use crate::gui::action::gio_action::GioAction;
use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::model::catalog::Catalog;
use crate::model::category::category_from_string;
use crate::model::sub_category::SubCategory;
use glib::BoxedAnyObject;
use glib::Variant;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::gdk::Display;
use gtk::glib::{Propagation, clone};
use gtk::prelude::BoxExt;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{self};
use gtk::{
    Label, ListItem, ListView, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    glib,
};

mod imp;

glib::wrapper! {
    pub struct GsrTreelistWindow(ObjectSubclass<imp::GsrTreelistWindow>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager;
}

const WRAP_IN_TREELISTROWS: bool = false;
const AUTOEXPAND: bool = true;

impl GsrTreelistWindow {
    pub fn new() -> Self {
        gtk::glib::Object::new()
    }

    pub fn new_with(
        application_window: &GsrApplicationWindow,
        main_controller_rc: &RcMainController,
        catalog: &Catalog,
        prompt: &str,
        initial_item_opt: Option<&str>,
        action_on_confirm: Action,
    ) -> Self {
        let obj = Self::new();
        obj.initialize(
            application_window,
            main_controller_rc,
            catalog,
            prompt,
            initial_item_opt,
            action_on_confirm,
        );
        obj
    }
    pub fn initialize(
        &self,
        gsr_application_window: &GsrApplicationWindow,
        main_controller_rc: &RcMainController,
        catalog: &Catalog,
        prompt: &str,
        initial_item_opt: Option<&str>,
        action_on_confirm: Action,
    ) {
        let prompt_label = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(prompt)
            .build();
        let prompt_css_provider = CssProvider::new();
        prompt_css_provider.load_from_string(
            "
            label {
                padding: 1px;
                font-size: 16px;
            }
        ",
        );
        prompt_label.style_context().add_provider(
            &prompt_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let scrolled_window = gtk::ScrolledWindow::builder()
            .min_content_width(400)
            .min_content_height(500)
            .build();
        let selector_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .homogeneous(false)
            .build();
        let window_css_provider = CssProvider::new();
        scrolled_window.add_css_class("tree-list");
        window_css_provider.load_from_string("window.tree-list { background-color:black;}");
        let list_view = self.build_list_view(catalog.root_category(), initial_item_opt);

        scrolled_window.set_child(Some(&list_view));
        selector_box.append(&prompt_label);
        selector_box.append(&scrolled_window);
        selector_box.add_css_class("tree-list");
        self.set_decorated(false);
        self.set_modal(true);
        self.set_default_width(TREELIST_WINDOW_WIDTH);
        self.set_default_height(TREELIST_WINDOW_HEIGHT);
        self.set_transient_for(Some(gsr_application_window));
        self.style_context().add_provider(
            &window_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        self.set_child(Some(&selector_box));
        self.attach_key_pressed_event_handler(&scrolled_window, action_on_confirm);
        let main_controller = main_controller_rc.borrow();
        self.insert_action_group("main-controller", Some(&main_controller.gio_action_group()));
        gtk::style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &window_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn list_view(&self) -> gtk::ListView {
        self.first_child()
            .expect("gsr_treelist_window has no child")
            .downcast::<gtk::Box>()
            .expect("gsr_treelist_window child is not a box")
            .first_child()
            .expect("selector_box has no child")
            .next_sibling()
            .expect("prompt label has no sibling")
            .downcast::<gtk::ScrolledWindow>()
            .expect("prompt sibling is not a scrolled window")
            .first_child()
            .expect("scrolled window has no child")
            .downcast::<gtk::ListView>()
            .expect("scrolled window child is not a list view")
    }

    pub fn position(&self) -> u32 {
        self.imp().position.get()
    }
    fn build_list_view(&self, root: SubCategory, initial_item_opt: Option<&str>) -> gtk::ListView {
        let store = gio::ListStore::new::<BoxedAnyObject>();
        store.append(&BoxedAnyObject::new(root));
        let tree_list_model: TreeListModel =
            TreeListModel::new(store, WRAP_IN_TREELISTROWS, AUTOEXPAND, |obj| {
                let boxed = obj.downcast_ref::<glib::BoxedAnyObject>().unwrap();
                let root = boxed.borrow::<SubCategory>();
                if root.sub_categories().is_empty() {
                    return None;
                }
                let sub_categories = gio::ListStore::new::<BoxedAnyObject>();
                for child in &root.sub_categories() {
                    sub_categories.append(&BoxedAnyObject::new(child.clone()));
                }
                Some(sub_categories.upcast())
            });

        let signal_list_item_factory = SignalListItemFactory::new();
        signal_list_item_factory.connect_setup(|_, item| {
            let expander = TreeExpander::new();
            let label = Label::new(None);
            expander.set_child(Some(&label));
            item.downcast_ref::<gtk::ListItem>()
                .unwrap()
                .set_child(Some(&expander));
        });

        signal_list_item_factory.connect_bind(|_, item| {
            let item = item.downcast_ref::<ListItem>().unwrap();
            let row = item.item().unwrap().downcast::<gtk::TreeListRow>().unwrap();
            let expander = item
                .child()
                .unwrap()
                .downcast::<gtk::TreeExpander>()
                .unwrap();
            expander.set_list_row(Some(&row));
            let label = expander.child().unwrap().downcast::<Label>().unwrap();
            let boxed = row
                .item()
                .unwrap()
                .downcast::<glib::BoxedAnyObject>()
                .unwrap();
            let node = boxed.borrow::<SubCategory>();
            label.set_text(&node.name());
        });

        let selection = SingleSelection::new(Some(tree_list_model.clone()));
        if let Some(initial_item) = initial_item_opt {
            for position in 0..tree_list_model.n_items() {
                let Some(obj) = tree_list_model.item(position) else {
                    continue;
                };
                let row = obj.downcast::<gtk::TreeListRow>().unwrap();
                let Some(item) = row.item() else {
                    continue;
                };
                let boxed = item.downcast::<glib::BoxedAnyObject>().unwrap();
                let node = boxed.borrow::<SubCategory>();
                if node.name() == initial_item {
                    selection.set_selected(position);
                    *self.imp().selected.borrow_mut() = node.name();
                    self.imp().position.set(position);
                    break;
                }
            }
        };

        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong (rename_to=this)]
            self,
            #[strong]
            selection,
            move |_, _key, _key_code, _modifier_type| {
                let selected: String = if let Some(row_object) = selection.selected_item() {
                    let row = row_object.downcast::<gtk::TreeListRow>().unwrap();
                    if row.item().is_some() {
                        let boxed = row
                            .item()
                            .unwrap()
                            .downcast::<glib::BoxedAnyObject>()
                            .unwrap();
                        let sub_category = boxed.borrow::<SubCategory>();
                        sub_category.name()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };
                *this.imp().selected.borrow_mut() = selected;

                Propagation::Proceed
            }
        ));
        let initial_position = self.imp().position.get();
        dbg!(initial_position);
        let view = ListView::new(Some(selection), Some(signal_list_item_factory));
        view.add_controller(event_controller_key);
        view.add_css_class("catalog");
        view.scroll_to(initial_position, gtk::ListScrollFlags::FOCUS, None);
        view
    }

    fn selected(&self) -> String {
        self.imp().selected.borrow().clone()
    }

    fn activate_confirm_action(&self, action_on_confirm: Action) {
        let action = match action_on_confirm {
            Action::Categorize(_) => Action::Categorize(category_from_string(&self.selected())),
            Action::AddCategory(source, _) => Action::AddCategory(source, self.selected()),
            other => other,
        };
        let action_call = GioAction::from(action).to_simple_action_call();
        let name = action_call.0.clone();
        let variant = action_call.1.clone();
        let variant_ref: Option<&Variant> = match &variant {
            None => None,
            Some(v) => Some(v.as_ref()),
        };
        match self.activate_action(&name, variant_ref) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "connect_key_pressed_controller for gsr_entry_window {} {:?} : {}",
                    name, variant_ref, e
                )
            }
        }
    }

    fn attach_key_pressed_event_handler(
        &self,
        window: &gtk::ScrolledWindow,
        action_on_confirm: Action,
    ) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong (rename_to = this)]
            self,
            move |_, key, _key_code, _modifier_type| {
                let key_name = key.name().unwrap_or_default();
                let key_name = key_name.as_str();
                println!("{:?}", &key_name);
                match key_name {
                    // TEMPORARY, activate action instead
                    "Escape" => this.activate_confirm_action(Action::Cancel),
                    "Return" => this.activate_confirm_action(action_on_confirm.clone()),
                    _ => {}
                };
                Propagation::Stop
            }
        ));
        window.add_controller(event_controller_key);
    }
}
