use crate::model::category::Category;
use crate::model::category::category_from_string;
use crate::model::view_option::ViewOption;
use crate::model::order::Order;
use gtk::glib::prelude::ToVariant;
use crate::gui::action::GioActionTy;
use crate::gui::action::Action;
use crate::gui::action::gio_action_parameter::GioActionParameter;
#[derive(Debug)]
pub struct GioAction {
    name: String,
    action_entry_name: String,
    parameter: Option<GioActionParameter>,
}

impl From<Action> for GioAction {
    fn from(action: Action) -> Self {
        let gio_action_ty = GioActionTy::from(action.clone());
        let gio_action_parameter = match action.clone() {
            Action::AddCategory(category_name,target_category_name) =>
                Some(GioActionParameter::from((category_name, target_category_name))),
            Action::AddTag(tag) =>
                Some(GioActionParameter::from(tag)),
            Action::ApplyOrderSetting(order) =>
                Some(GioActionParameter::from(order)),
            Action::ApplyViewSetting(view_option) =>
                Some(GioActionParameter::from(view_option)),
            Action::CancelSelectionRange =>
                None,
            Action::Categorize(category_opt) =>
                Some(GioActionParameter::from(category_opt)),
            _ => todo!()
        };
        Self {
            name: gio_action_ty.name(),
            action_entry_name: gio_action_ty.action_entry_name(),
            parameter: gio_action_parameter,
        }
    }
}

impl From<GioAction> for Action {
    fn from(gio_action: GioAction) -> Self {
        match &gio_action.name() as &str {
            "add-category" => {
                let string_pair: (String, String) = <(String, String)>::from(gio_action.parameter().unwrap());
                Action::AddCategory(string_pair.0, string_pair.1)
            },
            "add-tag" => Action::AddTag(String::from(gio_action.parameter().unwrap())),
            "apply-order-setting" => Action::ApplyOrderSetting(Order::from(gio_action.parameter().unwrap())),
            "apply-view-setting" => Action::ApplyViewSetting(ViewOption::from(gio_action.parameter().unwrap())),
            "cancel-selection-range" => Action::CancelSelectionRange,
            "categorize" => Action::Categorize(Category::from(gio_action.parameter().unwrap())),

            _ => todo!()
        }
    }
}

impl GioAction {

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn action_entry_name(&self) -> String {
        self.action_entry_name.clone()
    }

    pub fn parameter(&self) -> Option<GioActionParameter> {
        self.parameter.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_action_to_and_from(action: Action) {
        let source = action.clone();
        let gio_action = GioAction::from(source.clone());
        let target = Action::from(gio_action);
        assert_eq!(target, source);
    }

    #[test]
    fn gio_action_from_action_and_vice_versa() {
        check_action_to_and_from(Action::AddCategory("foo".to_string(), "bar".to_string()));
        check_action_to_and_from(Action::AddTag("foo".to_string()));
        check_action_to_and_from(Action::ApplyOrderSetting(Order::Name));
        check_action_to_and_from(Action::ApplyViewSetting(ViewOption::Thumbnails));
        check_action_to_and_from(Action::CancelSelectionRange);
        check_action_to_and_from(Action::Categorize(category_from_string("foo")));
    }

}

