use gtk::glib::Propagation;
use crate::RcController;
use crate::clone;
use crate::gui::event::Event;
use gtk::prelude::GtkWindowExt;
use gtk::prelude::WidgetExt;

#[derive(Clone, Debug)]
pub struct TreeListWindow {
    window: gtk::Window,
}

impl TreeListWindow {
    pub fn new(
        application_window: &gtk::ApplicationWindow,
        prompt: &str,
        selected: &str,
        controller_rc: &RcController,
    ) -> Self {
        let scrolled_window = gtk::ScrolledWindow::builder()
            .min_content_width(400)
            .min_content_height(500)
            .build();
        Self::attach_key_pressed_event_handler(&scrolled_window, controller_rc);
        let window = gtk::Window::builder()
            .child(&scrolled_window)
            .build();
        TreeListWindow {
            window: window,
        }
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
                println!("{:?}", key);
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
