use crate::cli::command_line_arguments::CommandLineArguments;
use crate::env::configuration::CONFIGURATION;
use crate::env::default_values::APPLICATION_ID;
use crate::gui::controller::Controller;
use crate::gui::main_controller::MainController;
use crate::gui::view::View;
use crate::gui::view_state::navigator::Navigator;
use crate::model::gallery::Gallery;
use crate::model::shared::Shared;
use gtk::gdk::Display;
mod imp;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct GsrApplication(ObjectSubclass<imp::GsrApplication>)
        @extends gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for GsrApplication {
    fn default() -> Self {
        let obj: Self = glib::Object::builder()
            .property("application-id", APPLICATION_ID)
            .build();
        obj.connect_startup(|_| style_context_add_provider_for_display());
        obj
    }
}

impl GsrApplication {
    pub fn set_state(&self, clargs: CommandLineArguments, gallery: &Gallery) {
        self.imp().set_state(clargs, gallery)
    }
    pub fn shared_view(&self) -> Shared<View> {
        (*self.imp().view.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_navigator(&self) -> Shared<Navigator> {
        (*self.imp().navigator.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_gallery(&self) -> Shared<Gallery> {
        (*self.imp().gallery.borrow()).as_ref().unwrap().clone()
    }

    pub fn shared_command_line_arguments(&self) -> Shared<CommandLineArguments> {
        (*self.imp().command_line_arguments.borrow())
            .as_ref()
            .unwrap()
            .clone()
    }
}

pub fn style_context_add_provider_for_display() {
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_string(
        "window { background-color:black;} 
        image { margin:1em ; } 
        label { color:white; 
                font-family:sans-serif;
                font-size:12px;}
        label.pane {
            color: gray;
            font-size: 22px;
            background-color:black;
        }
        label.entry {
            padding: 10px;
            font-size: 32px;
        }
        listview.catalog {
            background-color:black;
            }
        listview.catalog treeexpander expander {
            color: white;
        }
        ",
    );
    gtk::style_context_add_provider_for_display(&Display::default().unwrap(), &css_provider, 1000);
}
