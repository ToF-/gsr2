use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter::GioActionParameter;
#[derive(Debug)]
pub struct GioAction {
    action_entry_name: String,
    parameter: GioActionParameter,
}

impl From<Action> for GioAction {
    fn from(action: Action) -> Self {
        todo!()
    }
}
