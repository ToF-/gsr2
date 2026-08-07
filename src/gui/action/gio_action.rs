use crate::gui::action::GioActionTy;
use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter::GioActionParameter;
#[derive(Debug)]
pub struct GioAction {
    action_entry_name: String,
    parameter: GioActionParameter,
}

impl From<Action> for GioAction {
    fn from(action: Action) -> Self {
        let gio_action_ty = GioActionTy::from(action);
        Self {
            action_entry_name: gio_action_ty.action_entry_name(),
            parameter: GioActionParameter::from("foo"),
        }
    }
}

impl GioAction {
    pub fn action_entry_name(&self) -> String {
        self.action_entry_name.clone()
    }

    pub fn parameter(&self) -> GioActionParameter {
        self.parameter.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gio_action_from_action() {
        let action = Action::AddTag("foo".to_string());
        let gio_action = GioAction::from(action);
        assert_eq!("main-controller.add-tag", gio_action.action_entry_name());
    }

}
