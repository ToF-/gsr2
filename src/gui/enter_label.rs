use crate::gui::controller::Controller;
use gtk::glib::clone;
use crate::env::default_values::BLINKING;
use crate::env::default_values::BLINKING_DURATION;
use crate::model::label::Label;
use crate::model::repository::Repository;
use crate::gui::completion_dispenser::CompletionDispenser;
use crate::gui::editor::entry_editor::EntryEditor;
use crate::gui::entry_kind::EntryKind;
use crate::gui::entry_prompt::entry_prompt;
use crate::gui::validator::Validator;
use crate::gui::view::entry_view::EntryView;
use std::cell::RefCell;

pub fn enter_label(application_window: &gtk::ApplicationWindow, repository: &Repository) {
    let entry_view = EntryView::new_with(
        application_window,
        &entry_prompt(EntryKind::Label),
        "",
    );
    let entry_view_rc = RefCell::new(entry_view);
    let entry_editor = EntryEditor::new_with(
        entry_view_rc.clone(),
        Validator::new(EntryKind::Label),
        CompletionDispenser::new_with(repository.all_labels())
    );
    let entry_editor_rc = RefCell::new(entry_editor);

    entry_view_rc
        .borrow()
        .attach_key_pressed_editor(&entry_editor_rc, BLINKING);

    entry_editor_rc
        .borrow()
        .connect_key_pressed(|editor, key_name| {
            editor.edit_entry(key_name);
        });

    entry_editor_rc.borrow().connect_closed( move |editor| {
        if !editor.entry().is_empty() {
             
        }
        if let Some(entry_view) = editor.view() {
            entry_view.close();
        }

    });
    entry_view_rc.borrow().present();
}
