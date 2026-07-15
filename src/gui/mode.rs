use crate::gui::control::Control;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Mode {
    Setting(Control),
    View,
    Editing,
    Selecting,
}
