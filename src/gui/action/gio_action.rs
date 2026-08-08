use gtk::glib::prelude::ToVariant;
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
        let gio_action_ty = GioActionTy::from(action.clone());
        let gio_action_parameter = match action.clone() {
            Action::AddCategory(category_name,target_category_name) =>
                GioActionParameter::from((category_name, target_category_name)),
            Action::AddTag(tag) =>
                GioActionParameter::from(tag),
            _ => todo!()
        };
        Self {
            action_entry_name: gio_action_ty.action_entry_name(),
            parameter: gio_action_parameter,
        }
    }
}

impl GioAction {
    pub fn action_entry_name(&self) -> String {
        self.action_entry_name.clone()
    }

    pub fn parameter(&self) -> gtk::glib::Variant {
        self.parameter.variant()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gio_action_from_action_add_category() {
        let action = Action::AddCategory("foo".to_string(), "bar".to_string());
        let gio_action = GioAction::from(action);
        assert_eq!("main-controller.add-category", gio_action.action_entry_name());
        let parameter_as_string_pair: (String, String) = gio_action.parameter()
            .get().unwrap();
        assert_eq!("foo".to_string(), parameter_as_string_pair.0);
        assert_eq!("bar".to_string(), parameter_as_string_pair.1);

    }
    #[test]
    fn gio_action_from_action_add_tag() {
        let action = Action::AddTag("foo".to_string());
        let gio_action = GioAction::from(action);
        assert_eq!("main-controller.add-tag", gio_action.action_entry_name());
        let parameter_as_string: String = gio_action.parameter()
            .get().unwrap();
        assert_eq!("foo".to_string(), parameter_as_string);
    }

}
