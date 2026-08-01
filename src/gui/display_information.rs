use crate::gui::view::entry_view::EntryView;
use crate::gui::controller::entry_controller::EntryController;
use std::cell::RefCell;

pub fn display_information(application_window: &gtk::ApplicationWindow, message: &str) {
        let entry_view = EntryView::new_with(
            application_window,
            "information",
            message);
        let entry_controller = EntryController::new();
        let EntryController_rc = RefCell::new(entry_controller);

        entry_view.attach_key_pressed_controller(&EntryController_rc);

        if let Ok(entry_controller) = EntryController_rc.try_borrow() {
            entry_controller.connect_key_pressed(|controller, _| {
                // whatever the key is, we close
                controller.close()
            });
            let the_entry_view = entry_view.clone();
            entry_controller.connect_closed(move |controller| {
                the_entry_view.close()
            });
        };
        entry_view.present()
    }
