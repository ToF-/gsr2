use gtk::prelude::GtkWindowExt;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use crate::gui::action::Action;
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;

pub fn display_information(
    application_window: &gtk::ApplicationWindow,
    message: &str,
) -> GsrEntryWindow {
    let gsr_entry_window = GsrEntryWindow::new_with(
        application_window,
        &entry_prompt(EntryKind::Information),
        message,
        None
    );
    gsr_entry_window.present();
    gsr_entry_window
}
