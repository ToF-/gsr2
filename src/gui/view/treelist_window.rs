use gdk::{Key, ModifierType};
use gtk::prelude::*;
use gtk::{self, gdk};
use gtk::EventControllerKey;
use gtk::gdk::Display;
use gtk::prelude::ListItemExt;
use gtk::glib::object::Cast;
use gtk::gio;
use crate::model::catalog::SubCategory;
use glib::BoxedAnyObject;
use crate::model::catalog::Catalog;
use crate::RcController;
use crate::clone;
use crate::env::default_values::{TREELIST_WINDOW_HEIGHT, TREELIST_WINDOW_WIDTH};
use crate::gui::event::Event;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::glib::Propagation;
use gtk::prelude::BoxExt;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use gtk::{glib, Label, ListItem, ListView, ScrolledWindow, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel};

#[derive(Clone, Debug)]
pub struct TreeListWindow {
    window: gtk::Window,
    selected: String,
}

#[allow(deprecated)]
impl TreeListWindow {
    pub fn new(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        _selected: &str,
        catalog: &Catalog,
        controller_rc: &RcController,
    ) -> Self {
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
        window_css_provider.load_from_string(
        "window.tree-list { background-color:black;}");
        let list_view = build_list_view(catalog.root(), controller_rc);

        scrolled_window.set_child(Some(&list_view));
        selector_box.append(&prompt_label);
        selector_box.append(&scrolled_window);
        selector_box.add_css_class("tree-list");
        let window = gtk::Window::builder()
            .decorated(false)
            .modal(true)
            .default_width(TREELIST_WINDOW_WIDTH)
            .default_height(TREELIST_WINDOW_HEIGHT)
            .transient_for(application_window)
            .build();
        window.style_context().add_provider(&window_css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        window.set_child(Some(&selector_box));
        Self::attach_key_pressed_event_handler(&scrolled_window, controller_rc);
         gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &window_css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
        TreeListWindow { window: window, selected: "".to_string(), }
    }
    pub fn popup(&self) {
        self.window.present()
    }

    pub fn close(&self) {
        self.window.close()
    }

    fn attach_key_pressed_event_handler(
        window: &gtk::ScrolledWindow,
        controller_rc: &RcController,
    ) {
        let event_controller_key = gtk::EventControllerKey::new();
        event_controller_key.connect_key_pressed(clone!(
            #[strong]
            controller_rc,
            move |_, key, key_code, modifier_type| {
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller.process_event(
                        Event::KeyPressed {
                            key,
                            key_code,
                            modifier_type,
                        },
                        &controller_rc,
                    );
                };
                Propagation::Stop
            }
        ));
        window.add_controller(event_controller_key);
    }
}

const WRAP_IN_TREELISTROWS: bool = false;
const DONT_AUTOEXPAND: bool = false;
const AUTOEXPAND: bool = true;

fn build_list_view(root: SubCategory, controller_rc: &RcController) -> gtk::ListView {
    let store = gio::ListStore::new::<BoxedAnyObject>();
    store.append(&BoxedAnyObject::new(root));
    let tree_list_model: TreeListModel = TreeListModel::new(store, WRAP_IN_TREELISTROWS, AUTOEXPAND, |obj| {
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
        let expander = item.child().unwrap().downcast::<gtk::TreeExpander>().unwrap();
        expander.set_list_row(Some(&row));
        let label = expander.child().unwrap().downcast::<Label>().unwrap();
        let boxed = row.item().unwrap().downcast::<glib::BoxedAnyObject>().unwrap();
        let node = boxed.borrow::<SubCategory>();
        label.set_text(&node.name());
    });
    let selection = SingleSelection::new(Some(tree_list_model));
    let event_controller_key = EventControllerKey::new();
    event_controller_key.connect_key_pressed(clone!( #[strong] controller_rc, #[strong] selection, move  |_, key, key_code, modifier_type| {
                println!("selection.selected_item:{:?}", selection.selected_item());
                if let Some(row_object) = selection.selected_item() {
                    let row = row_object
                        .downcast::<gtk::TreeListRow>()
                        .unwrap();
                    println!("row: {:?}", row);
                    if let Some(item) = row.item() {
                        let boxed = row.item().unwrap().downcast::<glib::BoxedAnyObject>().unwrap();
                        let sub_category = boxed.borrow::<SubCategory>();
                        println!("{}",sub_category.name());
                    }
                };
                if let Ok(mut controller) = controller_rc.try_borrow_mut() {
                    controller.process_event(
                        Event::KeyPressed {
                            key,
                            key_code,
                            modifier_type,
                        },
                        &controller_rc,
                    );
                };
                Propagation::Proceed

            }
    ));
    let view = ListView::new(Some(selection), Some(signal_list_item_factory));
    view.add_controller(event_controller_key);
    view.add_css_class("catalog");

    view
}
