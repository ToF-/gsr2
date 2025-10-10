use crate::gui::control::Control;

#[derive(PartialEq, Clone, Debug)]
pub enum EntryKind {
    Label,
    Number,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Mode {
    Setting(Control),
    View,
    Editing(EntryKind)
}


