use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::editor::entry_editor::EntryEditor;
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;
use crate::gui::validator::Validator;
use crate::gui::view::entry_view::EntryView;
use crate::gui::action::Action;
use std::cell::RefCell;

pub fn display_information(
    application_window: &gtk::ApplicationWindow,
    message: &str,
) {
    let entry_view = EntryView::new_with(
        application_window,
        &entry_prompt(EntryKind::Information),
        message,
        Action::Nothing,
    );
    let entry_view_rc = RefCell::new(entry_view);
    let entry_editor = EntryEditor::new_with(
        entry_view_rc.clone(),
        Validator::new(EntryKind::Information),
        CompletionDispenser::new(),
    );
    let entry_editor_rc = RefCell::new(entry_editor);

    entry_view_rc
        .borrow()
        .attach_key_pressed_editor(&entry_editor_rc, false);

    entry_editor_rc
        .borrow()
        .connect_key_pressed(|editor, _| editor.close());
    entry_editor_rc.borrow().connect_closed(|editor| {
        if let Some(entry_view) = editor.view() {
            entry_view.close()
        }
    });
    entry_view_rc.borrow().present();
}
