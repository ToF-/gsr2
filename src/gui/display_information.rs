use crate::gui::validator::Validator;
use crate::gui::controller::entry_controller::EntryController;
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;
use crate::gui::view::entry_view::EntryView;
use std::cell::RefCell;

pub fn display_information(application_window: &gtk::ApplicationWindow, message: &str) {
    let entry_view = EntryView::new_with(application_window, &entry_prompt(EntryKind::Information), message);
    let entry_view_rc = RefCell::new(entry_view);
    let entry_controller = EntryController::new_with(entry_view_rc.clone(), Validator::new(EntryKind::Information));
    let entry_controller_rc = RefCell::new(entry_controller);

    entry_view_rc.borrow().attach_key_pressed_controller(&entry_controller_rc);

    entry_controller_rc.borrow().connect_key_pressed(|controller,_| { controller.close() });
    entry_controller_rc.borrow().connect_closed(|controller| {
        if let Some(entry_view) = controller.view() {
            entry_view.close()
        }
    });
    entry_view_rc.borrow().present();
}
