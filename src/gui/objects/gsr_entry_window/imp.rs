use crate::env::default_values::ENTRY_WINDOW_HEIGHT;
use crate::env::default_values::ENTRY_WINDOW_WIDTH;
use gtk::Align;
use gtk::CssProvider;
use gtk::Orientation;
use gtk::glib;
use gtk::glib::subclass::prelude::*;
use gtk::prelude::GtkWindowExt;
#[allow(deprecated)]
use gtk::prelude::StyleContextExt;
use gtk::prelude::WidgetExt;
use std::cell::RefCell;

use crate::gui::editor::Editor;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct GsrEntryWindow {
    pub prompt_rc: RefCell<String>,
    pub input_rc: RefCell<String>,
    pub editor_opt_rc: RefCell<Option<Editor>>,
}

impl GsrEntryWindow {
    pub fn initialize(
        &self,
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        input: &str,
        editor_opt: Option<Editor>,
    ) {
        *self.prompt_rc.borrow_mut() = prompt.to_owned();
        *self.input_rc.borrow_mut() = input.to_owned();
        *self.editor_opt_rc.borrow_mut() = editor_opt;
        let obj = self.obj();
        obj.set_decorated(false);
        obj.set_modal(true);
        obj.set_default_width(ENTRY_WINDOW_WIDTH);
        obj.set_default_height(ENTRY_WINDOW_HEIGHT);
        obj.set_transient_for(Some(application_window));
        let entry_text = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(input)
            .build();
        entry_text.add_css_class("entry");
        let prompt_label = gtk::Label::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .label(prompt)
            .build();
        let prompt_css_provider = CssProvider::new();
        prompt_css_provider.load_from_string(
            "
            label {
                padding: 10px;
                font-size: 16px;
            }
        ",
        );
        #[allow(deprecated)]
        prompt_label.style_context().add_provider(
            &prompt_css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let entry_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .halign(Align::Fill)
            .valign(Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .homogeneous(false)
            .build();
        entry_box.append(&prompt_label);
        entry_box.append(&entry_text);
        obj.set_child(Some(&entry_box));
    }
}


#[glib::object_subclass]
impl ObjectSubclass for GsrEntryWindow {
    const NAME: &'static str = "GsrEntryWindow";
    type Type = super::GsrEntryWindow;
    type ParentType = gtk::Window;
}

impl ObjectImpl for GsrEntryWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.set_title(Some("My Custom Window"));
        obj.set_default_size(800, 600);

        let label = gtk::Label::new(Some("Hello from subclass"));

        obj.set_child(Some(&label));
    }
}

impl WidgetImpl for GsrEntryWindow {}

impl WindowImpl for GsrEntryWindow {}
