use crate::gui::action::Action;
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;
use crate::gui::main_controller::MainController;
use crate::gui::main_controller::RcMainController;
use crate::gui::objects::gsr_entry_window::GsrEntryWindow;
use gtk::prelude::GtkWindowExt;

pub fn display_information(
    application_window: &gtk::ApplicationWindow,
    main_controller_rc: &RcMainController,
    message: &str,
) -> GsrEntryWindow {
    let gsr_entry_window = GsrEntryWindow::new_with(
        application_window,
        main_controller_rc,
        &entry_prompt(EntryKind::Information),
        message,
        None,
    );
    gsr_entry_window.present();
    gsr_entry_window
}
