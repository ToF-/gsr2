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

#[derive(Clone, Debug)]
pub struct TreeListWindow {
    window: gtk::Window,
}

#[allow(deprecated)]
impl TreeListWindow {
    pub fn new(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        selected: &str,
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
        selector_box.append(&prompt_label);
        selector_box.append(&scrolled_window);
        let window = gtk::Window::builder()
            .decorated(false)
            .modal(true)
            .default_width(TREELIST_WINDOW_WIDTH)
            .default_height(TREELIST_WINDOW_HEIGHT)
            .transient_for(application_window)
            .build();
        window.set_child(Some(&selector_box));
        Self::attach_key_pressed_event_handler(&scrolled_window, controller_rc);
        TreeListWindow { window: window }
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
