use crate::gui::action::Action;
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;
use crate::gui::view::entry_view::EntryView;
use std::cell::RefCell;

pub fn display_information(
    application_window: &gtk::ApplicationWindow,
    message: &str,
) -> EntryView {
    let entry_view = EntryView::new_with(
        application_window,
        &entry_prompt(EntryKind::Information),
        message,
        Action::Nothing,
    );
    entry_view.present();
    entry_view
}
