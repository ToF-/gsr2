use crate::env::default_values::TREELIST_WINDOW_HEIGHT;
use crate::env::default_values::TREELIST_WINDOW_WIDTH;
use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_application_window::GsrApplicationWindow;
use crate::model::catalog::Catalog;
use crate::model::sub_category::SubCategory;
use glib::BoxedAnyObject;
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
use gtk::{self};
use gtk::{
    Label, ListItem, ListView, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    glib,
};

pub struct GsrTreelistWindow {}

impl Default for GsrTreelistWindow {
    fn default() -> Self {
        Self {}
    }
}

const WRAP_IN_TREELISTROWS: bool = false;
const AUTOEXPAND: bool = true;

impl GsrTreelistWindow {
    pub fn initialize(
        &self,
        gsr_application_window: &GsrApplicationWindow,
        main_controller_rc: &RcMainController,
        catalog: &Catalog,
        prompt: &str,
        initial_item_opt: Option<&str>,
    ) {
        let window = self.obj();
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
        let list_view = Self::build_list_view(catalog.root());

        scrolled_window.set_child(Some(&list_view));
        selector_box.append(&prompt_label);
        selector_box.append(&scrolled_window);
        selector_box.add_css_class("tree-list");
        window.set_decorated(false);
        window.set_modal(true);
        window.set_default_width(TREELIST_WINDOW_WIDTH);
        window.set_default_height(TREELIST_WINDOW_HEIGHT);
        window.set_transient_for(Some(gsr_application_window));
        window.style_context().add_provider(
            &window_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        window.set_child(Some(&selector_box));
        Self::attach_key_pressed_event_handler(&scrolled_window, window.as_ref());
        gtk::style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &window_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
    fn build_list_view(root: SubCategory) -> gtk::ListView {
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
        let selection = SingleSelection::new(Some(tree_list_model));
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong]
            selection,
            move |_, _key, _key_code, _modifier_type| {
                let _selected: String = if let Some(row_object) = selection.selected_item() {
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
                Propagation::Proceed
            }
        ));
        let view = ListView::new(Some(selection), Some(signal_list_item_factory));
        view.add_controller(event_controller_key);
        view.add_css_class("catalog");

        view
    }
    fn attach_key_pressed_event_handler(
        window: &gtk::ScrolledWindow,
        gsr_treelist_window: &super::GsrTreelistWindow,
    ) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong]
            gsr_treelist_window,
            move |_, key, _key_code, _modifier_type| {
                let key_name = key.name().unwrap_or_default();
                let key_name = key_name.as_str();
                match key_name {
                    "Escape" => gsr_treelist_window.close(),
                    _ => {}
                };
                Propagation::Stop
            }
        ));
        window.add_controller(event_controller_key);
    }
}
#[gtk::glib::object_subclass]
impl ObjectSubclass for GsrTreelistWindow {
    const NAME: &'static str = "GsrTreelistWindow";
    type Type = super::GsrTreelistWindow;
    type ParentType = gtk::Window;
}

impl ObjectImpl for GsrTreelistWindow {}

impl WidgetImpl for GsrTreelistWindow {}

impl WindowImpl for GsrTreelistWindow {}
